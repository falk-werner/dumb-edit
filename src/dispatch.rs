use fltk::prelude::*;

use crate::editorapp::EditorApp;
use crate::message::Message;
use crate::file_operations;
use crate::search_operations;

pub fn dispatch(app: &mut EditorApp) {
    if let Some(msg) = app.receiver.recv() {
        match msg {
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
            }
        }
    }
}
