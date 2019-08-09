#include color_space_convert

layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform JuliaUniform {
     vec2 screen;
     vec2 const_c;
     float hue;
     float brightness;
};

const int max_repeat = 100;

vec3 julia_set_color(float hue, float brightness, float escape_time) {
    // float hue = 0.;
    // float brightness = 0.55;
    float saturation = 1.0;
    // 改变 saturation 的系数就会改变整个背景色
    saturation = brightness / 10. + 0.65 * saturation;
    brightness = 1.0 - brightness;
    vec3 color_out = hsb_to_rgl(hue, saturation, brightness);
    brightness = (1. - cos(brightness * 3.1415926)) / 6.;
    vec3 color_in = hsb_to_rgl(hue, saturation, brightness);

    vec3 rgb = vec3(0., 0., 0.);
    for (int i=0; i<3; i++) {
        // 保持色相不变
        if (color_in[i] != color_out[i]) {
            // 1.8 makes the image brighter
            rgb[i] = (escape_time - color_in[i]) / (color_out[i] - color_in[i]) * 1.8 ;
            // rgb[i] = mix( color_in[i], color_out[i], (1. - escape_time));
        } else {
            // rgb[i] = 0.;
            rgb[i] = (escape_time > color_in[i]) ? 1.0 : 0.0;
        }
    }
    return rgb;
}

vec2 multiply(vec2 a, vec2 b) {
	return vec2(a.x * b.x - a.y * b.y,  2.0 * a.y * b.x);
}

// const vec2 const_c2 = vec2(-0.769, 0.296);
void main(void) {
    vec2 center = vec2(screen.x / 2., screen.y / 2.);
    float wh_scale = screen.x;
    if ( screen.x > screen.y ) {
        wh_scale = screen.y;
    } 
    // 坐标缩放到 -2，2 之间
    float current_x = (gl_FragCoord.x - center.x) / wh_scale * 3.;
    float current_y = (gl_FragCoord.y - center.y) / wh_scale * 3.;

    // Z 的起始值设定为当前的坐标值
    vec2 Z = vec2(current_x, current_y);

    int stepper = 0;
    for (int i=0; i<max_repeat; i++) {
        stepper += 1;
        if (dot(Z, Z) > 4.0) {
            break;
        }
        Z = multiply(Z, Z) + const_c;
    }
    // escape-time: 0 to 1
    float escape_time = float(stepper) / float(max_repeat);
    frag_color = vec4(julia_set_color(hue, brightness, escape_time), 1.);
    // 在 ES 版本的着色器里，int 与 float 是不能直接  * 的
    // frag_color = vec4( mix(outer_color1, outer_color2, (1.0 - float(stepper) / float(max_repeat) ) ), 1.);//按迭代次数采用不同的颜色着色
    
  }