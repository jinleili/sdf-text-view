layout(set = 0, binding = 0) uniform ParticleUniform {
    // lattice 在正规化坐标空间的大小
    vec2 lattice_size;
    // 粒子数
    vec2 particle_nxy;
};
layout(binding = 1, rgba32f) uniform image2D particles;

void main() {
	ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    vec2 normal_xy = (vec2(gl_GlobalInvocationID.xy) / particle_nxy) * 2.0;
    normal_xy = vec2(normal_xy.x - 1.0, normal_xy.y - 1.0);
    imageStore(particles, uv, vec4(normal_xy, 0.0, 0.0));

}