float hue2rgb(float p, float q, float t) {
    if (t < 0.) {
        t += 1.;
    } else if (t > 1.) {
        t -= 1.;
    } else if (t < 1./6.) {
        return p + (q - p) * 6. * t;
    } else if(t < 1./2.) {
        return q;
    } else if(t < 2./3.) {
        return p + (q - p) * (2./3. - t) * 6.;
    }
    return p;         
}

vec3 hsl_to_rgb(float h, float s, float l) {
    float q = l < 0.5 ? l * (1. + s) : l + s - l * s;
    float p = 2. * l - q;
    return vec3(hue2rgb(p, q, h + 1./3.), hue2rgb(p, q, h), hue2rgb(p, q, h - 1./3.));
}

// h 取值为色相环的角度 0~360 度转换为 0~1 的值
vec3 hsb_to_rgl(float h, float s, float b) {
    // 落在色相环的哪个颜色上
    float step = h * 6.;
    // the hue position within the current step
    float pos = step - floor(step);
    float p = b * (1. - s);
    float q = b * (1. - pos * s);
    float t = b * (1. - (1. - pos) * s);

    vec3 rgb ;

    // 根据在色相环上的位置来转换,
    // Switch expression must be of type int or uint 
    // es 2.0 不支持 switch 写法
    // 
    int int_step = int(floor(step));
    if (int_step == 0) {
        rgb = vec3(b, t, p);
    } else if (int_step == 1) {
        rgb = vec3(q, b, p);
    } else if (int_step == 2) {
        rgb = vec3(p, b, t);
    } else if (int_step == 3) {
        rgb = vec3(p, q, b);
    } else if (int_step == 4) {
        rgb = vec3(t, p, b);
    } else if (int_step == 5) {
        rgb = vec3(b, p, q);
    }
    return rgb;
}