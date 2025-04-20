
/// Editor UI 抽象层
pub struct Editor {
    pub mode: Mode,
}


pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
}

pub struct View{
    pub view_id: String,
    // pub area: Rect,
}
