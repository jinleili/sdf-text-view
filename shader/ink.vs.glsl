#include vs_micros
in vec2 position;
in vec2 tex_coord;

uniform mat4 mv_matrix;
uniform mat4 p_matrix;

uniform float p_size;

out vec2 v_tex_coord;

void main(void) {
  v_tex_coord = tex_coord;
  gl_Position = p_matrix * mv_matrix * vec4(position.xy, 0.0, 1.0);
  // 在着色器中影响点的大小默认是关闭的，需要开启 OpenGL 的 GL_PROGRAM_POINT_SIZE：
  gl_PointSize = p_size ;
}