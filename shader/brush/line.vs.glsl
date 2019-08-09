
layout(location = 0) in vec3 position;
layout(location = 1) in float weight;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

layout(location = 0) out float distance_weight;

void main() {
    gl_Position = mvp_matrix * vec4(position, 1.0);
    distance_weight = weight;
}