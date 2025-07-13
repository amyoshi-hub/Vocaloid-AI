use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use std::io::Write;
mod roma_parser::split_romaji;

fn wav_shorter(input_path: &str, output_path: &str, ratio: f32) -> hound::Result<()> {
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    let shorter_len = (samples.len() as f32 * ratio) as usize;
    let shorter_samples = &samples[..shorter_len];

    let mut writer = WavWriter::create(output_path, spec)?;
    for &sample in shorter_samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    Ok(())
}

fn adjust_pitch(input_path: &str, output_path: &str, pitch_ratio: f32) -> hound::Result<()> {
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    // 単純リサンプリング（間引き or 重複で速度調整）
    let new_len = (samples.len() as f32 / pitch_ratio) as usize;
    let mut new_samples = Vec::with_capacity(new_len);

    for i in 0..new_len {
        let orig_idx = (i as f32 * pitch_ratio) as usize;
        let idx = if orig_idx < samples.len() { orig_idx } else { samples.len() -1 };
        new_samples.push(samples[idx]);
    }

    let mut writer = WavWriter::create(output_path, spec)?;
    for sample in new_samples {
        writer.write_sample(sample)?;
    }
    //短くする分そこに同じ音を入れれば良い
    writer.finalize()?;
    Ok(())
}


fn createWav(input_string: &str) -> Result<()> {
    let mut input_files: Vec<String> = Vec::new();
    let syllables = split_romaji(input_string);
    for s in input_string.chars() {
        if ch == ' ' {
            input_files.push("voice/silence.wav".to_string());
        }else{
            //wavを少し短く 
            input_files.push(format!("voice/__{}.wav", s)); 
        }
    }
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

fn main() -> Result<()>{
    let input_string = "konni iueo";
    createWav(input_string)?; 
    Ok(())
}
