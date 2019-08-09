#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

uniform lowp sampler2D tex;

varying vec2 uvVarying;

void main()
{
	gl_FragColor.rgb = texture2D(tex, uvVarying).rgb;
	gl_FragColor.a = gl_FragColor.b;
}
