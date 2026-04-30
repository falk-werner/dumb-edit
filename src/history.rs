use std::path::Path;

use crate::message::Message;
use crate::config::Config;

use fltk::{app, enums::Shortcut, menu::{MenuButton, MenuFlag}, prelude::MenuExt};

const MAX_ENTRIES: usize = 5;

pub struct History {
    pub button: MenuButton,
}

fn escape_label(label: &str) -> String {
    label.replace("\\", "\\\\")
}

impl History {

    pub fn new(sender: &app::Sender<Message>) -> Self {
        let button = MenuButton::default();
        let mut history = Self { button };

        let config = Config::new();
        history.update_menuentry(&config, sender);

        history
    }

    pub fn update(&mut self, sender: &app::Sender<Message>, filename: &str) {
        let mut config = Config::new();
        let entry = String::from(filename);
        if config.history.contains(&entry) {
            return
        }

        config.history.push_back(entry);
        if config.history.len() > MAX_ENTRIES {
            config.history.pop_front();
        }

        config.save();
        self.update_menuentry(&config, sender);
    }

    fn update_menuentry(&mut self, config: &Config, sender: &app::Sender<Message>) {
        self.button.clear();

        if config.history.is_empty() {
            self.button.add_emit("<empty>", Shortcut::empty(), MenuFlag::empty(), *sender, Message::Empty);
            self.button.at(0).unwrap().deactivate();
        }

        for entry in &config.history {
            let path = Path::new(entry.as_str());
            if ! path.is_file() {
                continue;
            }

            let label = escape_label(entry.as_str());            
            self.button.add_emit(label.as_str(), Shortcut::empty(), MenuFlag::empty(), *sender, Message::Load(entry.clone()));
        }

    }
}
