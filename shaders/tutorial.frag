#version 330 core

in vec2 pos;

out vec4 outputColor;

uniform float iTime;

vec3 palette(float t)
{
  const vec3 a = vec3(0.938, 0.328, 0.718);
  const vec3 b = vec3(0.659, 0.438, 0.328);
  const vec3 c = vec3(0.388, 0.388, 0.296);
  const vec3 d = vec3(2.538, 2.478, 0.168);

  return a + b * cos(6.28318 * (c * t + d));
}

void main()
{
  vec3 finalColor = vec3(0.);
  vec2 uv = pos;

  for (int i = 0; i < 4; i++)
  {
    uv = fract(uv * 1.5) - 0.5;

    float d = length(uv) * exp(-length(pos));

    vec3 col = palette(length(pos) + i * .4 + iTime);

    d = sin(d * 8. + iTime) / 8.;
    d = abs(d);

    d = pow(0.005 / d, 1.2);

    finalColor += col * d;
  }

  outputColor = vec4(finalColor, 1.);
}
