#![windows_subsystem = "windows"]

mod file_operations;
mod search_operations;
mod message;
mod mainmenu;
mod editorapp;
mod dispatch;

use std::env;

use crate::editorapp::EditorApp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut app = EditorApp::new();
    if let Some(filename) = args.get(1) {
        app.load(filename.as_str());
    }

    app.run();
}
