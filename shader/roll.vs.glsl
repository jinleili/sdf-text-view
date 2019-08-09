layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coord;

layout(location = 0) out vec2 v_tex_coord;

layout(set = 0, binding = 0) uniform RollUniforms {
    mat4 mvp_matrix;
    // 卷动到哪个位置
    float roll_to ; 
    // 开始卷动的半径
    float start_radius ; 
};

// 屏幕一半的高度,用来计算顶点离坐标离屏幕原点(左下角)的位置
const float half_height = 1.0; 
const float PI = 3.14159265;
const float PI_2 = 1.5707963;

void main() {
    v_tex_coord = tex_coord;

    vec3 new_position = vec3(position, 0.);
    // position.y 的取值范围是 -1 ~ 1，将其转换成 0 ~ 2 的长度值来方便计算
    float point_y = position.y + 1.0;
    // red_value = point_y * 0.5;
    if (point_y < roll_to) {
        float step_radius = 0.5 * start_radius / 2.0 ;
        // 当前顶点的卷动半径：每一点的卷动半径是不一样的
        float current_radius = start_radius + step_radius * point_y;
        // 卷动停止点的半径
        float stop_radius = start_radius + step_radius * roll_to;
         // 卷动的圆的周长
        float perimeter = current_radius * 2.0 * PI;

        // 弧长需要按 roll_to 的点往下计算，离 roll_to 越远弧长越长
        float arc_length = mod((roll_to - point_y), perimeter);
        //卷起的弧度,每一个顶点被卷起的弧度值是不一样的;
        float radian = (-PI_2) + (arc_length / perimeter) * (2.0 * PI) ;

        new_position.y = (roll_to - cos(radian) * current_radius) - half_height;
        // new_position.y = (roll_to - sin(radian) * current_radius) - half_height;

        // 卷起后构成的同心圆中心点是固定的
        new_position.z  = (stop_radius + sin(radian) * current_radius);
        // new_position.z  = (stop_radius - current_radius)/2.0 + (current_radius - cos(radian)*current_radius);
    } 
    gl_Position = mvp_matrix * vec4(new_position, 1.0);
}