#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

uniform lowp sampler2D tex; 
uniform vec4 color;

varying vec2 uvVarying;

void main()
{
	gl_FragColor = color;
	gl_FragColor.a *= 1.0 - texture2D(tex, uvVarying).r;
}
