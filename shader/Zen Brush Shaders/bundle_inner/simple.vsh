attribute vec4 position;
attribute vec2 uv;
attribute float alpha;

uniform mat4 transformMatrix;

varying vec2 uvVarying;

void main()
{
    gl_Position = transformMatrix * position;
	uvVarying = uv;
}