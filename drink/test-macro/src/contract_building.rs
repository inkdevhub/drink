use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use cargo_metadata::{Metadata, MetadataCommand, Package};
use contract_build::{
    BuildArtifacts, BuildMode, ExecuteArgs, Features, ManifestPath, Network, OptimizationPasses,
    OutputType, Target, UnstableFlags, Verbosity,
};

/// Contract package differentiator.
const INK_AS_DEPENDENCY_FEATURE: &str = "ink-as-dependency";

/// Stores the manifest paths of all contracts that have already been built.
///
/// This prevents from building the same contract for every testcase separately.
static CONTRACTS_BUILT: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();

/// Build the current package with `cargo contract build --release` (if it is a contract package),
/// as well as all its contract dependencies.
///
/// A package is considered as a contract package, if it has the `ink-as-dependency` feature.
///
/// A contract dependency, is a package defined in the `Cargo.toml` file with the
/// `ink-as-dependency` feature enabled.
pub fn build_contracts() {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Error invoking `cargo metadata`");

    for contract_crate in get_contract_crates(&metadata) {
        build_contract_crate(contract_crate);
    }
}

fn get_contract_crates(metadata: &Metadata) -> Vec<&Package> {
    let pkg_lookup = |id| {
        metadata
            .packages
            .iter()
            .find(|package| package.id == id)
            .expect(&format!("Error resolving package {id}"))
    };

    let dep_graph = metadata
        .resolve
        .as_ref()
        .expect("Error resolving dependencies");

    let contract_deps = dep_graph
        .nodes
        .iter()
        .filter_map(|node| {
            node.features
                .contains(&INK_AS_DEPENDENCY_FEATURE.to_string())
                .then(|| node.id.clone())
        })
        .map(pkg_lookup);

    let root = dep_graph
        .root
        .as_ref()
        .expect("Error resolving root package");
    let root = pkg_lookup(root.clone());

    root.features
        .contains_key(INK_AS_DEPENDENCY_FEATURE)
        .then(|| root)
        .into_iter()
        .chain(contract_deps)
        .collect()
}

fn build_contract_crate(pkg: &Package) {
    let manifest_path = get_manifest_path(pkg);

    if !CONTRACTS_BUILT
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .expect("Error locking mutex")
        .insert(manifest_path.clone().into())
    {
        return;
    }

    let args = ExecuteArgs {
        manifest_path,
        verbosity: Verbosity::Default,
        build_mode: BuildMode::Release,
        features: Features::default(),
        network: Network::Online,
        build_artifact: BuildArtifacts::All,
        unstable_flags: UnstableFlags::default(),
        optimization_passes: Some(OptimizationPasses::default()),
        keep_debug_symbols: false,
        lint: false,
        output_type: OutputType::HumanReadable,
        skip_wasm_validation: false,
        target: Target::Wasm,
    };

    contract_build::execute(args).expect("Error building contract");
}

fn get_manifest_path(package: &Package) -> ManifestPath {
    ManifestPath::new(package.manifest_path.clone().into_std_path_buf()).expect(&format!(
        "Error resolving manifest path for package {}",
        package.name
    ))
}
