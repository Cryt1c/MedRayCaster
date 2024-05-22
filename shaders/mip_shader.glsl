#version 330 core

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;

uniform sampler3D volume;
uniform vec3 cam_pos;
uniform uint lower_threshold;
uniform uint upper_threshold;

const int MAX_SAMPLES = 2000;
const vec3 MIN_TEX = vec3(0);
const vec3 MAX_TEX = vec3(1);
const float STEP_SIZE = 0.001;

void main() {
    vec3 data_position = vUV;
    vec3 direction = normalize((vUV - vec3(0.5)) - cam_pos);
    vec3 step = direction * STEP_SIZE;

    float max_value = 0.0;
    bool stop = false;

    for (int i = 0; i < MAX_SAMPLES; i++) {
        data_position += step;
        stop = dot(sign(data_position - MIN_TEX), sign(MAX_TEX - data_position)) < 3.0;

        if (stop) {
            vFragColor.rgba = vec4(max_value, max_value, max_value, max_value);
            break;
        }

        float value = texture(volume, data_position).r;

        float scaled_value = value * 255.0;
        if (scaled_value < lower_threshold || scaled_value > upper_threshold)
            continue;

        if (value > max_value) {
            max_value = value;
        }
    }
}
