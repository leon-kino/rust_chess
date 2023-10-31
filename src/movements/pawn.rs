use crate::errs::Errs;
use crate::i_o;
use crate::library::*;
use crate::piece::{Board, Piece, PieceKinds};

use super::move_piece;

/// ポーンの移動を行う
/// * `pieces` 駒の配置を記憶した配列
/// * `strs` 駒の配置を記憶した配列
pub fn pawn<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
    // 文字列の検証
    let x = alphabet_to_number(&strs[0])?;
    let y = strs[1] as isize - '1' as isize;
    // yが正しいかの検証
    if y < 0 || y > 7 {
        return Err(Errs::SecondStrErr);
    }

    // 1マス進む or 初期値で２マス進む
    let piece_adjust = if is_white { -1 } else { 1 };

    if judge_exist(pieces, x, y as usize, is_white) == 0 {
        for i in 1..3 {
            let piece_y = is_inner_board(y + piece_adjust * i)?;

            if pieces[piece_y][x].piece_kind == PieceKinds::Pawn {
                if pieces[piece_y][x].color == solve_color(is_white) {
                    // ２マス動く場合の処理
                    if !(i == 2 && pieces[piece_y][x].is_moved) {
                        // 駒を移動させる
                        let mut rtn = move_piece(*pieces, piece_y, x, y as usize, x)?;

                        // プロモーション
                        if y == 0 || y == 7 {
                            let mut replace_piece;
                            let mut input_str;
                            loop {
                                println!("プロモーションする駒を選択してください。");
                                // 入力
                                input_str = i_o::input();

                                match &input_str[..] {
                                    "Q" | "q" | "Queen" | "queen" | "QUEEN" => {
                                        replace_piece = Piece::create_instance(
                                            PieceKinds::Queen,
                                            solve_color(is_white),
                                        );
                                        break;
                                    }
                                    "R" | "r" | "Rook" | "rook" | "ROOK" => {
                                        replace_piece = Piece::create_instance(
                                            PieceKinds::Rook,
                                            solve_color(is_white),
                                        );
                                        break;
                                    }
                                    "B" | "b" | "Bishop" | "bishop" | "BISHOP" => {
                                        replace_piece = Piece::create_instance(
                                            PieceKinds::Bishop,
                                            solve_color(is_white),
                                        );
                                        break;
                                    }
                                    "N" | "n" | "Knight" | "knight" | "KNIGHT" => {
                                        replace_piece = Piece::create_instance(
                                            PieceKinds::Knight,
                                            solve_color(is_white),
                                        );
                                        break;
                                    }
                                    _ => println!("コマの種類を選択してください"),
                                }
                            }
                            replace_piece.is_moved = true;

                            rtn[y as usize][x] = replace_piece;
                        }
                        return Ok(rtn);
                    }
                }
            }
        }
        return Err(Errs::CantMoveErr);
    } else {
        return Err(Errs::CantMoveErr);
    }
}

#[cfg(test)]
mod pawn_tests {
    use super::*;
    use crate::piece::*;

    /// １マス進めるかのテスト
    #[test]
    fn fowerd_test() {
        let pieces = pawn(&init(), &Vec::from(['a', '3']), true);
        let mut ans = init();
        // 駒が移動先にあること
        ans[2][0] = Piece {
            piece_kind: PieceKinds::Pawn,
            color: Colors::White,
            is_moved: true,
        };

        // 移動元のコマが消えていること
        ans[1][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);

        assert_eq!(Ok(ans), pieces);
    }

    /// 2マス進めるかテスト
    #[test]
    fn fowerd_test2() {
        let pieces = pawn(&init(), &Vec::from(['a', '4']), true);
        let mut ans = init();
        // 駒が移動先にあること
        ans[3][0] = Piece {
            piece_kind: PieceKinds::Pawn,
            color: Colors::White,
            is_moved: true,
        };

        // 移動元のコマが消えていること
        ans[1][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);

        assert_eq!(Ok(ans), pieces);
    }

    /// 黒が１マス進めるかのテスト
    #[test]
    fn fowerd_test_b() {
        let pieces = pawn(&init(), &Vec::from(['a', '6']), false);
        let mut ans = init();
        // 駒が移動先にあること
        ans[5][0] = Piece {
            piece_kind: PieceKinds::Pawn,
            color: Colors::Black,
            is_moved: true,
        };

        // 移動元のコマが消えていること
        ans[6][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);

        assert_eq!(Ok(ans), pieces);
    }

    /// 2マス進めるかテスト
    #[test]
    fn fowerd_test2_b() {
        let pieces = pawn(&init(), &Vec::from(['a', '5']), false);
        let mut ans = init();
        // 駒が移動先にあること
        ans[4][0] = Piece {
            piece_kind: PieceKinds::Pawn,
            color: Colors::Black,
            is_moved: true,
        };

        // 移動元のコマが消えていること
        ans[6][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);

        assert_eq!(Ok(ans), pieces);
    }

    /// 1マス先にコマがある場合(NG1)
    #[test]
    fn ng1_piece_exist() {
        let mut board = init();

        // 移動先に駒がある
        board[2][0] = Piece {
            piece_kind: PieceKinds::Pawn,
            color: Colors::Black,
            is_moved: true,
        };
        let pieces = pawn(&board, &Vec::from(['a', '3']), true);

        assert_eq!(Err(Errs::CantMoveErr), pieces);
    }

    /// 2マス進む時に1マス先にコマがある場合(NG2)
    #[test]
    fn ng2_piece_exist_2() {
        let mut board = init();

        // 移動先に駒がある
        board[2][0] = Piece {
            piece_kind: PieceKinds::Pawn,
            color: Colors::Black,
            is_moved: true,
        };
        let pieces = pawn(&board, &Vec::from(['a', '4']), true);

        assert_eq!(Err(Errs::CantMoveErr), pieces);
    }

    /// ３マス以上の場所が指定された場合
    #[test]
    fn ng3_over() {
        let pieces = pawn(&init(), &Vec::from(['a', '5']), true);

        assert_eq!(Err(Errs::CantMoveErr), pieces);
    }

    /// 0マス以下の場所が指定された場合
    #[test]
    fn ng4_0() {
        let pieces = pawn(&init(), &Vec::from(['a', '2']), false);

        assert_eq!(Err(Errs::CantMoveErr), pieces);
    }

    /// 存在しないマスを指定された場合
    #[test]
    fn ng5_num() {
        let pieces = pawn(&init(), &Vec::from(['a', '9']), false);

        assert_eq!(Err(Errs::SecondStrErr), pieces);
    }
}
