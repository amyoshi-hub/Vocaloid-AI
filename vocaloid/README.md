## Overview
This crate provides a real-time voice synthesis interface using WAV files and emotion parameters.
Designed for Raspberry Pi and AI-driven Vocaloid applications.

toml
vocaloid = "0.1.3"

## function
emotion_vocaloid()
- this is based OSAI format

vocaloid()
- util vocaloid

## Exsample
```rs
use std::error::Error;
// vocaloidモジュールをインポートします。
// vocaloid::vocaloidとvocaloid::emotion_vocaloid関数が存在することを想定しています。
use vocaloid;

fn main() -> Result<(), Box<dyn Error>> {
    println!("モードを選択してください: [1] emotion_vocaloid  [2] vocaloid");

    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("error on input: {}", e);
        return Err(e.into());
    }

    let trimmed_mode = input.trim();

    match trimmed_mode {
        "1" => {
            println!("emotion mode selected processing...");
            vocaloid::emotion_vocaloid()?;
        },
        "2" => {
            println!("normalmode input lyric:");
            let mut text_input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut text_input) {
                eprintln!("text error: {}", e);
                return Err(e.into());
            }
            vocaloid::vocaloid(text_input.trim())?;
        },
        _ => {
            eprintln!("error: error mode('{}')。", trimmed_mode);
            return Err("error mode".into());
        }
    }

    Ok(())
}
pub fn emotion_vocaloid() -> Result<(), hound::Error>{
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
    
    // 修正2: words (Vec<String>, 日本語の単語リスト)を
    // イテレートして、一つずつhiragana_to_romajiに渡し、RomajiのVec<String>を生成
    let roma_c: Vec<String> = words.iter()
        .map(|word| hiragana_to_romaji(word))
        .collect();

    println!("{:?}", roma_c);
    create_wav(&roma_c, &params)?;
    Ok(())

}


// 修正1: Rustの関数引数は `変数名: 型` の形式で定義します
pub fn vocaloid(text: &str) -> Result<(), hound::Error> {

    let input = hiragana_to_romaji(text);
    println!("{}", input);
    let words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let params: Vec<Vec<f32>> = vec![vec![5.0; 14]; words.len()];  // 感情パラメータ無し＝フラット

    create_wav(&words, &params)?;
    Ok(())
}


## Lyrics format
word,param1,param2,...param14

## ⚡Please dowmload vioce dirctory
- https://github.com/amyoshi-hub/Vocaloid-AI/blob/main/vocaloid/teto_voice.zip
- install voice cargo.toml dir
