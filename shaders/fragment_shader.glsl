#version 330
uniform mat4 ViewMatrix;
uniform mat3 NormalMatrix;

uniform float focal_length;
uniform float aspect_ratio;
uniform vec2 viewport_size;
uniform vec3 ray_origin;
uniform vec3 top;
uniform vec3 bottom;

uniform vec3 background_colour;
uniform vec3 material_colour;
uniform vec3 light_position;

uniform float step_length;
uniform float threshold;

uniform sampler3D volume;
uniform sampler2D jitter;

uniform float gamma;

// Ray
struct Ray {
    vec3 origin;
    vec3 direction;
};

// Axis-aligned bounding box
struct AABB {
    vec3 top;
    vec3 bottom;
};

out vec4 out_color;

in vec3 TexCoord;
in vec3 ourColor;

uniform sampler3D texture1;

void main() {
    out_color = texture(texture1, TexCoord);
}
