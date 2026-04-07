pub fn pow(a: usize, b: usize) -> usize {
    match b {
        0 => 1,
        _ => a * pow(a, b - 1),
    }
}

pub fn lerp<T>(min: T, max: T, ratio: f64) -> T
where
    T: Into<f64> + From<f64>,
{
    let minf = min.into();
    let maxf = max.into();
    (minf + (ratio * (maxf - minf))).into()
}

pub fn lerp_usize(min: usize, max: usize, ratio: f64) -> usize {
    let minf = min as f64;
    let maxf = max as f64;
    f64::round(minf + (ratio * (maxf - minf))) as usize
}

pub fn lerp_ln_usize(min: usize, max: usize, ratio: f64) -> usize {
    let mut minf = min as f64;
    let maxf = max as f64;

    if minf == 0. {
        minf = 0. + 0.00001
    }

    f64::round(minf * (maxf / minf).powf(ratio)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(pow(0, 0), 1);
        assert_eq!(pow(0, 1), 0);
        assert_eq!(pow(0, 2), 0);
        assert_eq!(pow(1, 0), 1);
        assert_eq!(pow(1, 1), 1);
        assert_eq!(pow(1, 110), 1);
        assert_eq!(pow(2, 2), 4);
        assert_eq!(pow(3, 3), 27);
        assert_eq!(pow(1000, 1), 1000);
        assert_eq!(pow(1000, 2), 1000000);
        assert_eq!(pow(1000, 0), 1);
    }

    #[test]
    fn test_lerp_usize() {
        assert_eq!(lerp_usize(0, 10, 0.), 0);
        assert_eq!(lerp_usize(10, 10, 0.), 10);
        assert_eq!(lerp_usize(0, 10, 1.), 10);
        assert_eq!(lerp_usize(0, 10, 0.5), 5);
        assert_eq!(lerp_usize(10, 100, 0.), 10);
        assert_eq!(lerp_usize(10, 100, 1.), 100);
        assert_eq!(lerp_usize(10, 100, 0.5), 55);
    }

    #[test]
    fn test_lerp_log() {
        assert_eq!(lerp_ln_usize(1, 10, 0.99), 9);
        assert_eq!(lerp_ln_usize(0, 10, 0.99), 9);
    }
}
