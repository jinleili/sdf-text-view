
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 params;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

layout(location = 0) out vec3 distance_params;
layout(location = 1) out vec2 tex_uv;

void main() {
    gl_Position = mvp_matrix * vec4(position, 1.0);
    distance_params = params;
    tex_uv = uv;
}