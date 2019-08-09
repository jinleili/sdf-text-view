layout(location = 0) in vec3 v_params;
layout(location = 0) out vec4 frag_color;

void main() {
    // if (abs(distance_weight) <= 0.1) {
    //     frag_color = vec4(0.9, 0., 0., 1.0);
    // } else if (distance_weight >= 0.4) {
    //     frag_color = vec4(0.0, 0.9, 0.0, 1.0);
    // } else {
    //     frag_color = vec4(0.9, 0.9, 0.9, 1.0);
    // }
    frag_color = vec4(v_params, 0.7);
}