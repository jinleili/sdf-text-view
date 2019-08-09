layout(location = 0) in vec2 uv;
layout(location = 1) in vec3 verCoord;

layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform TurningUniform {
    // 开始卷动的半径
    float radius; 
    float angle;
    vec2 np; 
    vec2 n;
    float alpha;
};

layout(set = 0, binding = 2) uniform texture2D skin_texture;
layout(set = 0, binding = 3) uniform sampler tex_sampler;

const float whiteWeight = 0.85;
const float texWeight = 0.10;

void main() {
    vec4 texColor = texture(sampler2D(skin_texture, tex_sampler), uv);
    float diameter = radius * 2.0;
    if (verCoord.z > 0.0) {
        if (verCoord.z > radius) {
                texColor *= texWeight;
                vec4 newColor = vec4(whiteWeight + texColor.r , whiteWeight  + texColor.g, whiteWeight*0.98 + texColor.b, 1.0); 
            if (verCoord.z < diameter) {
                //模拟卷起片段的背面阴影, 卷起得越高,阴影越小
                 newColor.rgb *= (1.0 - 0.15 * ((diameter-verCoord.z) / radius));
                 frag_color = vec4(newColor.rgb, 1.0);
            } else {
                frag_color = newColor; 
            } 
        } else {
            //高效模拟卷起片段的内面阴影, 卷起得越高,阴影越大
            texColor.rgb *= (1.0 - 0.2 * (verCoord.z / radius));
            frag_color = vec4(texColor.rgb, 1.0);
        } 
        frag_color.a = alpha;       
    } else {
        frag_color = vec4(texColor.rgb, 1.0);
        // frag_color.a = alpha;
        // frag_color = vec4(0.7, 0.7, 0.7, 1.0);
    }
}