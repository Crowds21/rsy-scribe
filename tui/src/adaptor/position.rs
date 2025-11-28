use ratatui::layout::{Position, Rect};
use crate::adaptor::rect::URect;

/// 使用 usize 的坐标点适配器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UPosition {
    pub x: usize,
    pub y: usize,
}

impl UPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    /// 安全转换为 ratatui Position
    pub fn to_ratatui(&self) -> Position {
        Position {
            x: self.x.min(u16::MAX as usize) as u16,
            y: self.y.min(u16::MAX as usize) as u16,
        }
    }

    /// 从 ratatui Position 转换
    pub fn from_ratatui(pos: Position) -> Self {
        Self {
            x: pos.x as usize,
            y: pos.y as usize,
        }
    }

    /// 带边界检查的安全转换
    pub fn try_to_ratatui(&self) -> Result<Position, PositionConversionError> {
        if self.x > u16::MAX as usize || self.y > u16::MAX as usize {
            return Err(PositionConversionError::ValueTooLarge {
                x: self.x,
                y: self.y,
            });
        }

        Ok(self.to_ratatui())
    }

    /// 坐标运算
    pub fn offset(&self, dx: isize, dy: isize) -> Self {
        let new_x = if dx >= 0 {
            self.x.saturating_add(dx as usize)
        } else {
            self.x.saturating_sub((-dx) as usize)
        };

        let new_y = if dy >= 0 {
            self.y.saturating_add(dy as usize)
        } else {
            self.y.saturating_sub((-dy) as usize)
        };

        Self::new(new_x, new_y)
    }

    pub fn distance_to(&self, other: &UPosition) -> f64 {
        let dx = (self.x as isize - other.x as isize).abs() as f64;
        let dy = (self.y as isize - other.y as isize).abs() as f64;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn within(&self, rect: &URect) -> bool {
        self.x >= rect.x &&
            self.y >= rect.y &&
            self.x < rect.x + rect.width &&
            self.y < rect.y + rect.height
    }
}

/// 错误类型
#[derive(Debug, thiserror::Error)]
pub enum PositionConversionError {
    #[error("Position value too large for u16: x={x}, y={y}")]
    ValueTooLarge { x: usize, y: usize },
}

// 转换 trait 实现
impl From<Position> for UPosition {
    fn from(pos: Position) -> Self {
        Self::from_ratatui(pos)
    }
}

impl From<UPosition> for Position {
    fn from(upos: UPosition) -> Self {
        upos.to_ratatui()
    }
}

impl From<(usize, usize)> for UPosition {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl From<UPosition> for (usize, usize) {
    fn from(pos: UPosition) -> Self {
        (pos.x, pos.y)
    }
}