use crate::movements::pawn::pawn;
use crate::movements::pawn::xpawn;
use crate::piece::Piece;
use crate::Errs;
use std::io;
use std::process;

type Board = [[Piece; 8]; 8];

/* ---------------------------------------------------------------------------------------- */
/* 入力関係                                                                                  */
/* ---------------------------------------------------------------------------------------- */

/// 入力を受け、文字列を返す
/// ### 入力された文字列
pub fn input() -> String {
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("入力に失敗しました。");
    return input_str;
}

/// 受け取った文字列を解析し、適切な関数に処理を振り分ける
/// ### 戻り値:
/// ### Ok:移動処理の結果
/// ### Ng:移動できなかった理由
/// * `strs`: 入力文字列
/// * `pieces`: 盤面
/// * `index` : ターン数
pub fn analize_str<'a>(strs: Vec<char>, pieces: &Board, index: isize) -> Result<Board, Errs> {
    let is_white = index % 2 == 0;
    // 先頭文字を確認する (\nを含む)
    match strs[0] {
        'a'..='h' => {
            if strs[1] == 'x' {
                return xpawn(pieces, &strs, is_white);
            }
            return pawn(pieces, &strs, is_white);
        }
        _ => return Err(Errs::FirstStrErr),
    }
}

/* ---------------------------------------------------------------------------------------- */
/* 出力関係                                                                                  */
/* ---------------------------------------------------------------------------------------- */

/// 盤面の情報を表示する
/// * `pieces` - 表示したい盤面の情報
pub fn show_board(pieces: &Board) {
    clear_sclean();
    for line in pieces {
        println!("-------------------------");
        line.iter().for_each(|x| print!("|{}", x.print()));
        println!("|");
    }

    println!("-------------------------");
}

/// 画面の消去を行う
fn clear_sclean() {
    process::Command::new("clear")
        .arg("linux")
        .stdout(process::Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
