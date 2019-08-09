#include fs_micros

uniform sampler2D texture0;
in vec2 px_tex_coord;

void main() {
    frag_color = texture(texture0, px_tex_coord);  
    // frag_color = vec4(0., 0., 0., 1.);
}