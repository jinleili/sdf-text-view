
layout(location = 0) in vec2 tex_coord;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

layout(set = 0, binding = 1) uniform texture2D tex;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

void main() {
    gl_Position = mvp_matrix * vec4(texture(sampler2D(tex, tex_sampler), tex_coord).xy, 0.0, 1.0);
    gl_PointSize = 2.0;
}