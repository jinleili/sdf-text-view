#define BRICKWIDTH 160.
#define BRICKHEIGHT 85.

#define MORTARTHICKNESS 15.

#define BMWIDTH (BRICKWIDTH + MORTARTHICKNESS)
#define BMHEIGHT (BRICKHEIGHT + MORTARTHICKNESS)

// 半个石灰宽度与总半度对比，相当于将石灰在砖块内的坐标规范化
#define MWF (MORTARTHICKNESS * 0.5 / BMWIDTH) 
#define MHF (MORTARTHICKNESS * 0.5 / BMHEIGHT)

const vec3 brick_color = vec3(0.4, 0.4, 0.4);
const vec3 mortar_color = vec3(0.5, 0.5, 0.5);

out vec4 frag_color;

void main(void) {
    // 横向第几个块
    float ss = gl_FragCoord.x / BMWIDTH; 
    float tt = gl_FragCoord.y / BMHEIGHT;

    // 判断奇偶行，偶数行错开半块砖
    if (mod(tt * 0.5, 1.) > 0.5) {
        ss += 0.5; 
    } 

    // 纵横向哪个块
    float sbrick = floor(ss); 
    float tbrick = floor(tt); 

    // 块内坐标， 0~1
    ss -= sbrick; 
    tt -= tbrick;

    // 第一个 step = 是否在左边的石灰区，第二个 step = 是否在右边的石灰区,
    float w = step(MWF, ss) - step(1. - MWF, ss); 
    float h = step(MHF, tt) - step(1. - MHF, tt);
    
    // w, h 都为 0，则表示坐标在石灰区    
    frag_color = vec4(mix(mortar_color, brick_color, w * h), 1.);
}
 