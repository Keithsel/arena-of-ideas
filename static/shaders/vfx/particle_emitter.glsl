#include <common.glsl>

#ifdef VERTEX_SHADER
out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

uniform vec2 u_position = vec2(0);
uniform float u_scale = 1;
uniform float u_scale_over_t = 0;
uniform int u_trail_count = 1;
uniform float u_lifetime = 1;

flat out int p_index;
flat out float p_t;

void main() {
    int trail_index = gl_InstanceID % u_trail_count;
    float trail_shift = 0.01 * trail_index;
    p_index = gl_InstanceID - trail_index;
    float time = u_global_time + u_lifetime * rand(p_index);
    p_t = time / u_lifetime - floor(time / u_lifetime) - trail_shift;
    v_quad_pos = a_pos;
    vec2 vel = rotateCW((randVec(p_index + 1) - vec2(0.5)), p_t * PI);
    vel = vec2(sign(vel.x) * vel.x * vel.x, sign(vel.y) * vel.y * vel.y);
    vel *= 5;
    vec2 pos = v_quad_pos * 1.0 * (u_scale + u_scale_over_t * p_t) + u_position + vel * p_t;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 v_quad_pos;
flat in int p_index;
flat in float p_t;

uniform vec4 u_start_color;
uniform vec4 u_end_color;

void main() {
    float dist = length(v_quad_pos);
    if(dist > 1. || p_t < 0 || p_t > 1)
        discard;
    gl_FragColor = vec4(mix(u_start_color, u_end_color, p_t).rgb, 0.5);
}
#endif