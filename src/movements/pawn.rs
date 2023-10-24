use crate::errs::Errs;
use crate::i_o;
use crate::library::*;
use crate::piece::{Board, Piece, PieceKinds};

use super::move_piece;

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
