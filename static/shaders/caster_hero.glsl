#include <common.glsl>

varying vec2 v_quad_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    const float padding = 1.;
    v_quad_pos = a_pos * (1.0 + padding);
    float size = u_unit_radius * u_spawn * (1.0 - 0.25 * u_action);
    vec2 pos = v_quad_pos * size + u_unit_position;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER

vec3 getTriangleColor(vec2 uv, float ang, vec3 col, float size)
{
    const float tan3 = tan(pi/3.0);
    const float innerRad = sqrt(3.0) / 6.0;
    vec2 tuv = uv;
    tuv = vec2(
        tuv.x * cos(ang) + tuv.y * sin(ang),
        -tuv.x * sin(ang) + tuv.y * cos(ang));
    
    return col * float(
                   tuv.y + innerRad < (tuv.x + size) * tan3 + tan3 / 2.0
                && tuv.y + innerRad < -(tuv.x - size) * tan3 + tan3 / 2.0
                && tuv.y + innerRad > -size
                );
}

void main() {
    vec2 uv = v_quad_pos;

    vec3 colors[3];
    colors[0] = u_alliance_color_1.rgb;
    colors[1] = u_alliance_color_2.rgb;
    colors[2] = u_alliance_color_3.rgb;

    
    float anim = animationFunc(u_action) / 4.;
    
    float dist = distance(uv,vec2(0.0,0.0));
    
    vec4 col = vec4(0.,0.,0.,0.);
    if (dist < 1.0 - thickness)
    {
        col = vec4(colors[0],1);
        const float timeShift = 0.18;
        const float sizeShift = 0.15;
        float ang;
        for (float i = 8.0; i >= -1.0; i -= 1.0)
        {
            vec3 c = colors[mod(int(i + 10000.), 3)];
            ang = sin(u_time + timeShift * i);
            vec3 tc = getTriangleColor(uv,ang,c,sizeShift*i - sizeShift*0.0 + anim * -2.);
            if (tc != vec3(0.0,0.0,0.0))
                col = vec4(tc,1.);
        }
    }
    else if (dist > 1.0 - thickness && dist < 1.0 + thickness) {
        col = vec4(colors[0],1);
    }
    else if (dist > 1.0 && dist < 1.0 + glow)
    {
        float v = (dist - 1.0) / glow;
        col = vec4(colors[0], mix(glowStart, 0., v));
    } else {
        col = vec4(0);
    }

    gl_FragColor = col;
}
#endif