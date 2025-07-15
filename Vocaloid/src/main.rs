use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use std::io::{self, Write};
mod roma_parser;
use roma_parser::split_romaji;
use fileio_handle::FileIO;

fn wav_shorter(input_path: &str, output_path: &str, ratio: f32) -> hound::Result<()> {
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();
    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    let shorter_len = ((samples.len() as f32 * ratio).min(samples.len() as f32)) as usize;
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

fn emotion_to_pitch(p: &[f32]) -> f32 {
            //14このパラメーターのうち７こはプラスなのでpitchは高くゆっくりに
            if p.is_empty() {
                return 1.0;
            }
            let half = p.len() / 2;
            let mut emotion = 0.0;
            for (i, val) in p.iter().enumerate() {
                if i >= half {
                    emotion += val;
                }else{
                    emotion -= val;
                }
            }
            let ratio = 1.0 + emotion / 1000.0 / 7.0;
            ratio.clamp(0.5, 1.1)
}


fn create_word_wav(word: &str, temp: &str) -> hound::Result<()>{
    let syllables = split_romaji(word);
    let out_file = format!("{}.wav", word);
    let spec = WavReader::open(format!("voice/__{}.wav", syllables[0]))?.spec();
    let mut writer = WavWriter::create(temp, spec)?;

    for s in syllables {
        let input_file = format!("voice/__{}.wav", s);    
        let render = WavReader::open(&input_file)?;
        for sample in render.into_samples::<i16>() {
            writer.write_sample(sample?)?;    
        }
    }
    writer.finalize()?;
    Ok(())
}

fn create_wav(words: &[String], params: &[Vec<f32>]) -> hound::Result<()> {
    let mut input_files = Vec::new();
    std::fs::create_dir_all("temp")?;

    for (i, word) in words.iter().enumerate() {
        let temp_main = format!("temp/word_{}.wav", i);

        if word == " " {
            let silence_file = "voice/silence.wav";
            std::fs::copy(silence_file, &temp_main)?;
        } else {
            create_word_wav(word, &temp_main)?;
        }

        let adjusted_file = format!("temp/pitched_{}.wav", i);
        let pitch = emotion_to_pitch(&params[i]);
        println!("{}", pitch);

        wav_shorter(&temp_main, &adjusted_file, pitch)?;
        adjust_pitch(&adjusted_file, &adjusted_file, pitch)?;
        input_files.push(adjusted_file);
    }

    // 最後に結合
    let output_file = "output.wav";
    let spec = WavReader::open(&input_files[0])?.spec();
    let mut output = WavWriter::create(output_file, spec)?;

    for file in &input_files {
        let reader = WavReader::open(file)?;
        for sample in reader.into_samples::<i16>() {
            output.write_sample(sample?)?;
        }
    }

    output.finalize()?;
    println!("結合完了: {}", output_file);
    Ok(())
}

fn main() -> hound::Result<()> {
    let mut file = FileIO::new("lyric.txt");
    file.read_lines()?;
    let sep_list = [","];
    let mut words = Vec::new();
    let mut params = Vec::new();

    for line in &file.contents {
        let splits = FileIO::phaser(line, &sep_list);
        if splits.is_empty() {
            continue;
        }

        let word = splits[0].clone();
        let param: Vec<f32> = splits[1..]
            .iter()
            .filter_map(|s| s.parse::<f32>().ok())
            .collect();

        words.push(word);
        params.push(param);
    }

    println!("{:?}", words);
    println!("{:?}", params);

    create_wav(&words, &params)?;
    Ok(())
}

