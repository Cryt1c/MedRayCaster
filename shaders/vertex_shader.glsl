#version 330
uniform mat4 ModelViewProjectionMatrix;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

uniform double time;

out vec3 ourColor;
out vec3 TexCoord;

void main() {
	gl_Position = vec4(aPos, 1.0);
	ourColor = aColor;
	TexCoord = vec3(aTexCoord.x, aTexCoord.y, time);
}
