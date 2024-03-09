#version 330

// layout (location = 0) in vec3 aPos;
// layout (location = 1) in vec3 aColor;
// layout (location = 2) in vec2 aTexCoord;

// uniform double time;

// out vec3 ourColor;
// out vec3 TexCoord;

uniform float step_length;
uniform mat4 m_model_view_projection_matrix;
layout (location = 0) in vec4 a_position;

void main() {
	// gl_Position = vec4(aPos, 1.0);
	// ourColor = aColor;
	// TexCoord = vec3(aTexCoord.x, aTexCoord.y, time);
	// gl_Position = a_position;
	gl_Position = m_model_view_projection_matrix * a_position;
}
