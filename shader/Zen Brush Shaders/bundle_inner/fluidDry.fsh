#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

uniform lowp sampler2D texNoise;

uniform vec4 dry;

varying vec2 uvVarying;

void main()
{
	gl_FragColor = dry * (0.5 + texture2D(texNoise, uvVarying).a);
}
