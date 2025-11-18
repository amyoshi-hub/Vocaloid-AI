use std::error::Error;
// vocaloidモジュールをインポートします。
// vocaloid::vocaloidとvocaloid::emotion_vocaloid関数が存在することを想定しています。
use vocaloid;

fn main() -> Result<(), Box<dyn Error>> {
    // ユーザーにモード選択を促すプロンプトを表示します。
    // [1] 感情表現ボーカロイド  [2] 通常ボーカロイド
    println!("モードを選択してください: [1] emotion_vocaloid  [2] vocaloid");

    let mut input = String::new();
    // ユーザーからの入力を読み取ります。
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("入力の読み取り中にエラーが発生しました: {}", e);
        return Err(e.into());
    }

    // 入力文字列の先頭と末尾の空白（改行を含む）を削除します。
    let trimmed_mode = input.trim();

    match trimmed_mode {
        // "1"が入力された場合、感情表現ボーカロイドの関数を呼び出します。
        "1" => {
            println!("感情表現モードを選択しました。処理を開始します...");
            vocaloid::emotion_vocaloid()?;
        },
        // "2"が入力された場合、通常ボーカロイドのテキスト入力を促します。
        "2" => {
            println!("通常モードを選択しました。歌わせたいテキストを入力してください:");
            let mut text_input = String::new();
            // テキスト入力を読み取ります。
            if let Err(e) = std::io::stdin().read_line(&mut text_input) {
                eprintln!("テキストの読み取り中にエラーが発生しました: {}", e);
                return Err(e.into());
            }
            // 改行を削除したクリーンなテキストをvocaloid関数に渡します。
            vocaloid::vocaloid(text_input.trim())?;
        },
        // "1"も"2"も入力されなかった場合、無効な入力としてエラーを返します。
        _ => {
            eprintln!("エラー: 無効なモード選択です ('{}')。", trimmed_mode);
            return Err("無効なモード選択".into());
        }
    }

    // 成功を示す
    Ok(())
}
