layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

struct FluidCell {
    vec3 color;
};

layout (set = 0, binding = 1) buffer FluidBuffer  { 
	FluidCell fluidCells[];
};

const float nx = 10.0;
const float ny = 10.0;

const float stripeX = 1.0 / nx;
const float stripeY = 1.0 / ny;

void main(void) {
	int index = int(floor(uv.x / stripeX + ((uv.y) / stripeY) * nx));
	vec3 color = fluidCells[index].color;
	if (index > 9999) {
		    frag_color = vec4(0.0, 0.0, 1.0, 1.0);
	} else if (index >= 9900) {
				    frag_color = vec4(1.0, 0.0, 1.0, 0.7);
	} else {
    	frag_color = vec4(abs(mod(color, 1.0)), 1.0);
	}
	// frag_color = vec4(uv.x / stripeX  / nx, (uv.y / stripeY) * nx / (nx * ny), 0.3, 1.0);
}


