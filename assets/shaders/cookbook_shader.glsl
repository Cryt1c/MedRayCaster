#version 300 es
precision highp float;
precision highp sampler3D;

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;

uniform sampler3D volume;
uniform vec3 cam_pos;
uniform uint lower_threshold;
uniform uint upper_threshold;

const int MAX_SAMPLES = 300;
const vec3 MIN_TEX = vec3(0);
const vec3 MAX_TEX = vec3(1);
const float STEP_SIZE = 0.01;

void main() {
    vec3 data_position = vUV;
    vec3 direction = normalize((vUV - vec3(0.5)) - cam_pos);
    vec3 dirStep = direction * STEP_SIZE;
    bool stop = false;

    for (int i = 0; i < MAX_SAMPLES; i++) {
        data_position += dirStep;

        stop = dot(sign(data_position - MIN_TEX), sign(MAX_TEX - data_position)) < 3.0;

        if (stop)
            break;

        float value = texture(volume, data_position).r;

        float scaled_value = value * 255.0;
        if (scaled_value < float(lower_threshold) || scaled_value > float(upper_threshold))
            continue;

        float prev_alpha = value - (value * vFragColor.a);
        vFragColor.rgb = prev_alpha * vec3(value) + vFragColor.rgb;
        vFragColor.a += prev_alpha;

        if (vFragColor.a > 0.99)
            break;
    }
}
