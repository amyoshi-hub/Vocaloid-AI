## Overview
This crate provides a real-time voice synthesis interface using WAV files and emotion parameters.
Designed for Raspberry Pi and AI-driven Vocaloid applications.

toml
vocaloid = "0.1.0"

## function
emotion_vocaloid()
- this is based OSAI format

vocaloid()
- util vocaloid

## Exsample
```rs
use std::error::Error;

use vocaloid;

fn main() -> Result<(), Box<dyn Error>> {
    println!("select: [1] emotion_vocaloid  [2] vocaloid");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => vocaloid::emotion_vocaloid()?,
        "2" => vocaloid::vocaloid()?,
        _ => println!("wrong number"),
    }

    Ok(())
}
```

vocaloid
```rs
pub fn vocaloid() -> Result<(), hound::Error> {
    use std::io::{self, BufRead};

    println!("input lyric:");
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap_or_else(|| Ok(String::new()))?;

    let words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let params: Vec<Vec<f32>> = vec![vec![0.0; 14]; words.len()];  // no emotion

    create_wav(&words, &params)?;
    Ok(())
}
```
vocaloid_emotion
```rs
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

    create_wav(&words, &params)?;
    Ok(())
}
```

## Lyrics format
word,param1,param2,...param14

## ⚡Please dowmload vioce dirctory
- https://github.com/amyoshi-hub/Vocaloid-AI/tree/main/vocaloid/voice
- install voice cargo.toml dir
