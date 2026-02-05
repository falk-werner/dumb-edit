use crate::message::Message;
use crate::mainmenu::MainMenu;
use crate::dispatch::dispatch;

use fltk::{app, enums::{CallbackTrigger, Event, Font}, group::Flex, image::PngImage, prelude::*, text, window::Window};

pub struct EditorApp {
    pub app: app::App,
    pub win: Window,
    pub receiver: app::Receiver<Message>,
    pub editor: text::TextEditor,
    pub mainmenu: MainMenu,
    pub filename: String,
    pub modified: bool,
}

impl EditorApp {

    pub fn new() -> Self {
        let app = app::App::default();
        app::get_system_colors();
        let (sender, receiver) = app::channel::<Message>();

        let mut win = Window::default()
            .with_size(640, 480)
            .with_label("*New* - DumbEdit");

        let mut col = Flex::default_fill().column();
        col.set_pad(0);

        let mainmenu = MainMenu::new(&sender);

        let mut editor = text::TextEditor::default();
        let mut buffer = text::TextBuffer::default();
        buffer.set_tab_distance(4);
        editor.set_buffer(buffer);
        editor.set_trigger(CallbackTrigger::Changed);
        editor.emit(sender, Message::Change);
        editor.wrap_mode(text::WrapMode::AtColumn, 100);

        editor.set_text_font(Font::CourierBold);
        editor.set_linenumber_width(40);

        win.resizable(&col);
        win.set_icon(Some(PngImage::from_data(include_bytes!("img/document.png")).unwrap()));
        win.set_callback(move |_| {
            if app::event() == Event::Close {
                sender.send(Message::Quit);
            }
        });
        
        // col.fixed(&m.menu, 30);
        col.fixed(&mainmenu.frame, 24);
        col.end();
        win.end();
        win.show();

        let _ = editor.take_focus();
        let filename = String::new();
        Self { app, win, receiver, editor, mainmenu, filename, modified: false }
    }

    pub fn run(mut self) {
        while self.app.wait() {
            dispatch(&mut self);
        }
    }
}
