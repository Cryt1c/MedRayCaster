#version 330

uniform mat4 m_model_view_projection_matrix;
in vec4 a_position;

out vec4 TexCoord0;

void main() {
	gl_Position = m_model_view_projection_matrix * a_position;
	TexCoord0 = a_position;
}
