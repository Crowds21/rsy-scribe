
#[derive(Default)]
pub struct ViewPosition{
    /// 视图坐标(0,0)在整个界面中的行号
    pub anchor: usize,
    /// 视图的水平偏移量
    pub horizontal_offset: usize,
    /// 视图的垂直偏移量
    pub vertical_offset: usize,
}
