// --- 日本語 to Romaji 変換ヘルパー関数 ---
pub fn hiragana_to_romaji(hiragana: &str) -> String {
    let mut romaji = String::new();
    // 全て小文字に統一して処理をシンプルにする
    let lower_input = hiragana.to_lowercase();
    let chars: Vec<char> = lower_input.chars().collect();
    let mut i = 0;

    // 基本的なひらがな to ローマ字マッピング (拗音も含む)
    // 要素数を110から119に修正
    let kana_map: [(&str, &str); 119] = [
        // 50音 (CV)
        ("あ", "a"), ("い", "i"), ("う", "u"), ("え", "e"), ("お", "o"),
        ("か", "ka"), ("き", "ki"), ("く", "ku"), ("け", "ke"), ("こ", "ko"),
        ("さ", "sa"), ("し", "shi"), ("す", "su"), ("せ", "se"), ("そ", "so"),
        ("た", "ta"), ("ち", "chi"), ("つ", "tsu"), ("て", "te"), ("と", "to"),
        ("な", "na"), ("に", "ni"), ("ぬ", "nu"), ("ね", "ne"), ("の", "no"),
        ("は", "ha"), ("ひ", "hi"), ("ふ", "fu"), ("へ", "he"), ("ほ", "ho"),
        ("ま", "ma"), ("み", "mi"), ("む", "mu"), ("め", "me"), ("も", "mo"),
        ("や", "ya"), ("ゆ", "yu"), ("よ", "yo"),
        ("ら", "ra"), ("り", "ri"), ("る", "ru"), ("れ", "re"), ("ろ", "ro"),
        ("わ", "wa"), ("を", "wo"), ("ん", "n"),

        // 濁音 (GV)
        ("が", "ga"), ("ぎ", "gi"), ("ぐ", "gu"), ("げ", "ge"), ("ご", "go"),
        ("ざ", "za"), ("じ", "ji"), ("ず", "zu"), ("ぜ", "ze"), ("ぞ", "zo"),
        ("だ", "da"), ("ぢ", "ji"), ("づ", "zu"), ("で", "de"), ("ど", "do"),
        ("ば", "ba"), ("び", "bi"), ("ぶ", "bu"), ("べ", "be"), ("ぼ", "bo"),
                
        // 半濁音 (PV)
        ("ぱ", "pa"), ("ぴ", "pi"), ("ぷ", "pu"), ("ぺ", "pe"), ("ぽ", "po"),
                
        // 拗音 (CYV) - 2文字で処理
        ("きゃ", "kya"), ("きゅ", "kyu"), ("きょ", "kyo"),
        ("しゃ", "sha"), ("しゅ", "shu"), ("しょ", "sho"),
        ("ちゃ", "cha"), ("ちゅ", "chu"), ("ちょ", "cho"),
        ("にゃ", "nya"), ("にゅ", "nyu"), ("にょ", "nyo"),
        ("ひゃ", "hya"), ("ひゅ", "hyu"), ("ひょ", "hyo"),
        ("みゃ", "mya"), ("みゅ", "myu"), ("みょ", "myo"),
        ("りゃ", "rya"), ("りゅ", "ryu"), ("りょ", "ro"), // 注: ユーザーコードのまま "りょ" -> "ro" は維持

        // 拗音 (GVY) - 2文字で処理
        ("ぎゃ", "gya"), ("ぎゅ", "gyu"), ("ぎょ", "gyo"),
        ("じゃ", "ja"), ("じゅ", "ju"), ("じょ", "jo"),
        ("びゃ", "bya"), ("びゅ", "byu"), ("びょ", "byo"),
                
        // 拗音 (PVY) - 2文字で処理
        ("ぴゃ", "pya"), ("ぴゅ", "pyu"), ("ぴょ", "pyo"),

        // 捨て仮名 (小文字のぁぃぅぇぉゃゅょゎはここでは使用しない。全て大文字に置き換える。)
        ("ぁ", "a"), ("ぃ", "i"), ("ぅ", "u"), ("ぇ", "e"), ("ぉ", "o"),
                
        // 句読点とスペース、長音記号
        ("。", ""), ("、", ""), ("ー", "-"), (" ", " "),
        ("!", ""), ("?", ""),
                
        // 特別な発音
        ("ふぁ", "fa"), ("ふぃ", "fi"), ("ふぇ", "fe"), ("ふぉ", "fo"),
    ];

    // カタカナを対応するひらがなに変換する
    let to_hiragana = |c: char| -> char {
        // カタカナ(U+30A0)とひらがな(U+3040)のオフセットは0x60
        if c >= 'ァ' && c <= 'ヶ' {
            (c as u32 - 0x60) as u8 as char
        } else {
            c
        }
    };

    while i < chars.len() {
        // 現在の文字をひらがな化
        let current_char = to_hiragana(chars[i]);

        // 1. 拗音チェック (2文字先読み)
        if i + 1 < chars.len() {
            let next_char = to_hiragana(chars[i+1]);
            let mut two_chars = String::new();
            two_chars.push(current_char);
            two_chars.push(next_char);
                        
            // 拗音（2文字）をチェック
            if let Some((_, romaji_str)) = kana_map.iter().find(|(k, _)| **k == two_chars) {
                romaji.push_str(romaji_str);
                i += 2;
                continue;
            }
        }
                
        // 2. 促音チェック (っ)
        if current_char == 'っ' || current_char == 'ッ' {
            if i + 1 < chars.len() {
                // 次の文字をチェックし、そのローマ字の最初の子音を複製する
                let next_char = to_hiragana(chars[i+1]);
                let next_kana_str = next_char.to_string();
                                
                // 次の1文字のローマ字マッピングを見つける
                if let Some((_, romaji_str)) = kana_map.iter().find(|(k, _)| *k == next_kana_str) {
                    // 最初の文字（子音）を複製
                    if let Some(first_char) = romaji_str.chars().next() {
                         // 母音でなければ複製
                         if !['a', 'i', 'u', 'e', 'o'].contains(&first_char) {
                             romaji.push(first_char);
                         }
                    }
                }
            }
            i += 1;
            continue;
        }

        // 3. 1文字音節チェック (CV, V, N, 句読点, スペース)
        let current_kana_str = current_char.to_string();
        if let Some((_, romaji_str)) = kana_map.iter().find(|(k, _)| **k == current_kana_str) {
            romaji.push_str(romaji_str);
            i += 1;
            continue;
        }

        // 4. マッチしなかった文字はスキップ (漢字など)
        i += 1;
    }

    // 長音処理: '-' の直前の母音を重ねる (例: ra-men -> raamen)
    let processed_romaji = romaji.replace("a-", "aa")
                                 .replace("i-", "ii")
                                 .replace("u-", "uu")
                                 .replace("e-", "ee")
                                 .replace("o-", "oo");
        
    // スペースとアルファベットのみ残し、その他の記号（変換しきれなかったもの）はスキップ
    processed_romaji
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
        .collect::<String>()
}


