attribute float particleIndex;
attribute float dirIndex;

uniform vec4 particleData[128];
uniform vec2 unit[128];

uniform vec2 dir[6];
uniform float dirAlpha[6];

uniform mat4 transformMatrix;
varying float alphaRatioVarying;

void main()
{
	int pi = int(particleIndex);
	int di = int(dirIndex);
	
	vec2 d = dir[di];
	vec2 u = unit[pi];
	vec4 particle = particleData[pi];
	
	gl_Position = transformMatrix * vec4(particle.xy + particle.z * (d.x * u + d.y * vec2(u.y, -u.x)), 0.0, 1.0);
	alphaRatioVarying = particle.a * dirAlpha[di];
}