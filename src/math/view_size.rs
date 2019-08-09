#[derive(Copy, Clone)]
pub struct ViewSize {
    pub width: f32,
    pub height: f32,
}

impl ViewSize {
    pub fn center_x(&self) -> f32 {
        self.width / 2.0
    }

    pub fn center_y(&self) -> f32 {
        self.height / 2.0
    }
}

impl From<ViewSize> for [f32; 2] {
    fn from(vs: ViewSize) -> Self {
        [vs.width, vs.height]
    }
}
