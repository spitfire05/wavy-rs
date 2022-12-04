//! Definitions of periodic functions.

use crate::PeriodicFunction;
use alloc::boxed::Box;
use core::f64::consts::PI;

/// Helper wrapping a custom periodic function
/// See: [Custom periodic functions]
///
/// # Examples
/// ```
/// let custom_func = wavegen::periodic_functions::custom(|t| t % 2.0);
/// ```
///
/// [Custom periodic functions]: ../index.html#custom-periodic-functions
#[inline(always)]
pub fn custom<F: Fn(f64) -> f64 + Send + Sync + 'static>(f: F) -> PeriodicFunction {
    Box::new(f)
}

/// DC Bias function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.dc_bias.html
pub fn dc_bias(bias: f64) -> PeriodicFunction {
    Box::new(move |_| bias)
}

/// Builder macro for DC Bias [PeriodicFunction].
///
/// Takes just one argument - the bias value.
///
/// # Examples
///
/// Defines bias of amplitude +10
/// ```
/// use wavegen::dc_bias;
///
/// let bias = dc_bias!(10);
///
/// assert!((0..100000).all(|x| bias(x as f64) == 10.0))
/// ```
#[macro_export]
macro_rules! dc_bias {
    ($bias:expr) => {
        $crate::periodic_functions::dc_bias($bias as f64)
    };
}

#[cfg(feature = "std")]
fn frac(x: f64) -> f64 {
    // this is actually slower than `x - ((x as i64) as f64)` on x86_64-pc-windows-msvc target,
    // but faster than the "casting hack" when `target-cpu=native` (tested on skylake)
    x.fract()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn frac(x: f64) -> f64 {
    use libm::modf;
    let (frac, _) = modf(x);

    frac
}

/// Sawtooth function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.sawtooth.html
pub fn sawtooth(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    Box::new(move |t| 2.0 * amplitude * frac(t * frequency + phase) - amplitude)
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
    (frequency: $frequency:expr) => {
        sawtooth!($frequency)
    };
    ($frequency:expr, $amplitude:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        sawtooth!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::sawtooth($frequency as f64, $amplitude as f64, $phase as f64)
    };
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
fn _sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    Box::new(move |t| {
        let radians = (2.0 * PI * frequency * t) + (phase * 2.0 * PI);
        let sine = radians.sin();

        sine * amplitude
    })
}

#[cfg(feature = "libm")]
fn _sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    use libm::sin;
    Box::new(move |t| sin((2.0 * PI * frequency * t) + (phase * 2.0 * PI)) * amplitude)
}

/// Sine function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.sine.html
pub fn sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    _sine(frequency, amplitude, phase)
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
///
/// # Examples
///
/// 50 Hz sine of amplitude 1 and no phase shift
/// ```
/// use wavegen::sine;
///
/// let sine = sine!(50);
/// ```
///
/// 50 Hz sine of amplitude 20 and no phase shift
/// ```
/// use wavegen::sine;
///
/// let sine = sine!(frequency: 50, amplitude: 20);
/// ```
///
/// 50 Hz sine of amplitude 20 and phase shift of half a turn
/// ```
/// use core::f64::consts::PI;
/// use wavegen::sine;
///
/// let sine = sine!(50, 20, 0.5);
/// ```
#[macro_export]
macro_rules! sine {
    (frequency: $frequency:expr) => {
        sine!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        sine!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        sine!($frequency, $amplitude, $phase)
    };
    ($frequency:expr) => {
        sine!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        sine!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::sine($frequency as f64, $amplitude as f64, $phase as f64)
    };
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
fn _square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    // TODO: implement duty cycle control
    Box::new(move |t| {
        let power = (2.0 * (t - phase) * frequency).floor() as i32;

        amplitude * (-1f64).powi(power)
    })
}

#[cfg(feature = "libm")]
fn _square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    // TODO: implement duty cycle control
    use libm::{floor, pow};
    Box::new(move |t| amplitude * pow(-1.0, floor(2.0 * (t - phase) * frequency)))
}

/// Square function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.square.html
pub fn square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    _square(frequency, amplitude, phase)
}

/// Builder macro for Square [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
#[macro_export]
macro_rules! square {
    (frequency: $frequency:expr) => {
        square!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        square!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr) => {
        square!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::square($frequency as f64, $amplitude as f64, $phase as f64)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const EPS: f64 = 1e-3;

    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias!(y);
        for x in (0..10000000).map(|x| x as f64) {
            assert_eq!(dc(x), y);
        }
    }

    #[test]
    fn frac_of_non_integer() {
        assert!(approx_eq!(f64, frac(1.5), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, frac(21.37), 0.37, epsilon = EPS));
        assert!(approx_eq!(f64, frac(42.69), 0.69, epsilon = EPS));
        assert!(approx_eq!(f64, frac(-5.55), -0.55, epsilon = EPS));
    }

    #[test]
    fn default_sawtooth_has_amplitude_of_one() {
        let f = sawtooth!(2.0);

        assert!(approx_eq!(f64, f(0.49999), 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, f(0.5), -1.0, epsilon = EPS));
    }

    #[test]
    fn default_sine_has_amplitude_of_one_and_no_phase_shift() {
        let sine = sine!(1);

        let max = sine(0.25);
        let min = sine(0.75);
        let zero = sine(0.5);

        assert!(approx_eq!(f64, max, 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, min, -1.0, epsilon = EPS));
        assert!(approx_eq!(f64, zero, 0.0, epsilon = EPS));
    }

    #[test]
    fn sine_phase_affects_min_max_amplitude_position() {
        let sine = sine!(1, 1, 0.5);

        let max = sine(0.75);
        let min = sine(0.25);
        let zero = sine(0.5);

        assert!(approx_eq!(f64, max, 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, min, -1.0, epsilon = EPS));
        assert!(approx_eq!(f64, zero, 0.0, epsilon = EPS));
    }

    #[test]
    fn default_square_has_amplitude_of_one() {
        let square = square!(1);

        for x in [0.0, 0.1, 0.2, 0.3, 0.4] {
            assert!(approx_eq!(f64, square(x), 1.0, epsilon = EPS))
        }

        for x in [0.5, 0.6, 0.7, 0.8, 0.9] {
            assert!(approx_eq!(f64, square(x), -1.0, epsilon = EPS))
        }
    }
}
