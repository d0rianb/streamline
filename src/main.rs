extern crate termion;

use std::io::{stdin, stdout, Stdout, Write};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use crate::popup::{PopupItem, Popup};

mod popup;

struct Stream {
    boxes: Vec<TextBox>,
    length: u16,
    y: u16,
    popup: Option<Popup>,
}

struct TextBox {
    label: String,
    index: u8,
    input: Option<DataType>,
    output: Option<DataType>,
}

#[derive(Debug, PartialEq)]
enum DataType { Audio, Video, Text, JSON }

const MARGIN: u16 = 5; // px

impl Stream {
    pub fn new(t_size: (u16, u16)) -> Self {
        Self {
            boxes: Vec::new(),
            y: t_size.1 / 2,
            length: t_size.0 - 2 * MARGIN,
            popup: None,
        }
    }
    
    pub fn add_box(&mut self, text_box: TextBox) {
        self.boxes.push(text_box);
    }

    pub fn toggle_add_popup(&mut self) {
       let items = vec![
            PopupItem::new("Box".into()),
            PopupItem::new("Branch".into()),
            PopupItem::new("Other".into()),
       ];
       self.popup = Some(Popup::new(items));
    }

    pub fn select_popup_value(&mut self) {
        assert!(self.popup.is_some());
        let popup = self.popup.as_ref().unwrap();
        match popup.items.get(popup.index as usize).unwrap() {
            _ => {}
        }
    }

    pub fn render(&self, stdout: &mut Stdout, c_index: u16) {
        let max_height = self.boxes.iter()
            .max_by_key(|tb| tb.get_height())
            .map(|tb| tb.get_height())
            .unwrap_or(1);
        // let content: Vec<String> = vec!["-".repeat(self.length as usize - 1), ">"];
        let content = String::from("-").repeat(self.length as usize - 1) + ">";

        print!("{}{}", termion::clear::All, termion::cursor::Goto(MARGIN, self.y));
        write!(stdout, "{}{}", content, termion::cursor::Goto(MARGIN + c_index, self.y)).unwrap();
        if let Some(popup) = &self.popup {
           popup.render(stdout, (MARGIN + c_index, self.y)) 
        }
        stdout.lock().flush().unwrap();
    }
}

impl TextBox {
    pub fn new(text: String, index: u8) -> Self {
        Self {
            label: text,
            index,
            input: None,
            output: None
        }
    }

    pub fn get_height(&self) -> u8 {
        const MAX_WIDTH: u8 = 10;
        self.label.len() as u8 / MAX_WIDTH
    }
}

fn main() {
    let stdin = stdin();
    assert!(termion::is_tty(&stdin), "The terminal is not TTY compatible");
    let mut stdout = stdout().into_raw_mode().unwrap();

    let t_size = termion::terminal_size().unwrap();
    let mut stream = Stream::new(t_size);
    let keys = std::io::stdin().keys();
    let mut cursor_index: i16 = 0;
    stream.render(&mut stdout, cursor_index as u16);
    for c in keys {
        let key = c.expect("failed to read key");
        match key {
            Key::Char('q') | Key::Ctrl('c') => break,
            Key::Right | Key::Char('l') => cursor_index += 1,
            Key::Left | Key::Char('h') => cursor_index -= 1,
            Key::Char('a') => stream.toggle_add_popup(),
            Key::Down | Key::Char('\t')  => if let Some(popup) = &mut stream.popup { popup.down() },
            Key::Up | Key::BackTab  => if let Some(popup) = &mut stream.popup { popup.up() },
            Key::Esc => stream.popup = None,
            Key::Char('\n') => if let Some(popup) = &mut stream.popup { stream.select_popup_value() },
            _ => {} 
        }

        let cursor_index: u16 = cursor_index.clamp(0, (t_size.0 - 2 * MARGIN) as i16) as u16;
        stream.render(&mut stdout, cursor_index);
    }
}

