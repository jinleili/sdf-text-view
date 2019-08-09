attribute vec4 position;
attribute vec4 color;

uniform mat4 transformMatrix;

varying vec4 colorVarying;

void main()
{
    gl_Position = transformMatrix * position;
	colorVarying = color;
}