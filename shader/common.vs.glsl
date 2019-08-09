// 通用 vs
// 核心逻辑在片段着色器里时，不用重复创建同样代码顶点着色器
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoord;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

layout(location = 0) out vec2 uv;

void main() {
    gl_Position = mvp_matrix * vec4(position, 1.0);
    uv = texcoord;
}
