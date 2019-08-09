layout(location = 0) in vec3 distance_params;
layout(location = 1) in vec2 tex_uv;

layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 1) uniform LineUniform {
    // 用来做抗锯齿的, 最小 1.0
    float blur_distance;
    float edge_distance;
    float[32] lookup_table;
};
layout(set = 0, binding = 2) uniform texture2D tex0;
layout(set = 0, binding = 3) uniform sampler tex_sampler;


void main() {
    float distance_weight = distance_params.x;
    vec4 tex_color = texture(sampler2D(tex0, tex_sampler), tex_uv);

    // float i = ceil(distance_weight / 8.0);
    // float hair_center = (i - 1.0) * 8.0 ;
    
    // if (distance_weight < (hair_center + 1.) && distance_weight > (hair_center - 1.)) {
    //     float local = abs(distance_weight - hair_center);
    //     float alpha = 1.0;
    //     if (local > 0.5) {
    //         alpha = (1.0 - (local - 0.5) / 0.5);
    //     }
    //     frag_color = vec4(0., 0., 0., alpha) ;

    // } else {
    //     frag_color = vec4(0., 0., 0., 0.0) ;
    // }

    // frag_color = vec4(0., 0., 0., (1.0 - tex_color.r)) ;
    // frag_color = vec4(0., 0., 0., tex_color.r) ;

    frag_color = tex_color;
    // frag_color.a = frag_color.a / 5.0;
    // frag_color = vec4(0., 0., 0., 0.2);

    // if (hair_center < 1.0 ) {
    //     frag_color = vec4(0.8, 0.4, 0.4, 1.0) ;
    // } else {
    //     frag_color = vec4(0.8, 0.8, 0.8, 1.0) ;
    // }

    // if (distance_weight > edge_distance) {
    //     // float w = lookup_table[int((distance_weight - d) * 16.0)];
    //     float w = (1.0 - (distance_weight - edge_distance) / blur_distance) ;
    //     frag_color.a = w;
    // } 
}