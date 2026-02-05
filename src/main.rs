//#![windows_subsystem = "windows"]

mod file_operations;
mod search_operations;
mod message;
mod mainmenu;
mod editorapp;
mod dispatch;

use crate::editorapp::EditorApp;

fn main() {
    let app = EditorApp::new();
    app.run();
}
