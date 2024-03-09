#version 330

in vec4 TexCoord0;
uniform sampler3D SamplerDataVolume;

out vec4 FragColor;

void main()
{
    vec3 camera = vec3(0.0, 0.0, -1.0);
    float stepsize = 0.01;
    vec3 volExtentMin = vec3(0.0, 0.0, 0.0);
    vec3 volExtentMax = vec3(1.0, 1.0, 1.0);
    vec4 value;
    float scalar;
    // Initialize accumulated color and opacity
    vec4 dst = vec4(0.0, 0.0, 0.0, 0.0);
    // Determine volume entry position
    vec3 position = TexCoord0.xyz;
    // Compute ray direction
    vec3 direction = TexCoord0.xyz - camera;
    direction = normalize(direction);
    // Loop for ray traversal
    for (int i = 0; i < 300; i++) // Some large number
    {
        // Data access to scalar value in 3D volume texture
        value = texture(SamplerDataVolume, position);
        // if (value.r > 0.0)
        // {
        //     dst = vec4(0.0, 1.0, 0.0, 0.0);        
        //     break;
        // }
        // else
        // {
        //     dst = vec4(1.0, 0.0, 0.0, 0.0);
        //     break;
        // }
        // // Apply transfer function
        if (value.r == 0.0)
            continue;
        vec4 src = vec4(vec3(1.0, 0.0, 0.0), value.r);
        // Front-to-back compositing
        dst = (1.0 - dst.a) * src + dst;
        // Advance ray position along ray direction
        position += direction * stepsize;
        // Ray termination: Test if outside volume...
        vec3 temp1 = sign(position - volExtentMin);
        vec3 temp2 = sign(volExtentMax - position);
        float inside = dot(temp1, temp2);
        if (inside < 3.0)
            break;
    }
    FragColor = dst;
}
