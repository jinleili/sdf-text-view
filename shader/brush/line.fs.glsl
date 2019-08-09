layout(location = 0) in float distance_weight;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform LineUniform {
    // 用来做抗锯齿的, 最小 1.0
    float blur_distance;
    float edge_distance;
    float[32] lookup_table;
};

void main() {
    frag_color = vec4(0.6, 0.6, 0.6, 1.0) ;
    if (distance_weight > edge_distance) {
        // float w = lookup_table[int((distance_weight - d) * 16.0)];
        float w = (1.0 - (distance_weight - edge_distance) / blur_distance) ;
        frag_color.a = w;
    } 
}