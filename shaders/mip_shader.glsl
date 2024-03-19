#version 330

uniform sampler3D SamplerDataVolume;
uniform vec2 viewport_size;

out vec4 FragColor;

void main()
{
    vec3 camera = vec3(0.0, 0.0, -1.0);
    float stepsize = 0.005;
    vec3 volExtentMin = vec3(-1.0, -1.0, -1.0);
    vec3 volExtentMax = vec3(1.0, 1.0, 1.0);
    vec4 value;
    float scalar;
    vec2 screen_xy = (gl_FragCoord.xy / viewport_size);
    vec3 normalized_screen = vec3(screen_xy, gl_FragCoord.z / gl_FragCoord.w);

    // Initialize accumulated color and opacity
    vec4 acc = vec4(0.0, 0.0, 0.0, 0.0);

    // Determine volume entry position
    vec3 position = normalized_screen.xyz;

    // Compute ray direction
    vec3 direction = normalized_screen.xyz - camera;
    direction = normalize(direction);

    // Loop for ray traversal
    for (int i = 0; i < 400; i++) // Some large number
    {
        // Sample the volume
        value = texture(SamplerDataVolume, position);
        
        if (value.r > 0.1) {
            acc = vec4(value.r, 0.0, 0.0, 1.0);
            break;
        }
        // Advance ray position along ray direction
        position += direction * stepsize;
        // Ray termination: Test if outside volume...
        vec3 temp1 = sign(position - volExtentMin);
        vec3 temp2 = sign(volExtentMax - position);
        float inside = dot(temp1, temp2);
        if (inside < 3.0)
            break;
    }
    FragColor = acc;
}

