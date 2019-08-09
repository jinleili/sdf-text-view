#include vs_micros

in vec2 tex_coord;
uniform mat4 mvp;
uniform sampler2D updated_position;

out vec2 px_tex_coord;
void main(void) {
    gl_Position = mvp * vec4(texture(updated_position, tex_coord).rg, 0.0, 1.0);
    gl_PointSize = 1.1;
    px_tex_coord = tex_coord;
}