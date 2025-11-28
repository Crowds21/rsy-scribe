use ratatui::layout::Rect;

/// 适配器 Rect，使用 usize 作为尺寸类型
#[derive(Debug, Clone, Copy, PartialEq, Eq,Default)]
pub struct URect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl URect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }

    /// 转换为 ratatui 的 Rect（安全转换，会检查边界）
    pub fn to_ratatui(&self) -> Rect {
        Rect {
            x: self.x.min(u16::MAX as usize) as u16,
            y: self.y.min(u16::MAX as usize) as u16,
            width: self.width.min(u16::MAX as usize) as u16,
            height: self.height.min(u16::MAX as usize) as u16,
        }
    }

    /// 从 ratatui 的 Rect 转换
    pub fn from_ratatui(rect: Rect) -> Self {
        Self {
            x: rect.x as usize,
            y: rect.y as usize,
            width: rect.width as usize,
            height: rect.height as usize,
        }
    }
}

// 实现 From trait 以便更自然地转换
impl From<Rect> for URect {
    fn from(rect: Rect) -> Self {
        Self::from_ratatui(rect)
    }
}

impl From<URect> for Rect {
    fn from(urect: URect) -> Self {
        urect.to_ratatui()
    }
}

/// 常用函数
impl URect {
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn left(&self) -> usize {
        self.x
    }

    pub fn right(&self) -> usize {
        self.x + self.width
    }

    pub fn top(&self) -> usize {
        self.y
    }

    pub fn bottom(&self) -> usize {
        self.y + self.height
    }

    /// 百分比布局
    pub fn percentage(width_pct: usize, height_pct: usize, max_width: usize, max_height: usize) -> Self {
        let width = (max_width * width_pct) / 100;
        let height = (max_height * height_pct) / 100;
        Self::new(0, 0, width, height)
    }

    /// 居中
    pub fn centered_in(&self, container: &URect) -> Self {
        let x = container.x + (container.width.saturating_sub(self.width)) / 2;
        let y = container.y + (container.height.saturating_sub(self.height)) / 2;
        Self::new(x, y, self.width, self.height)
    }
}