use fltk::{prelude::{DisplayExt, InputExt, WidgetExt}, text::WrapMode};

use crate::editorapp::EditorApp;

pub fn line_wrap(app: &mut EditorApp) {
    match app.mainmenu.line_wrap.is_toggled() {
        false => { app.editor.wrap_mode(WrapMode::None, 0); },
        true => { app.editor.wrap_mode(WrapMode::AtColumn, 100); },
    };    
}

pub fn find_first(app: &mut EditorApp) {
    let search_string = app.mainmenu.search.value();
    if search_string.is_empty() {
        return;
    }

    if let Some(mut buffer) = app.editor.buffer() {
        let match_case = app.mainmenu.match_case.is_toggled();
        
        if let Some(pos) = buffer.search_forward(0, &search_string, match_case) {
            buffer.select(pos, pos + search_string.len() as i32);
            app.editor.set_insert_position(pos);
            app.editor.show_insert_position();
            let _ = app.mainmenu.find_next.take_focus();
        }
    }
}

pub fn find_next(app: &mut EditorApp) {
    let search_string = app.mainmenu.search.value();
    if search_string.is_empty() {
        return;
    }

    if let Some(mut buffer) = app.editor.buffer() {
        let start = match buffer.selected() {
            false => 0,
            true => buffer.selection_position().unwrap().1 + 1,
        };
        let match_case = app.mainmenu.match_case.is_toggled();
        
        if let Some(pos) = buffer.search_forward(start, &search_string, match_case) {
            buffer.select(pos, pos + search_string.len() as i32);
            app.editor.set_insert_position(pos);
            app.editor.show_insert_position();
            let _ = app.mainmenu.find_next.take_focus();
        }
    }
}

pub fn find_prev(app: &mut EditorApp) {
    let search_string = app.mainmenu.search.value();
    if search_string.is_empty() {
        return;
    }

    if let Some(mut buffer) = app.editor.buffer() {
        let start = match buffer.selected() {
            false => buffer.length(),
            true => buffer.selection_position().unwrap().0 - 1,
        };
        let match_case = app.mainmenu.match_case.is_toggled();
        
        if let Some(pos) = buffer.search_backward(start, &search_string, match_case) {
            buffer.select(pos, pos + search_string.len() as i32);
            app.editor.set_insert_position(pos);
            app.editor.show_insert_position();
            let _ = app.mainmenu.find_prev.take_focus();
        }
    }
}

pub fn replace(app: &mut EditorApp) {    
    if let Some(mut buffer) = app.editor.buffer() {
        if let Some(pos) = buffer.selection_position() {
            let text = app.mainmenu.replace.value();
            buffer.replace(pos.0, pos.1, &text);
            find_next(app);
            let _ = app.mainmenu.replace_button.take_focus();
        }
    }
}
