use std::error::Error;

use vocaloid;

fn main() -> Result<(), Box<dyn Error>> {
    println!("モードを選択してください: [1] emotion_vocaloid  [2] vocaloid");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => vocaloid::emotion_vocaloid()?,
        "2" => vocaloid::vocaloid()?,
        _ => println!("不正な入力です。1か2を選んでください。"),
    }

    Ok(())
}

