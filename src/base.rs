use peroxide::fuga::*;

// =============================================================================
// High Level Structure
// =============================================================================
/// Bollinger Band
///
/// # Arguments
/// * `period` - usize
/// * `amplitude` - f64
pub struct BollingerBand {
    pub period: usize,
    pub amplitude: f64,
    pub ubb: Vec<f64>,
    pub mbb: Vec<f64>,
    pub lbb: Vec<f64>,
}

impl BollingerBand {
    pub fn new(period: usize, amplitude: f64) -> Self {
        BollingerBand {
            period,
            amplitude,
            ubb: vec![],
            mbb: vec![],
            lbb: vec![],
        }
    }

    pub fn get_ubb(&self) -> &Vec<f64> {
        &self.ubb
    }

    pub fn get_lbb(&self) -> &Vec<f64> {
        &self.lbb
    }

    pub fn get_mbb(&self) -> &Vec<f64> {
        &self.mbb
    }

    /// Calculate Bollinger Band
    ///
    /// # Arguments
    /// * `v` - &[f64]
    ///
    /// # Returns
    /// * (Vec<f64>, Vec<f64>)
    ///  * (ubb, lbb)
    ///  * ubb = sma + amplitude * mstd
    ///  * mbb = sma
    ///  * lbb = sma - amplitude * mstd
    pub fn bb(&self, v: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let sma = sma(v, self.period);
        let mstd = mstd(v, self.period, &sma);
        let (ubb, lbb) = bollinger_band(v, self.amplitude, &sma, &mstd);
        (ubb, sma, lbb)
    }

    pub fn bb_mut(&mut self, v: &[f64]) {
        let (ubb, mbb, lbb) = self.bb(v);
        self.ubb = ubb;
        self.mbb = mbb;
        self.lbb = lbb;
    }

    /// Calculate Percentage Band
    ///
    /// # Arguments
    /// * `v` - &[f64]
    ///
    /// # Returns
    /// * Vec<f64>
    ///   * (v - lbb) / (ubb - lbb)
    pub fn per_b(&self, v: &[f64]) -> Vec<f64> {
        let mut result = vec![0f64; v.len()];
        let ubb = self.get_ubb();
        let lbb = self.get_lbb();
        if ubb.len() > 0 {
            for i in 0 .. v.len() {
                let u = ubb[i];
                let l = lbb[i];
                if u == l {
                    result[i] = (v[i] - l) / (u - l + 1e-3);
                } else {
                    result[i] = (v[i] - lbb[i]) / (ubb[i] - lbb[i]);
                }
            }
        } else {
            let (ubb, _, lbb) = self.bb(v);
            for i in 0 .. v.len() {
                result[i] = (v[i] - lbb[i]) / (ubb[i] - lbb[i]);
            }
        }
        result
    }

    /// Calculate Band Width
    ///
    /// # Arguments
    /// * `v` - &[f64]
    ///
    /// # Returns
    /// * Vec<f64>
    ///  * (ubb - lbb) / mbb
    pub fn bw(&self, v: &[f64]) -> Vec<f64> {
        let mut result = vec![0f64; v.len()];
        let ubb = self.get_ubb();
        let mbb = self.get_mbb();
        let lbb = self.get_lbb();
        if self.ubb.len() > 0 {
            for i in 0 .. v.len() {
                result[i] = (ubb[i] - lbb[i]) / mbb[i];
            }
        } else {
            let (ubb, mbb, lbb) = self.bb(v);
            for i in 0 .. v.len() {
                result[i] = (ubb[i] - lbb[i]) / mbb[i];
            }
        }
        result
    }
}

// =============================================================================
// Low Level Function
// =============================================================================
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
//   * (ubb, lbb)
//   * ubb = sma + amplitude * mstd
//   * lbb = sma - amplitude * mstd
pub fn bollinger_band(v: &[f64], amplitude: f64, sma: &[f64], mstd: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut ubb = vec![0f64; v.len()];
    let mut lbb = vec![0f64; v.len()];
    for i in 0 .. v.len() {
        ubb[i] = sma[i] + amplitude * mstd[i];
        lbb[i] = sma[i] - amplitude * mstd[i];
    }
    (ubb, lbb)
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
