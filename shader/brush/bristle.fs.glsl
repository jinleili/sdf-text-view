layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform BristleUniform {
    vec2 center;
};

const float RADIUS = 4.0;

void main() {
    vec2 new_coord = gl_FragCoord.xy - center;
    if (new_coord.x < 1.0 || new_coord.y < 1.0) {
        // 中心点
        frag_color = vec4(vec3(0.0), 1.0);
    } else {
        
    }
}