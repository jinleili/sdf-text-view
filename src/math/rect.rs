use crate::math::{Position, ViewSize};

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(width: f32, height: f32, center_to: ViewSize) -> Self {
        Rect {
            left: (center_to.width - width) / 2.0,
            top: (center_to.height - height) / 2.0,
            width,
            height,
        }
    }

    pub fn center_x(&self) -> f32 {
        self.width / 2.0
    }

    pub fn center_y(&self) -> f32 {
        self.height / 2.0
    }

    // 一个正交投影坐标是否在区域内
    pub fn is_ortho_intersect(&self, ortho_point: Position) -> bool {
        let x_left = -self.center_x();
        let x_right = self.center_x();
        let y_top = self.center_y();
        let y_bottom = -self.center_y();
        if ortho_point.x >= x_left
            && ortho_point.x <= x_right
            && ortho_point.y >= y_bottom
            && ortho_point.y <= y_top
        {
            true
        } else {
            false
        }
    }
}
