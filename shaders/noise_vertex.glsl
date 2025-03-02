#version 100
precision lowp float;

attribute vec3 position; // The position of the vertex
attribute vec2 texcoord; // The texture coordinates

varying vec2 uv; // Output texture coordinates for the fragment shader

uniform mat4 Projection;
uniform mat4 Model;

void main() {
    // Transform the position of the vertex using the model and projection matrices
    gl_Position = Projection * Model * vec4(position, 1.0);
    
    // Pass the texture coordinates to the fragment shader
    uv = texcoord;
}
