#include vs_micros

in vec2 position;
in vec2 position2;
in vec2 tex_coord;

uniform mat4 mvp;
// 0: 第一帧； 1: 初始化速率与方向；2: 扩散隐形 + 碰撞反弹；3: 初始化显形动画的速率； 4：显形; 
uniform int status;
uniform float step_rate;
uniform float frame_index;
// 1 屏幕像素步长
uniform float pixel_step;

uniform sampler2D texture_noise;
uniform sampler2D last_position;

// 计算碰撞反弹：
// 与水平方向的壁面碰撞后的反弹角 out_angle = PI + (PI - in_angle) = PI_2 - in_angle
// 与竖直方向的壁面碰撞后的反弹角 out_angle = PI - in_angle

// rg: posiotion, b: 速率， a: 方向角
out vec4 updated_param;

const float PI_2 = 6.2831;
const float PI = 3.14159;


vec2 boundary_detect(vec2 position) {
    position.x = clamp(-1., position.x, 1.);
    position.y = clamp(-1., position.y, 1.);
    return position;
}

void main(void) {
    updated_param = vec4(0., 0., 0., 1.);
    if (status == 0) {
        updated_param.xy = position;
    } else if (status == 1) {
        vec2 p = position2 - position;
        updated_param.xy = p * step_rate;
        // 速率
        updated_param.z = sqrt(updated_param.x * updated_param.x + updated_param.y * updated_param.y);
        if (updated_param.z < 0.003) {
            updated_param.z = 0.003;
        }
        updated_param.xy += position;
        updated_param.a = atan(p.y, p.x);
    } else {
        updated_param = texture(last_position, tex_coord);
        if (status == 2) {
            // 与竖直（左右）壁面碰撞
            if (updated_param.x < -0.99 || updated_param.x > 0.99) {
                updated_param.a = PI - updated_param.a;
            } else if (updated_param.y < -0.99 || updated_param.y > 0.99) {
                updated_param.a = PI_2 - updated_param.a;
            }
            float noise = texture(texture_noise, (updated_param.xy + 1.) / 2.).r ;
            float angle = updated_param.a; 
            updated_param.x += cos(angle) * (updated_param.z + noise * 0.01);
            updated_param.y += sin(angle) * (updated_param.z + noise * 0.01);
        } else if (status == 3) {
            vec2 p = updated_param.xy - position.xy;
            float move_distance = sqrt(p.x * p.x + p.y * p.y);
            // 以平均步长 5 个像素计算步数
            updated_param.z = (pixel_step * 10.) / move_distance;
            if (updated_param.z > 1.) {
                updated_param.z = 1.;
            } 
            // 至少在一定帧内完成移动
            else if (updated_param.z < 0.03) {
                updated_param.z = 0.03;
            }
        } else {
            updated_param.xy = updated_param.xy - (updated_param.xy - position.xy) * updated_param.z;
        }
    }
    // 离屏渲染生成纹理时，左下角是原点，而后续要使用的是左上角为原点的纹理，得反转一下 Y 坐标
    gl_Position = mvp * vec4(position.x, -position.y, 0.0, 1.0);
    gl_PointSize = 1.0;
}
