layout(location = 0) in vec3 position;
layout(location = 1) in vec2 tex_coord;

layout(location = 0) out vec2 uv;
layout(location = 1) out vec3 verCoord;

layout(set = 0, binding = 0) uniform MVPUniform {
    mat4 mvp_matrix;
};

layout(set = 0, binding = 1) uniform TurningUniform {
    // 开始卷动的半径
    float radius; 
    float angle;
    vec2 np; 
    vec2 n;
    float alpha;
};

const float PI = 3.14159265358979;
const float PI_2 = 3.14159265358979 / 2.0;

void main() {
    uv = tex_coord;
    // 从 np 位置到 position 的矢量
    vec2 v = position.xy - np;
    // v 在单位矢量 n 上的投影长度
    float l = dot(v, n);

    // 投影长度值为正，表示 position 是需要被卷起的点 
    if (l > 0.0) {
        // 半圆周长
        float half_circle = PI * radius;
        vec3 new_position = position.xyz;

        // position 卷起后与之前的位置差
        float d = 0.0;

        // 切点到 half_circle 之间的顶点计算卷起
        if (l <= half_circle) {
            // 被卷起的弧度
            float degress = (l / half_circle) * PI - PI_2 ;
            d = l - cos(degress) * radius;
            // position 卷起后的高度
            new_position.z  = (radius + sin(degress) * radius) ;
        } else {
            d = l + (l - half_circle);
            // half_circle 之外的顶点，z 轴是固定的圆的直径
            new_position.z  = radius * 2.0;
        }
        new_position.y -= sin(angle) * d ;
        new_position.x -= cos(angle) * d;

        verCoord = new_position;
        gl_Position = mvp_matrix * vec4(new_position, 1.0);
    } else {
        verCoord = position;
        gl_Position = mvp_matrix * vec4(position, 1.0);
    }

}