use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::{self, seq::IndexedRandom};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};
use std::io;

const DEFAULT_NUM_WORDS: u16 = 500;

#[derive(Debug, Default)]
pub struct Word {
    input: String,
    actual: String,
}

impl Word {
    pub fn render<'a>(&self) -> Vec<Span<'a>> {
        let mut res: Vec<Span> = Vec::new();
        let mut actual_chars = self.actual.chars();

        for input_char in self.input.chars() {
            match actual_chars.next() {
                Some(a) if a == input_char => res.push(a.to_string().white()),
                Some(a) => res.push(a.to_string().red()),
                None => res.push(input_char.to_string().red()),
            }
        }

        for c in actual_chars {
            res.push(c.to_string().dark_gray());
        }

        res
    }

    pub fn enter_char(&mut self, new_char: char) {
        if !new_char.is_ascii() {
            return;
        }
        self.input.push(new_char);
    }

    pub fn delete_char(&mut self) {
        self.input.pop();
    }
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    current_word_index: usize,
    words: Vec<Word>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            current_word_index: 0,
            words: Vec::new(),
        }
    }

    fn reset_words(&mut self) {
        self.current_word_index = 0;

        let content = include_str!("../words.txt");
        let words: Vec<String> = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect();

        let mut rng = rand::rng();

        self.words = (0..DEFAULT_NUM_WORDS)
            .map(|_| Word {
                input: String::new(),
                actual: words
                    .choose(&mut rng)
                    .expect("Word list not properly initialised.")
                    .to_string(),
            })
            .collect::<Vec<_>>();
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.reset_words();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.modifiers.contains(KeyModifiers::ALT) {
            match key_event.code {
                KeyCode::Backspace => {
                    self.delete_word();
                }
                _ => {}
            }
            return;
        }

        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Char(' ') => self.handle_space_press(),
            KeyCode::Char(char) => self.enter_char(char),
            _ => {}
        }
    }

    fn handle_space_press(&mut self) {
        let current_word = self
            .words
            .get(self.current_word_index)
            .expect("Out of bounds");

        if current_word.input.len() > 0 {
            self.current_word_index += 1
        }
    }

    fn enter_char(&mut self, new_char: char) {
        let word_index = self.current_word_index;

        self.words[word_index].enter_char(new_char);
    }

    fn delete_char(&mut self) {
        let word_index = self.current_word_index;

        let word = &mut self.words[word_index];

        if word.input.len() == 0 {
            self.current_word_index -= 1;
        }

        word.delete_char();
    }

    fn delete_word(&mut self) {
        let word_index = self.current_word_index;

        let word = &mut self.words[word_index];

        let word_length = word.input.len();

        if word_length == 0 {
            self.current_word_index -= 1;
            self.words[self.current_word_index].input = String::new();
        } else {
            self.words[word_index].input = String::new();
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn render_text(&self) -> Text<'_> {
        let mut spans: Vec<Span> = Vec::new();
        let mut check = false;

        for (i, word) in self.words.iter().enumerate() {
            if i > 0 {
                if !check {
                    spans.push("  ".into());
                } else {
                    spans.push(" ".into());
                    check = false;
                }
            }

            if i == self.current_word_index {
                let mut temp = word.render().clone();

                if word.input.len() >= word.actual.len() {
                    temp.push(Span::from(" ").bg(Color::White));
                    check = true;
                } else {
                    temp[word.input.len()] =
                        temp[word.input.len()].clone().black().bg(Color::White);
                }

                spans.extend(temp);
                continue;
            }
            spans.extend(word.render());
        }
        Text::from(Line::from(spans))
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Type Test ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]);
        let _ = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text_box_label = Line::from("Type Here".bold());
        let text_box = Block::bordered()
            .title(text_box_label.left_aligned())
            .border_set(border::THICK);

        let text = self.render_text();

        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .block(text_box)
            .render(area, buf);
    }
}
