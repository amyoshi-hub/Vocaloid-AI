use hound::{WavReader, WavWriter, Result};

fn createWav(input_string: &str) -> Result<()> {
    let mut input_files = Vec::new();
    for ch in input_string.chars() {
        if(ch == " "){
            //少し空白
        }else if(ch == "-"){
            input_files.push(format!("voice/__{}.wav", ch)); 
        }else{
            //wavを少し短く 
            input_files.push(format!("voice/__{}.wav", ch)); 
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
    let input_string = "aiueo";
    createWav(input_string)?; 
    Ok(())
}
