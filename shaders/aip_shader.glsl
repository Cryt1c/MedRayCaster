#version 330 core

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;		

uniform sampler3D	volume;
uniform vec3		camPos;
uniform uint lower_threshold;
uniform uint upper_threshold;

const int MAX_SAMPLES = 2000;	
const vec3 texMin = vec3(0);	
const vec3 texMax = vec3(1);	

void main() {
	float step_size = 0.001;
	vec3 dataPos = vUV;
	vec3 geomDir = normalize((vUV-vec3(0.5)) - camPos); 
	vec3 dirStep = geomDir * step_size; 
	bool stop = false; 

	float aggregated_value = 0.0;
	int amount_of_samples = 0;
	for (int i = 0; i < MAX_SAMPLES; i++) {
		dataPos = dataPos + dirStep;
		stop = dot(sign(dataPos-texMin), sign(texMax-dataPos)) < 3.0;

		if (stop) {
      float average_value = aggregated_value / amount_of_samples;
			vFragColor.rgba = vec4(average_value, average_value, average_value, average_value);
			break;
		}

    float sample = texture(volume, dataPos).r;

		float scaled_sample = sample * 255.0;
		if (scaled_sample < lower_threshold || scaled_sample > upper_threshold)
			continue;

		aggregated_value += sample;
		amount_of_samples++;
	}
}
