#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_Uv /= vec2(gl_Position.x / Vertex_Position.z, gl_Position.y / Vertex_Position.z);
}
