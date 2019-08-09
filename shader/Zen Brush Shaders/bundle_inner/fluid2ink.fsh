#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

uniform lowp sampler2D tex;
uniform vec4 ink;
uniform float alphaScale;
uniform float alphaOffset;

varying vec2 uvVarying;


void main()
{
	gl_FragColor = ink;
	gl_FragColor.a *= min(alphaScale * (texture2D(tex, uvVarying).a - alphaOffset), 1.0);
}
