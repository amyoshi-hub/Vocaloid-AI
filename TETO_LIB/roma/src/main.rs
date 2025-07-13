use std::fs;
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::io::Write;

fn main() {
    let parent_dir = "../TETO_alone";
    for entry in fs::read_dir(parent_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let fname = path.file_name().unwrap().to_string_lossy();

        if let Some((stem, ext)) = fname.rsplit_once('.') {
            // kakasiによる変換
            let mut child = Command::new("kakasi")
                .args(["-Ja", "-Ha", "-Ka", "-Ea", "-iutf8", "-outf8", "-s"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to start kakasi");

            {
                let stdin = child.stdin.as_mut().expect("failed to open stdin");
                write!(stdin, "{}", stem).expect("failed to write to kakasi");
            }

            let output = child.wait_with_output().expect("failed to read stdout");
            let romaji = String::from_utf8_lossy(&output.stdout).trim().replace(" ", "_");

            // 新しいパスを同じディレクトリに作る
            let mut new_path = PathBuf::from(&path);
            new_path.set_file_name(format!("{}.{}", romaji, ext));

            println!("Renaming: {} → {}", path.display(), new_path.display());

            if new_path.exists() {
                eprintln!("⚠️ Skipped: {} already exists", new_path.display());
            } else {
                fs::rename(&path, &new_path).expect("rename failed");
            }
        }
    }
}

