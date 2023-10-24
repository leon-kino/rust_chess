use crate::errs::Errs;
use crate::piece::{Board, Colors};

/// アルファベットを数字に変換する
pub fn alphabet_to_number(alph: &char) -> Result<usize, Errs> {
    match alph {
        'a'..='h' => Ok(*alph as usize - 'a' as usize),
        'A'..='H' => Ok(*alph as usize - 'A' as usize),
        _ => Err(Errs::SecondStrErr),
    }
}

/// 受け取った数字がマスの範囲内 => usizeに変換
/// 受け取った数字がマスの範囲外 => Errs::CantMoveErrを返す
pub fn is_inner_board(num: isize) -> Result<usize, Errs> {
    if num < 0 || num > 7 {
        Err(Errs::CantMoveErr)
    } else {
        Ok(num as usize)
    }
}

/// 指定された座標にある駒の色を特定する
/// 駒なし => 0
/// 同じ色の駒 => 1
/// 異なる色の駒 => 2
pub fn judge_exist(pieces: &Board, x: usize, y: usize, is_white: bool) -> usize {
    let color = solve_color(is_white);

    if pieces[y][x].color == Colors::Empty {
        0
    } else if color == pieces[y][x].color {
        1
    } else {
        2
    }
}

/// 引数がtrue => White
///     false => Black
pub fn solve_color(is_white: bool) -> Colors {
    if is_white {
        Colors::White
    } else {
        Colors::Black
    }
}
