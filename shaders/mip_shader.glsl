#version 330 core

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;		

uniform sampler3D	volume;
uniform vec3		camPos;

const int MAX_SAMPLES = 1000;	
const vec3 texMin = vec3(0);	
const vec3 texMax = vec3(1);	

void main() {
	float step_size = 0.001;
	vec3 dataPos = vUV;
	vec3 geomDir = normalize((vUV-vec3(0.5)) - camPos); 
	vec3 dirStep = geomDir * step_size; 
	bool stop = false; 

	for (int i = 0; i < MAX_SAMPLES; i++) {

        // Sample the volume
        float sample = texture(volume, dataPos).r;
        
        if (sample > 0.1) {
	    float scaled_sample = sample * 5;
            vFragColor.rgba = vec4(scaled_sample, scaled_sample, scaled_sample, 1.0);
            break;
        }

        // Advance ray position along ray direction
        dataPos = dataPos + dirStep;

        // Ray termination: Test if outside volume...
	stop = dot(sign(dataPos-texMin),sign(texMax-dataPos)) < 3.0;

	if (stop) 
		break;
	}
}
