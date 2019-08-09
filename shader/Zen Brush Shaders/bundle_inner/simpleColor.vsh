attribute vec4 position;
attribute vec2 uv;
attribute vec4 color;

uniform mat4 transformMatrix;

varying vec2 uvVarying;
varying vec4 colorVarying;

void main()
{
    gl_Position = transformMatrix * position;
	uvVarying = uv;
	colorVarying = color;
}