// --- 既存の Romaji 音節分割関数 (split_romaji) ---
pub fn split_romaji(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let lower_input = input.to_lowercase();
    let chars: Vec<char> = lower_input.chars().collect();
    let mut i = 0;

    // UTAUシステムで一般的に使われる日本語の音節リスト
    let two_syllables = [
        "a", "i", "u", "e", "o", // 母音 (len=1)
        "ka", "ki", "ku", "ke", "ko",
        "sa", "shi", "su", "se", "so", // "shi" は len=3
        "ta", "chi", "tsu", "te", "to", // "chi", "tsu" は len=3
        "na", "ni", "nu", "ne", "no",
        "ha", "hi", "fu", "he", "ho", // "fu" は len=2
        "ma", "mi", "mu", "me", "mo",
        "ya", "yu", "yo", // Y行
        "ra", "ri", "ru", "re", "ro",
        "wa", "wo", 
        // 濁音
        "ga", "gi", "gu", "ge", "go",
        "za", "ji", "zu", "ze", "zo", // "ji" は len=2
        "da", "de", "do",
        "ba", "bi", "bu", "be", "bo",
        // 半濁音
        "pa", "pi", "pu", "pe", "po",
        // 外来音
        "fa", "fi", "fe", "fo", "fu", // "fa", "fi", "fe", "fo" は len=2, "fu" は len=2
        "va", "vi", "vu", "ve", "vo",
    ];
    
    // 拗音（3文字）リスト
    let three_syllables = [
        "kya", "kyu", "kyo", "sha", "shu", "sho", "cha", "chu", "cho", 
        "nya", "nyu", "nyo", "hya", "hyu", "hyo", "mya", "myu", "myo",
        "rya", "ryu", "ryo", "gya", "gyu", "gyo", "ja", "ju", "jo",
        "bya", "byu", "byo", "pya", "pyu", "pyo", "she", "che", "tsa", 
        "tsi", "tse", "tso", "fya", "fyu", "fyo" 
    ];

    // すべての有効な音節を結合 (最長一致のために使用)
    let all_syllables: Vec<&str> = two_syllables.iter()
        .chain(three_syllables.iter())
        .cloned()
        .collect();

    while i < chars.len() {
        let rest = &chars[i..];
        let current_char = rest[0];

        // 0. 非Romaji文字のスキップ (主にスペースをスキップする)
        if !current_char.is_ascii_alphabetic() {
            i += 1;
            continue;
        }

        // 1. 促音（っ, Double Consonant）の処理
        // (次の文字が同じ子音の場合に、子音を音節として追加)
        if rest.len() >= 2 {
            let next_char = rest[1];
            // 連続する子音をチェック (n以外, nは撥音)
            if next_char.is_ascii_alphabetic() && current_char == next_char && current_char != 'n' {
                // 促音として、最初の子音のみを音節として追加 (例: "k"atte -> 'k')
                result.push(current_char.to_string());
                i += 1; // 1文字進めて、次の文字から再度解析を開始
                continue;
            }
        }
        
        // --- 修正された音節マッチング: 長いものから順にチェック (Greedy Matching) ---

        // 2. 3文字音節のマッチング (例: kya, chi, tsu, sha)
        if rest.len() >= 3 {
            let tri: String = rest[0..3].iter().collect();
            if all_syllables.contains(&tri.as_str()) {
                result.push(tri);
                i += 3;
                continue;
            }
        }
        
        // 3. 2文字音節のマッチング (例: ka, ni, ha, pa)
        if rest.len() >= 2 {
            let bi: String = rest[0..2].iter().collect();
            if all_syllables.contains(&bi.as_str()) {
                result.push(bi);
                i += 2;
                continue;
            }
        }

        // 4. 1文字音節のマッチング (撥音 'n', 母音: a, i, u, e, o)
        let single_syllable = current_char.to_string();
        if all_syllables.contains(&single_syllable.as_str()) {
            result.push(single_syllable);
            i += 1;
            continue;
        }
        // 'n'の単独処理はall_syllablesに"n"が含まれているため不要になりました。


        // 5. どの音節にもマッチしなかった場合は警告を出して1文字進める
        eprintln!("Warning: Unrecognized Romaji character sequence at index {}: {}", i, current_char);
        result.push(current_char.to_string());
        i += 1;
    }

    result
}
