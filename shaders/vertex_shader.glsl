#version 330
layout(location = 0) in vec3 a_position; //object space vertex position

uniform mat4 MVP;
uniform mat4 M;

smooth out vec3 vUV;

void main() {
	gl_Position =  MVP * vec4(a_position, 1.0);
	vUV = a_position + vec3(0.5);
}
