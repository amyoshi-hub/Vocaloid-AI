use hound::{WavReader, WavWriter, Result};

fn main() -> Result<()> {
    let input_files = vec!["a.wav", "b.wav", "c.wav"];
    let output_file = "output.wav";

    let spec = WavReader::open(&input_files[0])?.spec();
    let mut output = WavWriter::create(output_file, spec)?;

    for file in &input_files {
        let reader = WavReader::open(file)?;
        for sample in reader.into_samples::<i16>() {
            output.write_sample(sample?)?;
        }
    }

    output.finalize()?; // 必須
    println!("✅ 結合完了: {}", output_file);
    Ok(())
}
