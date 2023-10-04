use std::marker::PhantomData;

use crate::mock::builder_utils::{Set, Setter, UnSet};

type Body<Args, Ret> = Box<dyn Fn(Args) -> Ret>;
type Selector = [u8; 4];

pub struct MessageMock<Args, Ret> {
    selector: Selector,
    body: Body<Args, Ret>,
}

pub struct MessageMockBuilder<
    Args,
    Ret,
    SelectorSetter: Setter<Selector>,
    BodySetter: Setter<Body<Args, Ret>>,
> {
    selector: SelectorSetter,
    body: BodySetter,
    _phantom: PhantomData<(Args, Ret)>,
}

impl<Args, Ret> MessageMockBuilder<Args, Ret, UnSet<Selector>, UnSet<Body<Args, Ret>>> {
    pub fn new() -> Self {
        Self {
            selector: UnSet::new(),
            body: UnSet::new(),
            _phantom: PhantomData::default(),
        }
    }
}

impl<Args, Ret, BodySetter: Setter<Body<Args, Ret>>>
    MessageMockBuilder<Args, Ret, UnSet<Selector>, BodySetter>
{
    pub fn with_selector(
        self,
        selector: Selector,
    ) -> MessageMockBuilder<Args, Ret, Set<Selector>, BodySetter> {
        MessageMockBuilder::<Args, Ret, Set<Selector>, BodySetter> {
            selector: Set::new(selector),
            body: self.body,
            _phantom: self._phantom,
        }
    }
}

impl<Args, Ret, SelectorSetter: Setter<Selector>>
    MessageMockBuilder<Args, Ret, SelectorSetter, UnSet<Body<Args, Ret>>>
{
    pub fn with_body(
        self,
        body: Body<Args, Ret>,
    ) -> MessageMockBuilder<Args, Ret, SelectorSetter, Set<Body<Args, Ret>>> {
        MessageMockBuilder::<Args, Ret, SelectorSetter, Set<Body<Args, Ret>>> {
            selector: self.selector,
            body: Set::new(body),
            _phantom: self._phantom,
        }
    }
}

impl<Args, Ret> MessageMockBuilder<Args, Ret, Set<Selector>, Set<Body<Args, Ret>>> {
    pub fn build(self) -> MessageMock<Args, Ret> {
        MessageMock::<Args, Ret> {
            selector: self.selector.retrieve(),
            body: self.body.retrieve(),
        }
    }
}
