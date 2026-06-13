use fltk::dialog;
use fltk::prelude::*;

use crate::editorapp::EditorApp;
use crate::message::Message;
use crate::file_operations;
use crate::search_operations;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn dispatch(app: &mut EditorApp) {
    if let Some(msg) = app.receiver.recv() {
        match msg {
            Message::Empty => {},
            Message::Change => {
                if !app.modified && ! app.win.label().ends_with("*") {
                    app.win.set_label(&format!("{}*", app.win.label()));
                }
                app.modified = true;
            },
            Message::Quit => {
                if file_operations::ask_save_if_modified(app) {
                    app.app.quit();
                }
            }
            Message::New => {
                if file_operations::ask_save_if_modified(app) {
                    file_operations::new(app);
                }
            }
            Message::Open => {
                if file_operations::ask_save_if_modified(app) {
                    file_operations::open(app);
                }
            },
            Message::Save => {
                file_operations::save(app);
            },
            Message::SaveAs => {
                file_operations::save_as(app);
            },            
            Message::LineWrap => {
                search_operations::line_wrap(app);
            }
            Message::FindFirst => {
                search_operations::find_first(app);
            },
           Message::FindNext => {
                search_operations::find_next(app);
            },
            Message::FindPrev => {
                search_operations::find_prev(app);
            },
            Message::Replace => {
                search_operations::replace(app);
            },
            Message::ShowInfo => {
                dialog::message_icon_label("!");
                dialog::message_title("Dumb Edit");
                dialog::message_default(format!("Dumb Edit v{}\n\
                    github.com/falk-werner/dumb-edit\n\
                    \n\
                    {}
                    \n\
                    This project uses Icons from www.flaticon.com.\n\
                    \n\
                    This project depends on the following crates:\n\
                    - https://crates.io/crates/serde (License: MIT or Apache-2.0)\n\
                    - https://crates.io/crates/toml (License: MIT or Apache-2.0)\n\
                    - https://crates.io/crates/fltk (License: MIT License)\n\
                    ",
                    VERSION,
                    include_str!("../LICENSE")).as_str());
            },
            Message::Load(filename) => {
                app.load(filename.as_str());
            }
        }
    }
}
