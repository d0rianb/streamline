use std::io::{Stdout, Write};

pub struct Popup {
    pub index: i8,
    pub items: Vec<PopupItem>
}

pub struct PopupItem {
    name: String,
}

impl PopupItem {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Popup {
    pub fn new(items: Vec<PopupItem>) -> Self {
        Self {
            index: 0,
            items
        }
    }   

    pub fn get_max_width(&self) -> u16 {
        self.items.iter()
            .max_by(|a, b| a.name.len().cmp(&b.name.len()))
            .map(|item| item.name.len() as u16)
            .unwrap_or(1) + 2
    }

    pub fn up(&mut self) {
        self.index = (self.index + 1) % self.items.len() as i8;
    }

    pub fn down(&mut self) {
        self.index = (self.index - 1) % self.items.len() as i8;
    }

    pub fn render(&self, stdout: &mut Stdout, pos: (u16, u16),) {
        let width = self.get_max_width();
        write!(stdout, "{}╭{}╮", termion::cursor::Goto(pos.0, pos.1), "─".repeat(width as usize)).unwrap();
        for (i, item) in self.items.iter().enumerate() {
            let space_before = (width - item.name.len() as u16) / 2 - 1;
            let space_after = (width - item.name.len() as u16) / 2;
            write!(stdout, "{}│ {}{}{} │",
                termion::cursor::Goto(pos.0, pos.1 + i as u16 + 1), 
                " ".repeat(space_before as usize), 
                item.name, 
                " ".repeat(space_after as usize)
            ).unwrap();
        }
        write!(stdout, "{}╰{}╯", termion::cursor::Goto(pos.0, pos.1 + self.items.len() as u16 + 1), "─".repeat(width as usize)).unwrap();
        write!(stdout, "{}", termion::cursor::Goto(pos.0, pos.1 + self.index as u16 + 1)).unwrap();
    }
}
