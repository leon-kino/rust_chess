use crate::errs::Errs;
use crate::piece::{Board, Colors};

/// アルファベットを数字に変換する
/// ### Ok:アルファベットに対応する数字
/// ### Ng:返還に失敗した場合
/// * `alph`: 変換したいアルファベット
pub fn alphabet_to_number(alph: &char) -> Result<usize, Errs> {
    match alph {
        'a'..='h' => Ok(*alph as usize - 'a' as usize),
        'A'..='H' => Ok(*alph as usize - 'A' as usize),
        _ => Err(Errs::SecondStrErr),
    }
}

/// 引数が0以上7以下かを判断する
/// ### Ok:usizeに変換された数字
/// ### Ng:返還に失敗した場合はErrを返す
/// * `num` 変換したい数字
pub fn is_inner_board(num: isize) -> Result<usize, Errs> {
    if num < 0 || num > 7 {
        Err(Errs::CantMoveErr)
    } else {
        Ok(num as usize)
    }
}

/// 指定された座標にある駒の色を特定する
/// ### 駒なし => 0
/// ### 同じ色の駒 => 1
/// ### 異なる色の駒 => 2
/// * `pieces`: 盤面の情報
/// * `x`: x座標
/// * `y`: y座標
/// * `is_white`: 白=>true, 黒=>false
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

/// 引数で指定された色を返す
/// ### 引数がtrue => White
/// ### 引数がfalse => Black
/// * `is_white`: 白=>true, 黒=>false
pub fn solve_color(is_white: bool) -> Colors {
    if is_white {
        Colors::White
    } else {
        Colors::Black
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::init;

    use super::*;
    #[test]
    fn a_to_0() {
        assert_eq!(Ok(0), alphabet_to_number(&'a'))
    }
    #[test]
    fn b_to_1() {
        assert_eq!(Ok(1), alphabet_to_number(&'b'))
    }
    #[test]
    fn c_to_2() {
        assert_eq!(Ok(2), alphabet_to_number(&'c'))
    }
    #[test]
    fn d_to_3() {
        assert_eq!(Ok(3), alphabet_to_number(&'d'))
    }
    #[test]
    fn e_to_4() {
        assert_eq!(Ok(4), alphabet_to_number(&'e'))
    }
    #[test]
    fn f_to_5() {
        assert_eq!(Ok(5), alphabet_to_number(&'f'))
    }
    #[test]
    fn g_to_6() {
        assert_eq!(Ok(6), alphabet_to_number(&'g'))
    }
    #[test]
    fn h_to_7() {
        assert_eq!(Ok(7), alphabet_to_number(&'h'))
    }
    #[test]
    fn i_to_ng() {
        for alph in 'i'..='z' {
            assert_eq!(Err(Errs::SecondStrErr), alphabet_to_number(&alph))
        }
    }
    #[test]
    fn is_there_inner() {
        for num in 0..8 {
            assert_eq!(Ok(num as usize), is_inner_board(num))
        }
    }
    #[test]
    fn is_not_there_inner() {
        assert_eq!(Err(Errs::CantMoveErr), is_inner_board(-1));
        assert_eq!(Err(Errs::CantMoveErr), is_inner_board(8));
    }
    #[test]
    fn non_piece() {
        assert_eq!(0, judge_exist(&init(), 0, 2, true));
    }
    #[test]
    fn same_color() {
        assert_eq!(1, judge_exist(&init(), 0, 0, true));
    }
    #[test]
    fn other_color() {
        assert_eq!(2, judge_exist(&init(), 0, 6, true));
    }
    #[test]
    fn solving_color() {
        assert_eq!(Colors::White, solve_color(true));
        assert_eq!(Colors::Black, solve_color(false));
    }
}
