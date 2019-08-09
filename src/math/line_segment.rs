use crate::math::Position;

pub struct LineSegment {
    p0: [f32; 3],
    p1: [f32; 3],
}

impl LineSegment {
    pub fn new(p0: [f32; 3], p1: [f32; 3]) -> Self {
        if (p0[0] == p1[0] && p0[1] > p1[1]) || p0[0] > p1[0] {
            return LineSegment { p0: p1, p1: p0 };
        } else {
            return LineSegment { p0, p1 };
        }
    }

    pub fn get_vector(&self) -> Position {
        Position::new(self.p1[0] - self.p0[0], self.p1[1] - self.p0[1])
    }

    pub fn is_intersect_with(&self, other: &LineSegment) -> bool {
        let p = self.get_vector();
        let q = other.get_vector();
        let p_x_q = p.cross_multiply(&q);

        let q_minus_p = Position::new(other.p0[0] - self.p0[0], other.p0[1] - self.p0[1]);
        let qpq = q_minus_p.cross_multiply(&q);

        // p × q = 0 几何意义：p, q啊夹角0°或180°，即p, q平行。
        if p_x_q == 0.0 {
            // (q1 - p1) × q = 0几何意义：p1q1与q的夹角为0°或180°，即p1在线段q所在直线上。
            if qpq == 0.0 {
                println!("collinear");
                return false;
            } else {
                println!("parallel");
                return false;
            }
        };

        //  t = [ (q1 - p1) × q ] / (p × q)
        // u = [ (q1 - p1) × p ] / (p × q)
        // 相交：p × q ≠ 0 且 t , u ∈ [0 , 1]，交点为p1 + t * p = q1 + u * q
        // https://blog.csdn.net/weixin_42736373/article/details/84587005
        let t = qpq / p_x_q;

        if t >= 0.0 && t <= 1.0 {
            let qpp = q_minus_p.cross_multiply(&p);
            let u = qpp / p_x_q;

            if u >= 0.0 && u <= 1.0 {
                return true;
            };
        };

        false
    }

    pub fn intersect_with(&self, other: &LineSegment) -> Option<[f32; 3]> {
        let p = self.get_vector();
        let q = other.get_vector();
        let p_x_q = p.cross_multiply(&q);

        let q_minus_p = Position::new(other.p0[0] - self.p0[0], other.p0[1] - self.p0[1]);
        let qpq = q_minus_p.cross_multiply(&q);

        // p × q = 0 几何意义：p, q啊夹角0°或180°，即p, q平行。
        if p_x_q == 0.0 {
            // (q1 - p1) × q = 0几何意义：p1q1与q的夹角为0°或180°，即p1在线段q所在直线上。
            if qpq == 0.0 {
                return None;
            } else {
                return None;
            }
        };

        //  t = [ (q1 - p1) × q ] / (p × q)
        // u = [ (q1 - p1) × p ] / (p × q)
        // 相交：p × q ≠ 0 且 t , u ∈ [0 , 1]，交点为p1 + t * p = q1 + u * q
        // https://blog.csdn.net/weixin_42736373/article/details/84587005
        let t = qpq / p_x_q;

        if t >= 0.0 && t <= 1.0 {
            let qpp = q_minus_p.cross_multiply(&p);
            let u = qpp / p_x_q;

            if u >= 0.0 && u <= 1.0 {
                let m = p.multiply_f(t);
                return Some([self.p0[0] + m.x, self.p0[1] + m.y, 0.0]);
            };
        };

        None
    }
}
