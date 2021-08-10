pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    if n < min {
        return min;
    };
    if n > max {
        return max;
    };
    return n;
}
