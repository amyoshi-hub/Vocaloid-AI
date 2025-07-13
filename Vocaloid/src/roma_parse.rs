fn split_romaji(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let rest = &chars[i..];

        // 3文字音節
        if rest.len() >= 3 {
            let tri = rest[0..3].iter().collect::<String>();
            if ["kyo", "sha", "cho", "nyu", "ryo", "pyo"].contains(&tri.as_str()) {
                result.push(tri);
                i += 3;
                continue;
            }
        }

        // 2文字音節
        if rest.len() >= 2 {
            let bi = rest[0..2].iter().collect::<String>();
            if ["ka", "ki", "ku", "ke", "ko", "shi", "tsu", "chi", "no", "ne", "na", "ni", "nu", "ra", "re", "ri", "ru", "ro", "yo", "ya", "yu", "wa"].contains(&bi.as_str()) {
                result.push(bi);
                i += 2;
                continue;
            }
        }

        // 1文字音節（n など）
        result.push(chars[i].to_string());
        i += 1;
    }

    result
}

