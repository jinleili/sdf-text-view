use crate::math::Position;
use crate::vertex::PosWeight;

// 线条的宽度应该是整数才有意义
#[allow(dead_code)]
pub struct Line {
    start: Position,
    end: Position,
    width: u32,
}

#[allow(dead_code)]
impl Line {
    pub fn new(start: Position, end: Position, width: u32) -> Self {
        Line { start, end, width }
    }

    pub fn generate_vertices(&self) -> (Vec<PosWeight>, Vec<u16>) {
        let half_w = (self.width as f32) / 2.0;
        let left_top = PosWeight::new([self.start.x, self.start.y + half_w, 0.0], half_w);
        let left_bottom = PosWeight::new([self.start.x, self.start.y - half_w, 0.0], half_w);
        let right_top = PosWeight::new([self.end.x, self.end.y + half_w, 0.0], half_w);
        let right_bottom = PosWeight::new([self.end.x, self.end.y - half_w, 0.0], half_w);
        let s = PosWeight::new([self.start.x, self.start.y, 0.0], 0.0);
        let e = PosWeight::new([self.end.x, self.end.y, 0.0], 0.0);

        (
            vec![left_top, s, right_top, e, left_bottom, right_bottom],
            vec![0, 1, 2, 2, 3, 1, 3, 1, 4, 4, 5, 3],
        )
    }
}
