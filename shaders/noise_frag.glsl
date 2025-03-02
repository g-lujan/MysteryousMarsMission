#version 100
precision lowp float;

uniform float time;              // Uniform for time-based animation
varying vec2 uv;                 // The texture coordinates passed from the vertex shader

// Random noise function based on UV coordinates
float random(vec2 p) {
    return fract(sin(dot(p, vec2(12.9898, 78.233))) * 43758.5453);
}

// Function to generate more complex noise
float noise(vec2 p) {
    float n = random(p + time);  // Add time for animation
    return n;
}

void main() {
    // Generate noise based on UV coordinates and time
    float n = noise(uv * 10.0);  // Multiply UV for better texture density

    // Add more noise variation for TV-like effect
    float tv_noise = random(vec2(uv.x * 20.0, uv.y * 20.0 + time * 0.5)); 

    // Combine both noise components and adjust for transparency
    float noise_value = mix(n, tv_noise, 0.5);  // Mix the noises for added complexity

    // Create a grayscale color with the noise value
    vec4 finalColor = vec4(noise_value, noise_value, noise_value, 0.7);  // 0.2 alpha for blending

    // Set the final fragment color
    gl_FragColor = finalColor;
}
