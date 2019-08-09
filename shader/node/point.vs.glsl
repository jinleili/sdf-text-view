layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 params;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

layout(location = 0) out vec3 v_params;

void main() {
    gl_Position = mvp_matrix * vec4(position, 1.0);
    v_params = params;
    gl_PointSize = 4.0;
}