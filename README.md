# Vocaloid-AI

ボカロ　編集ソフトの超簡易版作りました
音声はテトを使っていますが、他のボカロに差し替えてもファイル名の処理をすれば動きます


## HOW_TO]_use:
- cargo run から標準入力からローマ字を入れればoutput.wavが作成されますからそれをaplayなどで再生

## 他のボカロに差し替え
- sudo apt install kakasi
- rootプロジェクト内のromaディレクトリにあるソースコードをご自分のボカロwav_dirに差し替え
- それからrename.shのようなもので整える

