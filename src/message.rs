#[derive(Clone)]
pub enum Message {
    Empty,
    Change,
    Quit,

    New,
    Open,
    Save,
    SaveAs,

    LineWrap,

    FindFirst,
    FindNext,
    FindPrev,
    Replace,

    ShowInfo,

    Load(String),
}