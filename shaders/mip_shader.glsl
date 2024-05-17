#version 330 core

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;		

uniform sampler3D	volume;
uniform vec3		camPos;

const int MAX_SAMPLES = 2000;	
const vec3 texMin = vec3(0);	
const vec3 texMax = vec3(1);	

void main() {
	float step_size = 0.001;
	vec3 dataPos = vUV;
	vec3 geomDir = normalize((vUV-vec3(0.5)) - camPos); 
	vec3 dirStep = geomDir * step_size; 
	bool stop = false; 

	float max_value = 0.0;
	for (int i = 0; i < MAX_SAMPLES; i++) {
		dataPos = dataPos + dirStep;
		stop = dot(sign(dataPos-texMin), sign(texMax-dataPos)) < 3.0;

		if (stop) {
			vFragColor.rgba = vec4(max_value, max_value, max_value, max_value);
			break;
		}

    float sample = texture(volume, dataPos).r;

		if (sample > max_value) {
	    max_value = sample;
		}
	}
}
