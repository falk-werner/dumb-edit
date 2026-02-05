#[derive(Copy, Clone)]
pub enum Message {
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
}