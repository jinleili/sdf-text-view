
layout(location = 0) out vec4 frag_color;
layout(location = 0) in float v_density;

void main(void)
{
    frag_color = vec4(0.9, (1.0 - v_density), (1.0 - v_density), 1.0);
}