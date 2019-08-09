attribute vec2 uv;

uniform vec2 uvOffset;

uniform vec2 uvSampling[8];

uniform vec2 uvDelta;
uniform lowp sampler2D tex;

varying vec2 uvVarying;

void main()
{
	vec2 uv2 = uv + uvOffset;
	float f = texture2D(tex, uv2).r;
	f += texture2D(tex, uv2 + uvSampling[0]).r;
	f += texture2D(tex, uv2 + uvSampling[1]).r;
	f += texture2D(tex, uv2 + uvSampling[2]).r;
	f += texture2D(tex, uv2 + uvSampling[3]).r;
	f += texture2D(tex, uv2 + uvSampling[4]).r;
	f += texture2D(tex, uv2 + uvSampling[5]).r;
	f += texture2D(tex, uv2 + uvSampling[6]).r;
	f += texture2D(tex, uv2 + uvSampling[7]).r;
	f *= 0.11111111111111;

    gl_Position = vec4(2.0 * (uv2 + uvDelta * f) - vec2(1.0, 1.0), 0.0, 1.0);
	uvVarying = uv2;
}