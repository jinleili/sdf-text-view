#include vs_micros

in vec2 position;
in vec2 target_position;

// 接近目标的比例
uniform float near_target_ratio;
uniform mat4 mvp;

void main(void) {
    vec2 new_position = position + (target_position - position) * near_target_ratio;
    gl_Position = mvp * vec4(new_position.xy , 0.0, 1.0);
}