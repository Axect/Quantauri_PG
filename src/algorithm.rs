use peroxide::fuga::*;

/// Simple Moving Average
///
/// # Arguments
/// * `v` - &Vec<f64>
/// * `window_size` - usize
///
/// # Returns
/// * Vec<f64>
///
/// # Examples
/// ```
/// use peroxide::fuga::*;
/// use quantauri::algorithm::sma;
///
/// fn main() {
///     let v = seq(1, 10, 1);
///     let sma = sma(&v, 3);
///     sma.print();
/// }
/// ```
pub fn sma(v: &[f64], window_size: usize) -> Vec<f64> {
    let mut result = vec![0f64; v.len()];
    for i in 0 .. window_size {
        result[i] = (0 .. i+1).map(|x| v[x]).sum::<f64>() / (i+1) as f64;
    }
    for i in window_size .. v.len() {
        result[i] = result[i-1] + (v[i] - v[i-window_size]) / window_size as f64;
    }
    result
}

/// Exponential Moving Average
///
/// # Arguments
/// * `v` - &Vec<f64>
/// * `window_size` - usize
///
/// # Returns
/// * Vec<f64>
///
/// # Examples
/// ```
/// use peroxide::fuga::*;
/// use quantauri::algorithm::ema;
/// fn main() {
///    let v = seq(1, 10, 1);
///    let ema = ema(&v, 12);
///    ema.print();
/// }
/// ```
pub fn ema(v: &[f64], window_size: usize) -> Vec<f64> {
    let mut result = vec![0f64; v.len()];
    let alpha = 2f64 / (window_size as f64 + 1f64);
    result[0] = v[0];
    for i in 1 .. v.len() {
        result[i] = alpha * v[i] + (1f64 - alpha) * result[i-1];
    }
    result
}

// Moving Standard Deviation
//
// # Arguments
// * `v` - &Vec<f64>
// * `window_size` - usize
// * `sma` - &[f64]
//
// # Returns
// * Vec<f64>
//
// # Examples
// ```
// use peroxide::fuga::*;
// use quantauri::algorithm::mstd;
//
// fn main() {
//    let v = seq(1, 10, 1);
//    let sma = sma(&v, 3);
//    let mstd = mstd(&v, 3, &sma);
//    mstd.print();
// }
// ```
pub fn mstd(v: &[f64], window_size: usize, sma: &[f64]) -> Vec<f64> {
    let mut result = vec![0f64; v.len()];
    for i in window_size-1 .. v.len() {
        result[i] = (0 .. window_size).map(|x| (v[i-x] - sma[i]).powi(2)).sum::<f64>() / window_size as f64;
        result[i] = result[i].sqrt();
    }
    result
}

// Bollinger Band
//
// # Arguments
// * `v` - &Vec<f64>
// * `amplitude` - f64
// * `sma` - &[f64]
// * `mstd` - &[f64]
//
// # Returns
// * (Vec<f64>, Vec<f64>)
//   * (upper, lower)
//   * upper = sma + amplitude * mstd
//   * lower = sma - amplitude * mstd
pub fn bollinger_band(v: &[f64], amplitude: f64, sma: &[f64], mstd: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut upper = vec![0f64; v.len()];
    let mut lower = vec![0f64; v.len()];
    for i in 0 .. v.len() {
        upper[i] = sma[i] + amplitude * mstd[i];
        lower[i] = sma[i] - amplitude * mstd[i];
    }
    (upper, lower)
}

// Moving Average Convergence Divergence
//
// # Arguments
// * `v` - &Vec<f64>
//
// # Returns
// * (Vec<f64>, Vec<f64>)
//   * (macd, signal)
//   * macd = ema(v, 12) - ema(v, 26)
//   * signal = ema(macd, 9)
pub fn macd(v: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let macd = ema(v, 12).sub_v(&ema(v, 26));
    let signal = ema(&macd, 9);
    (macd, signal)
}
