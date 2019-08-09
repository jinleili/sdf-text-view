#include fs_micros
in vec2 v_tex_coord;

uniform vec2 texture_step;
uniform sampler2D skin_texture;

void main(void) {
    vec2 coord = vec2( v_tex_coord.x + (gl_PointCoord.x - .5) * texture_step.x,  (1. - v_tex_coord.y) + (gl_PointCoord.y - .5) * texture_step.y);
    out_color = vec4(texture(skin_texture, coord).rgb, 1.);
    // out_color = vec4(1., 1., 1., .1);
}
