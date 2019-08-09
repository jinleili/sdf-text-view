attribute vec2 uv;

uniform float alpha;

uniform vec2 wh;
uniform vec2 whInv;

uniform vec2 uvScale;
uniform vec2 uvOffset;

uniform float offsetMax;
uniform float margin;
uniform float marginInv;

varying vec2 uvVarying;
varying float alphaVarying;

uniform float numWaves;
uniform vec2 waveCenter[16];
uniform float waveRadius[16];
uniform float waveRatio[16];

const float PI = 3.14159296535;
const float PI_2 = 0.5 * PI;

void main()
{
	vec2 pos = uv * wh;
	vec2 offset = vec2(0.0, 0.0);
	float aDamp = 0.0;
	
	int n = int(numWaves);
	for (int i  = 0; i < n; i++) {
		vec2 dir = pos - waveCenter[i];
		float len = length(dir);
		float m = min(max((len - waveRadius[i]) * marginInv, -1.0), 1.0);

		aDamp += max(-m, 0.0);
		offset += offsetMax * sin(PI * waveRatio[i]) * cos(PI_2 * m) * normalize(dir);
	}

    gl_Position = vec4(2.0 * (pos + offset) * whInv - vec2(1.0, 1.0), 1.0, 1.0);
	uvVarying = uv * uvScale + uvOffset;
	alphaVarying = alpha * (1.0 - 0.3333 * aDamp);
}