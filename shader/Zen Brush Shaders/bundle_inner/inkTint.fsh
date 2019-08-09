#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

uniform lowp sampler2D tex;

varying vec2 uvVarying;
varying vec4 tintRedColorVarying;
varying vec4 tintGreenColorVarying;


void main()
{
	vec4 col = texture2D(tex, uvVarying);
	gl_FragColor = tintRedColorVarying * col.r + tintGreenColorVarying * col.g;
}
