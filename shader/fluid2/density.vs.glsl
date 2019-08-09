layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coord;
layout(location = 2) in vec2 pos_index;

layout(set = 0, binding = 0) uniform MVPUniform
{
    mat4 mvp_matrix;
};
layout(set = 0, binding = 1) uniform texture2D tex;
layout(set = 0, binding = 2) uniform sampler tex_sampler;

layout(location = 0) out float v_density;

void main()
{
    vec4 params = texture(sampler2D(tex, tex_sampler), tex_coord);
    gl_Position = mvp_matrix * vec4(params.xy, 0.0, 1.0);
    v_density = params.z;
}
