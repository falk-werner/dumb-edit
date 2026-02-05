use crate::message::Message;
use fltk::{app, button::{Button, ToggleButton, }, enums::Shortcut, frame::Frame, prelude::{ButtonExt, WidgetExt}, group, image::PngImage, input::Input, prelude::*};

pub struct MainMenu {
    pub frame: group::Flex,
    pub search: Input,
    pub replace: Input,
    pub match_case: ToggleButton,
    pub line_wrap: ToggleButton,
    pub find_next: Button,
    pub find_prev: Button,
    pub replace_button: Button,
}

impl MainMenu {
    pub fn new(sender: &app::Sender<Message>) -> Self {
        let mut frame = group::Flex::default().row();

        let mut new_button = Button::default();
        new_button.set_image(Some(PngImage::from_data( include_bytes!("img/document.png")).unwrap()));
        new_button.set_shortcut(Shortcut::Ctrl | 'n');
        new_button.set_tooltip("New");
        new_button.emit(*sender, Message::New);

        let mut open_button = Button::default();
        open_button.set_image(Some(PngImage::from_data( include_bytes!("img/folder-open.png")).unwrap()));
        open_button.set_shortcut(Shortcut::Ctrl | 'o');
        open_button.set_tooltip("Open...");
        open_button.emit(*sender, Message::Open);

        let mut save_button = Button::default();
        save_button.set_image(Some(PngImage::from_data( include_bytes!("img/disk.png")).unwrap()));
        save_button.set_shortcut(Shortcut::Ctrl | 's');
        save_button.set_tooltip("Save");
        save_button.emit(*sender, Message::Save);

        let mut save_as_button = Button::default();
        save_as_button.set_image(Some(PngImage::from_data( include_bytes!("img/floppy-disks.png")).unwrap()));
        save_as_button.set_shortcut(Shortcut::Ctrl | Shortcut::Alt | 's');
        save_as_button.set_tooltip("Save as...");
        save_as_button.emit(*sender, Message::SaveAs);


        let mut line_wrap = ToggleButton::default();
        line_wrap.set_image(Some(PngImage::from_data( include_bytes!("img/arrow-turn-down-left2.png")).unwrap()));
        line_wrap.set_shortcut(Shortcut::Ctrl | Shortcut::Alt | 'w');
        line_wrap.set_tooltip("Line wrap");
        line_wrap.emit(*sender, Message::LineWrap);
        line_wrap.toggle(true);


        let mut search_icon = Frame::default();
        search_icon.set_image(Some(PngImage::from_data( include_bytes!("img/search.png")).unwrap()));

        let mut search = Input::default();
        search.emit(*sender, Message::FindFirst);

        let mut match_case = ToggleButton::default();
        match_case.set_image(Some(PngImage::from_data( include_bytes!("img/letter-case.png")).unwrap()));
        match_case.set_tooltip("Match case");

        let mut find_next = Button::default();
        find_next.set_image(Some(PngImage::from_data( include_bytes!("img/arrow-down.png")).unwrap()));
        find_next.set_tooltip("Find next");
        find_next.emit(*sender, Message::FindNext);

        let mut find_prev = Button::default();
        find_prev.set_image(Some(PngImage::from_data( include_bytes!("img/arrow-up.png")).unwrap()));
        find_prev.set_tooltip("Find previous");
        find_prev.emit(*sender, Message::FindPrev);

        let mut replace_icon = Frame::default();
        replace_icon.set_image(Some(PngImage::from_data( include_bytes!("img/replace.png")).unwrap()));

        let replace = Input::default();

        let mut replace_button = Button::default();
        replace_button.set_image(Some(PngImage::from_data( include_bytes!("img/replace2.png")).unwrap()));
        replace_button.set_tooltip("Replace");
        replace_button.emit(*sender, Message::Replace);

        let button_size = 24;
        frame.fixed(&new_button, button_size);
        frame.fixed(&open_button, button_size);
        frame.fixed(&save_button, button_size);
        frame.fixed(&save_as_button, button_size);
        frame.fixed(&line_wrap, button_size);
        frame.fixed(&search_icon, button_size);
        frame.fixed(&match_case, button_size);
        frame.fixed(&find_next, button_size);
        frame.fixed(&find_prev, button_size);
        frame.fixed(&replace_icon, button_size);
        frame.fixed(&replace_button, button_size);

        frame.end();


        Self { frame, search, replace, match_case, line_wrap, find_next, find_prev, replace_button}
    }
}