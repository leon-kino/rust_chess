use crate::movements::pawn::pawn;
use crate::piece::Piece;
use crate::Errs;
use std::io;
use std::process;

type Board = [[Piece; 8]; 8];

/* ---------------------------------------------------------------------------------------- */
/* 入力関係                                                                                  */
/* ---------------------------------------------------------------------------------------- */

pub fn input() -> String {
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("入力に失敗しました。");
    return input_str;
}

pub fn analize_str<'a>(strs: Vec<char>, pieces: &Board, index: isize) -> Result<Board, Errs> {
    let is_white = index % 2 == 0;
    // 文字の長さを確認する (\nを含む)
    match strs.len() {
        3 => return pawn(pieces, &strs, is_white),
        4 => todo!(),
        // match strs[0] {
        //     'k' | 'K' => king(pieces, &strs, is_white),
        //     'q' | 'Q' => queen(pieces, &strs, is_white),
        //     'r' | 'R' => rook(pieces, &strs, is_white),
        //     'b' | 'B' => bishop(pieces, &strs, is_white),
        //     'n' | 'N' => knight(pieces, &strs, is_white),
        //     'o' | 'O' => キャスリング
        //     _ => Err(Errs::FirstStrErr),
        // },
        5 => todo!(),
        // match strs[0] {
        //     'k' | 'K' => king(pieces, &strs, is_white),
        //     'q' | 'Q' => queen(pieces, &strs, is_white),
        //     'r' | 'R' => rook(pieces, &strs, is_white),
        //     'b' | 'B' => bishop(pieces, &strs, is_white),
        //     'n' | 'N' => knight(pieces, &strs, is_white),
        //     'p' | 'P' => pawn(pieces, &strs, is_white),
        //     _ => Err(Errs::FirstStrErr),
        // },
        6 => todo!(),
        // o-o-o
        _ => return Err(Errs::StrsLengthErr),
    }
}

/* ---------------------------------------------------------------------------------------- */
/* 入力関係                                                                                  */
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
