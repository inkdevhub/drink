use ratatui::text::Line;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Output {
    content: Vec<Line<'static>>,
    offset: u16,
    scrolling: bool,
    window_height: u16,
}

impl Output {
    pub fn content(&self) -> &[Line<'static>] {
        &self.content
    }

    pub fn push(&mut self, line: Line<'static>) {
        self.content.push(line)
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }

    fn max_offset(&self) -> u16 {
        (self.content.len() as u16).saturating_sub(self.window_height)
    }

    pub fn note_display_height(&mut self, height: u16) {
        self.window_height = height;
        if !self.scrolling {
            self.offset = self.max_offset();
        }
    }

    pub fn reset_scrolling(&mut self) {
        self.scrolling = false;
    }

    pub fn scroll_down(&mut self) {
        if self.offset < self.max_offset() {
            self.scrolling = true;
            self.offset += 1
        }
    }

    pub fn scroll_up(&mut self) {
        if self.offset > 0 {
            self.scrolling = true;
            self.offset -= 1;
        }
    }
}
