use mandelbrot::NumberType;

pub fn remap(
    value: NumberType,
    initial_range: (NumberType, NumberType),
    new_range: (NumberType, NumberType),
) -> NumberType {
    new_range.0
        + (new_range.1 - new_range.0) * (value - initial_range.0)
            / (initial_range.1 - initial_range.0)
}
