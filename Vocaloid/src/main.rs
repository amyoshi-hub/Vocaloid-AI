use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use std::fs::File;
use std::io::{BufWriter, Result};

fn main() -> Result<()> {
    let input_files = vec!["a.wav", "b.wav", "c.wav"];
    let output_file = "output.wav";

    // 最初のファイルから仕様（Spec）を取得
    let spec = WavReader::open(&input_files[0])?.spec();

    let writer = BufWriter::new(File::create(output_file)?);
    let mut output = WavWriter::new(writer, spec)?;

    for file in &input_files {
        let reader = WavReader::open(file)?;
        for sample in reader.into_samples::<i16>() {
            output.write_sample(sample?)?;
        }
    }

    output.finalize()?; // 必ず finalize を呼ぶ
    println!("✅ 結合完了: {}", output_file);
    Ok(())
}

