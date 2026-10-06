#version 300 es
precision highp float;
precision highp int;

uniform vec4 bounds; // xmin, xmax, ymin, ymax
uniform vec2 dimensions;
uniform int max_steps;
uniform float bailout;
uniform highp sampler2D palette;
out vec4 color;

vec2 multiply(vec2 a, vec2 b) {
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

int mandelbrot(vec2 input_point) {
    vec2 c0 = input_point;
    vec2 c = c0;
    vec2 dc = vec2(1.0, 0.0);
    vec2 dc_sum = vec2(0.0);
    for (int n = 1; n < max_steps; ++n) {
        // The same complex recurrence as Rust; complex squaring is expanded algebraically.
        c = multiply(c, c) + c0;
        dc = 2.0 * multiply(dc, c) + vec2(1.0, 0.0);
        dc_sum += dc;
        if (dot(dc_sum, dc_sum) >= bailout) {
            return n;
        }
    }
    return 0;
}

void main() {
    // GL's origin is bottom-left; the original image's first row uses ymin.
    vec2 pixel = vec2(gl_FragCoord.x - 0.5, dimensions.y - gl_FragCoord.y - 0.5);
    vec2 u = pixel / (dimensions - 1.0);
    vec2 point = vec2(
        bounds.x * (1.0 - u.x) + bounds.y * u.x,
        bounds.z * (1.0 - u.y) + bounds.w * u.y
    );
    int iteration = mandelbrot(point);
    // Preserve index = clamp(3 * iteration, 0, palette.len() - 3), without interpolation.
    int index = clamp(3 * iteration, 0, 3 * textureSize(palette, 0).x - 3);
    color = vec4(texelFetch(palette, ivec2(index / 3, 0), 0).rgb, 1.0);
}
