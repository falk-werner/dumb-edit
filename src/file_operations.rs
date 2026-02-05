use std::path::PathBuf;

use crate::editorapp::EditorApp;
use fltk::{app, dialog::{self, FileDialogAction, NativeFileChooser, NativeFileChooserType}, prelude::{DisplayExt, WidgetExt}};

pub fn new(editor: &mut EditorApp) {
    if let Some(mut buffer) = editor.editor.buffer() {
        editor.win.set_label("*New* - DumbEdit");
        editor.filename.replace_range(.., "");
        editor.modified = false;
        buffer.set_text("");
    }
}

pub fn open(editor: &mut EditorApp) {
    let mut dialog = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
    if let Ok(action) = dialog.try_show() {
        match action {
            FileDialogAction::Success => {
                let name = dialog.filename();
                if ! name.as_os_str().is_empty() {
                    if let Some(mut buffer) = editor.editor.buffer() {
                        if let Some(filename) = name.file_name() {
                            editor.win.set_label(&format!("{} - DumbEdit", &filename.to_string_lossy()));
                        }
                        editor.modified = false;
                        editor.filename.replace_range(.., name.to_str().unwrap());
                        buffer.load_file(name).unwrap();
                    }
                }
            }
            _ => { }
        }
   }
}

pub fn save(editor: &mut EditorApp) {
    if editor.filename.is_empty() {
        save_as(editor);
        return;
    }

    if let Some(mut buffer) = editor.editor.buffer() {
        let path = PathBuf::from(&editor.filename);
        if let Some(filename) = path.file_name() {
            editor.win.set_label(&format!("{} - DumbEdit", &filename.to_string_lossy()));
        }
        editor.modified = false;
        buffer.save_file(&editor.filename).unwrap();
    }
}

pub fn save_as(editor: &mut EditorApp) {
    let mut dialog = NativeFileChooser::new(NativeFileChooserType::BrowseSaveFile);
    if let Ok(action) = dialog.try_show() {
        match action {
            FileDialogAction::Success => {
                let name = dialog.filename();
                if ! name.as_os_str().is_empty() {
                    if let Some(mut buffer) = editor.editor.buffer() {
                        if let Some(filename) = name.file_name() {
                            editor.win.set_label(&format!("{} - DumbEdit", &filename.to_string_lossy()));
                        }
                        editor.modified = false;
                        editor.filename.replace_range(.., name.to_str().unwrap());
                        buffer.save_file(name).unwrap();
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn ask_save_if_modified(editor: &mut EditorApp) -> bool {
    if !editor.modified {
        return true;
    }

    let x = (app::screen_size().0 / 2.0) as i32 - 200;
    let y = (app::screen_size().1 / 2.0) as i32- 100;
    let message = "Would you like to save your work?";

    match dialog::choice2(x, y,message, "&No", "&Yes", "&Abort") {
        Some(0) => true,
        Some(1) => { save(editor); true },
        Some(2) => false,
        _ =>  false,
    }
}