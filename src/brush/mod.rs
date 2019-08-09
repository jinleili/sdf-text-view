// mod line_antialiasing;
// pub use line_antialiasing::LineAntialiasing;

mod ink_deposite;
pub use ink_deposite::InkDeposite;

mod brush_view;
pub use brush_view::BrushView;

mod curve;
pub use curve::{cal_pos, Curve};

mod paper_node;
pub use paper_node::PaperNode;

mod brush_data;
pub use brush_data::{generate_vertices, get_touchpoints, touch_points};

mod stroke_cal;
pub use stroke_cal::StrokeCalculator;

mod endpoint_cal;
pub use endpoint_cal::*;
