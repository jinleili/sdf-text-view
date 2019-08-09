#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

varying float alphaRatioVarying;

uniform float fluid;

void main()
{
	gl_FragColor.a = alphaRatioVarying;
	gl_FragColor.b = alphaRatioVarying;
	gl_FragColor.r = fluid;
}
