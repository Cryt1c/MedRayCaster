#version 330 core

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;		

uniform sampler3D	volume;
uniform vec3		camPos;
// uniform vec3		step_size;

const int MAX_SAMPLES = 300;	
const vec3 texMin = vec3(0);	
const vec3 texMax = vec3(1);	

void main()
{ 
	float step_size = 0.01;
	vec3 dataPos = vUV;
	vec3 geomDir = normalize((vUV-vec3(0.5)) - camPos); 
	vec3 dirStep = geomDir * step_size; 
	bool stop = false; 

	for (int i = 0; i < MAX_SAMPLES; i++) {
        // Sample the volume
        vec4 value = texture(volume, dataPos);
        
        if (value.r > 0.1) {
            vFragColor = vec4(value.r, value.r, value.r, 1.0);
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
