#version 300 es

layout(location = 0) out vec4 vFragColor;

smooth in vec3 vUV;		

uniform sampler3D	volume;
uniform vec3		camPos;

const int MAX_SAMPLES = 300;	
const vec3 texMin = vec3(0);	
const vec3 texMax = vec3(1);	

void main() {
	float step_size = 0.01;
	vec3 dataPos = vUV;
	vec3 geomDir = normalize((vUV-vec3(0.5)) - camPos); 
	vec3 dirStep = geomDir * step_size; 
	bool stop = false; 

	for (int i = 0; i < MAX_SAMPLES; i++) {
		dataPos = dataPos + dirStep;
		
		stop = dot(sign(dataPos-texMin),sign(texMax-dataPos)) < 3.0;

		if (stop) 
			break;
		
		float sample = texture(volume, dataPos).r;	
		
		float prev_alpha = sample - (sample * vFragColor.a);
		vFragColor.rgb = prev_alpha * vec3(sample) + vFragColor.rgb; 
		vFragColor.a += prev_alpha; 
			
		if( vFragColor.a>0.99)
			break;
	} 
}
