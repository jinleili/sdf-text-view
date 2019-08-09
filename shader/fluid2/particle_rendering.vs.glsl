
layout(location = 0) in vec2 tex_coord;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

// struct FluidCell {
//     vec3 color;
// };

// layout (set = 0, binding = 1) buffer FluidBuffer  { 
// 	FluidCell fluidCells[];
// };
// const float nx = 10.0;
// const float ny = 10.0;

// const float stripeX = 1.0 / nx;
// const float stripeY = 1.0 / ny;

layout(set = 0, binding = 1) uniform texture2D tex;
layout(set = 0, binding = 2) uniform sampler tex_sampler;


void main() {
    // float realX = uv.x / stripeX ;
    // float realY = (uv.y) / stripeY;
    // float minX = floor(realX);
    // float maxX = ceil(realX);
    // float minY = floor(realY);
    // float maxY = ceil(realY);
    // // 落在所在单元格的比重
    // float wx = (realX - minX) / stripeX;
    // float wh = (realY - minY) / stripeY;

    // // 四个采样格子
    // vec3 topleft = fluidCells[int(minX + minY * nx)].color;
    // vec3 topRight = fluidCells[int(maxX + minY * nx)].color;
    // vec3 bottomleft = fluidCells[int(minX + maxY * nx)].color;
    // vec3 bottomRight = fluidCells[int(maxX + maxY * nx)].color;

    // vec3 color = topleft * vec3(wx, wh, 1.0) + 

    gl_Position = mvp_matrix * vec4(texture(sampler2D(tex, tex_sampler), tex_coord).xy, 0.0, 1.0);
    gl_PointSize = 2.0;
}