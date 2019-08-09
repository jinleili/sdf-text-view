layout(location = 0) in vec2 v_tex_coord;
layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform Roll {
    mat4 mvp_matrix;
    // 卷动到哪个位置
    float roll_to ; 
    // 开始卷动的半径
    float start_radius ; 
};

layout(set = 0, binding = 1) uniform texture2D skin_texture;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

// uniform 结构体里传递布尔值似乎一直是 false
const bool use_texture = true;

void main() {
    if (use_texture) {
        vec4 color = texture(sampler2D(skin_texture, tex_sampler), v_tex_coord);
        color.a = 0.1;
        out_color = color;
    } else {
        out_color = vec4(.5, .6, .9, 0.1);
    } 
}
