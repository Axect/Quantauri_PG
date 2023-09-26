use peroxide::fuga::*;
use quantauri::algorithm::{sma, ema, mstd, bollinger_band, macd};

fn main() {
    let mut df = DataFrame::read_parquet("close.parquet").expect("Can't read parquet");
    df.print();
    df.as_types(vec![Str, F64, F64, F64]);
    let data: Vec<f64> = df["close"].to_vec();
    let sma = sma(&data, 20);
    let ema = ema(&data, 12);
    let mstd = mstd(&data, 20, &sma);
    let bb = bollinger_band(&data, 2.0, &sma, &mstd);
    let (macd, signal) = macd(&data);

    let mut df = DataFrame::new(vec![]);
    df.push("data", Series::new(data));
    df.push("sma", Series::new(sma));
    df.push("ema", Series::new(ema));
    df.push("mstd", Series::new(mstd));
    df.push("bb_up", Series::new(bb.0));
    df.push("bb_down", Series::new(bb.1));
    df.push("macd", Series::new(macd));
    df.push("signal", Series::new(signal));

    df.print();

    df.write_parquet("test.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet");
}
