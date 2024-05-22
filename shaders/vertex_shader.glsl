#version 330
layout(location = 0) in vec3 a_position;

uniform mat4 M;
uniform mat4 V;
uniform mat4 P;

smooth out vec3 vUV;

void main() {
    gl_Position = P * V * M * vec4(a_position, 1.0);
    vUV = a_position + vec3(0.5);
}
