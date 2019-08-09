attribute vec4 position;
attribute vec2 uv;
attribute vec4 tintRedColor;
attribute vec4 tintGreenColor;

uniform mat4 transformMatrix;

varying vec2 uvVarying;
varying vec4 tintRedColorVarying;
varying vec4 tintGreenColorVarying;

void main()
{
    gl_Position = transformMatrix * position;
	uvVarying = uv;
	tintRedColorVarying = tintRedColor;
	tintRedColorVarying.rgb *= tintRedColor.a;
	tintGreenColorVarying = tintGreenColor;
	tintGreenColorVarying.rgb *= tintGreenColor.a;
}