use alloc::boxed::Box;

use crate::PeriodicFunction;

fn frac(x: f64) -> f64 {
    let i = x as i64;

    x - i as f64
}

pub fn _sawtooth(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    Box::new(move |t| 2.0 * amplitude * frac(t / (1.0 / frequency) + phase) - amplitude)
}

/// Builder macro for Sine [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
#[macro_export]
macro_rules! sawtooth {
    ($frequency:expr) => {
        sawtooth!($frequency, 1.0, 0.0)
    };
    (frequency = $frequency:expr) => {
        sawtooth!($frequency)
    };
    ($frequency:expr, $amplitude:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    (frequency = $frequency:expr, amplitude = $amplitude:expr) => {
        sawtooth!($frequency, $amplitude)
    };
    (frequency = $frequency:expr, amplitude = $amplitude:expr, phase = $phase:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::sawtooth::_sawtooth(
            $frequency as f64,
            $amplitude as f64,
            $phase as f64,
        )
    };
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::frac;

    const EPS: f64 = 1e-3;

    #[test]
    fn frac_of_non_integer() {
        assert!(approx_eq!(f64, frac(1.5), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, frac(21.37), 0.37, epsilon = EPS));
        assert!(approx_eq!(f64, frac(42.69), 0.69, epsilon = EPS));
    }

    #[test]
    fn default_sawtooth_has_amplitude_of_one() {
        let f = sawtooth!(2.0);

        assert!(approx_eq!(f64, f(0.49999), 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, f(0.5), -1.0, epsilon = EPS));
    }
}