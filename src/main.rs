use peroxide::fuga::*;
use quantauri::base::{sma, ema, mstd, bollinger_band, macd, BollingerBand};

fn main() {
    let mut df = DataFrame::read_parquet("data/close.parquet").expect("Can't read parquet");
    df.print();
    df.as_types(vec![Str, F64, F64, F64]);

    let close: Vec<f64> = df["close"].to_vec();
    let high: Vec<f64> = df["high"].to_vec();
    let low: Vec<f64> = df["low"].to_vec();

    let tp = {
        let mut tp = vec![0f64; close.len()];
        for i in 0 .. close.len() {
            tp[i] = (high[i] + low[i] + close[i]) / 3f64;
        }
        tp
    };

    let sma_5 = sma(&tp, 5);
    let mut bollinger = BollingerBand::new(20, 2f64);
    bollinger.bb_mut(&tp);
    let ubb     = bollinger.get_ubb();
    let mbb     = bollinger.get_mbb();
    let lbb     = bollinger.get_lbb();
    let perb    = bollinger.per_b(&tp);
    let bw      = bollinger.bw(&tp);

    let mut df = DataFrame::new(vec![]);
    df.push("tp", Series::new(tp));
    df.push("sma_5", Series::new(sma_5));
    df.push("ubb", Series::new(ubb.clone()));
    df.push("mbb", Series::new(mbb.clone()));
    df.push("lbb", Series::new(lbb.clone()));
    df.push("perb", Series::new(perb));
    df.push("bw", Series::new(bw));
    df.print();

    df.write_parquet("data/bollinger.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet");
}
