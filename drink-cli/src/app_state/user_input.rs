#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Position {
    #[default]
    Fresh,
    History(usize),
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct UserInput {
    history: Vec<String>,
    position: Position,
    current_input: String,
}

impl UserInput {
    pub fn push(&mut self, c: char) {
        self.current_input.push(c);
    }

    pub fn pop(&mut self) {
        self.current_input.pop();
    }

    pub fn set(&mut self, s: String) {
        self.current_input = s;
        self.position = Position::Fresh;
    }

    pub fn prev_input(&mut self) {
        match self.position {
            Position::Fresh if self.history.is_empty() => {}
            Position::Fresh => {
                self.position = Position::History(self.history.len() - 1);
                self.current_input = self.history[self.history.len() - 1].clone();
            }
            Position::History(0) => {}
            Position::History(n) => {
                self.position = Position::History(n - 1);
                self.current_input = self.history[n - 1].clone();
            }
        }
    }

    pub fn next_input(&mut self) {
        match self.position {
            Position::Fresh => {}
            Position::History(n) if n == self.history.len() - 1 => {
                self.position = Position::Fresh;
                self.current_input.clear();
            }
            Position::History(n) => {
                self.position = Position::History(n + 1);
                self.current_input = self.history[n + 1].clone();
            }
        }
    }

    pub fn apply(&mut self) {
        if !self.current_input.is_empty()
            && self.history.last().cloned().unwrap_or_default() != self.current_input
        {
            self.history.push(self.current_input.clone());
        }
        self.current_input.clear();
        self.position = Position::Fresh;
    }

    pub fn current_input(&self) -> &str {
        &self.current_input
    }
}
