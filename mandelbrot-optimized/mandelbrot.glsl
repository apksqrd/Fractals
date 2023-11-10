#ifdef GL_ES
precision highp float;
#endif

uniform vec2 u_resolution;

const float x_sample_range_min = -2.1, x_sample_range_max = 2.1, y_sample_range_min = -2.1, y_sample_range_max = 2.1;
const int max_iterations = 500;

float remap(float value, float initial_range_min, float initial_range_max, float new_range_min, float new_range_max) {
    return new_range_min + (new_range_max - new_range_min) * (value - initial_range_min) / (initial_range_max - initial_range_min);
}

// from wikipedia
int get_num_iterations(float x0, float y0) {
    float x = 0., y = 0., x2 = 0., y2 = 0.;

    // int iterations = 0;
    // while ((x2 + y2 <= 4.0) && (iterations < max_iterations)) {
        //     y = (x + x) * y + y0;
        //     x = x2 - y2 + x0;
        //     x2 = x * x;
        //     y2 = y * y;
        //     iterations++;
    // }

    for(int iterations = 0; iterations < max_iterations; iterations++) {
        if(x2 + y2 > 4.) {
            return iterations;
        }
        y = (x + x) * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
    }

    return max_iterations;
}

vec4 num_iterations_to_color(int num_iterations) {
    if(num_iterations == max_iterations) {
        return vec4(0, 0, 0, 1);
    }

    float num_iterations_float = float(num_iterations);

    float color_factor = 1. - (1. / (num_iterations_float + 1.));
    float color_factor_2 = color_factor * color_factor;

    return vec4((1. - color_factor_2 / 2.), color_factor_2, color_factor_2, 1);
}

void main() {
    // float x_sample = remap(gl_FragCoord.x, 0., resolution.x, x_sample_range_min, x_sample_range_max);
    // float y_sample = remap(gl_FragCoord.y, 0., resolution.y, y_sample_range_min, y_sample_range_max);
    float x_sample = remap(gl_FragCoord.x, 0., u_resolution.x, x_sample_range_min, x_sample_range_max);
    float y_sample = remap(gl_FragCoord.y, 0., u_resolution.y, y_sample_range_min, y_sample_range_max);

    // Output to screen
    gl_FragColor = num_iterations_to_color(get_num_iterations(x_sample, y_sample));
    // gl_FragColor = vec4(gl_FragCoord.x - floor(gl_FragCoord.x), gl_FragCoord.y - floor(gl_FragCoord.x), gl_FragCoord.z - floor(gl_FragCoord.z), 1.);
    // gl_FragColor = vec4(gl_FragCoord.x/u_resolution.x, gl_FragCoord.y - floor(gl_FragCoord.x), gl_FragCoord.z - floor(gl_FragCoord.z), 1.);
    // gl_FragColor = vec4(1., 1., 1., 1.);
}