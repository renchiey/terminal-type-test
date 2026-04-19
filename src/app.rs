use rand::{self, seq::IndexedRandom};

const DEFAULT_NUM_WORDS: u16 = 500;

pub struct Word {
    pub input: String,
    pub actual: String,
}

impl Word {
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

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Exit,
}

pub struct App {
    pub current_word_index: usize,
    pub words: Vec<Word>,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_word_index: 0,
            words: Vec::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn reset_words(&mut self) {
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

    pub fn handle_space_press(&mut self) {
        let current_word = self
            .words
            .get(self.current_word_index)
            .expect("Out of bounds");

        if current_word.input.len() > 0 {
            self.current_word_index += 1
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        let word_index = self.current_word_index;

        self.words[word_index].enter_char(new_char);
    }

    pub fn delete_char(&mut self) {
        let word_index = self.current_word_index;

        let word = &mut self.words[word_index];

        if word.input.len() == 0 {
            self.current_word_index -= 1;
        }

        word.delete_char();
    }

    pub fn delete_word(&mut self) {
        let word_index = self.current_word_index;

        let word = &mut self.words[word_index];

        let word_length = word.input.len();

        if word_length == 0 {
            if self.current_word_index > 0 {
                self.current_word_index -= 1;
            }
            self.words[self.current_word_index].input = String::new();
        } else {
            self.words[word_index].input = String::new();
        }
    }

    pub fn exit(&mut self) {
        self.current_screen = CurrentScreen::Exit;
    }
}
