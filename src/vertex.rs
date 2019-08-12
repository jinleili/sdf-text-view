#[allow(dead_code)]

pub trait Pos {
    fn attri_descriptor(offset: u32) -> Vec<wgpu::VertexAttributeDescriptor>;
}

#[derive(Clone, Copy, Debug)]
pub struct PosTex {
    pos: [f32; 3],
    tex_coord: [f32; 2],
}

impl PosTex {
    pub fn vertex_i(pos: [i8; 3], tc: [i8; 2]) -> PosTex {
        PosTex {
            pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32],
            tex_coord: [tc[0] as f32, tc[1] as f32],
        }
    }

    pub fn vertex_f32(pos: [f32; 3], tex_coord: [f32; 2]) -> PosTex {
        PosTex { pos, tex_coord }
    }

    pub fn tex_offset() -> wgpu::BufferAddress {
        4 * 3
    }

    // pub fn vb_descriptor<'a>(offset: u32) -> wgpu::VertexBufferDescriptor<'a> {

    //     wgpu::VertexBufferDescriptor {
    //             stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
    //             step_mode: wgpu::InputStepMode::Vertex,
    //             attributes: &PosTex::attri_descriptor(offset)
    //         }
    // }
}

impl Pos for PosTex {
    fn attri_descriptor(offset: u32) -> Vec<wgpu::VertexAttributeDescriptor> {
        vec![
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 0,
                format: wgpu::VertexFormat::Float3,
                offset: 0,
            },
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float2,
                offset: PosTex::tex_offset(),
            },
        ]
    }
}

// pub struct VertexObj<T> {
//     pub pos_data: Vec<T>,
//     pub index_data: Vec<u16>,
// }

// impl VertexObj {
//     pub fn new() -> Self {

//     }
// }
#[derive(Clone, Copy, Debug)]
pub struct PosWeight {
    pub pos: [f32; 3],
    // 离数学中心位置的权重
    pub weight: f32,
}

impl PosWeight {
    pub fn new(pos: [f32; 3], weight: f32) -> Self {
        PosWeight { pos, weight }
    }

    pub fn slope_ridian(&self, last: &PosWeight) -> f32 {
        (self.pos[1] - last.pos[1]).atan2(self.pos[0] - last.pos[0])
    }
}

impl Pos for PosWeight {
    fn attri_descriptor(offset: u32) -> Vec<wgpu::VertexAttributeDescriptor> {
        vec![
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 0,
                format: wgpu::VertexFormat::Float3,
                offset: 0,
            },
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float,
                offset: 4 * 3,
            },
        ]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PosBrush {
    pos: [f32; 3],
    uv: [f32; 2],
    // weight, time_interval, pressure
    params: [f32; 3],
}

impl PosBrush {
    pub fn new(pos: [f32; 3], uv: [f32; 2], params: [f32; 3]) -> Self {
        PosBrush { pos, uv, params }
    }
}

impl Pos for PosBrush {
    fn attri_descriptor(offset: u32) -> Vec<wgpu::VertexAttributeDescriptor> {
        vec![
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 0,
                format: wgpu::VertexFormat::Float3,
                offset: 0,
            },
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float2,
                offset: 4 * 3,
            },
            wgpu::VertexAttributeDescriptor {
                shader_location: offset + 2,
                format: wgpu::VertexFormat::Float3,
                offset: 4 * (3 + 2),
            },
        ]
    }
}
