pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::errs::Errs;
use crate::piece::{Board, Colors, Piece, PieceKinds};
use std::cmp::{max, min};

/// 駒を移動させる
/// エラー: 移動元として駒がない場所を指定された場合
fn move_piece(
    pieces: Board,
    from_y: usize,
    from_x: usize,
    to_y: usize,
    to_x: usize,
) -> Result<Board, Errs> {
    // 乗り越えがないかチェック
    if is_between(&pieces, from_x, from_y, to_x, to_y) {
        return Err(Errs::CantMoveErr);
    }

    // 移動させたい駒
    let mut target_piece = pieces[from_y][from_x];
    target_piece.is_moved = true;

    // 戻り値用の変数
    let mut rtn_pieces = pieces;
    // コマの移動
    rtn_pieces[to_y][to_x] = target_piece;
    // 移動元を空白に
    rtn_pieces[from_y][from_x] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
    Ok(rtn_pieces)
}

/// 移動元と移動先の間に駒がある=> ture , 駒はない=>false
fn is_between(pieces: &Board, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
    if from_x == to_x && from_y == to_y {
        panic!("論理エラー（移動先と移動元が同じ）")
    }

    // 縦方向に移動
    if from_x == to_x {
        let min = min(from_y, to_y);
        let max = max(from_y, to_y);
        for y in min + 1..max {
            if pieces[y][from_x].piece_kind != PieceKinds::Empty {
                return true;
            }
        }
        return false;
    }

    // 横方向に移動
    if from_y == to_y {
        let min = min(from_x, to_x);
        let max = max(from_x, to_x);
        for x in min + 1..max {
            if pieces[from_y][x].piece_kind != PieceKinds::Empty {
                return true;
            }
        }
        return false;
    }

    // ＼方向に移動
    if from_y as isize - from_x as isize == to_y as isize - to_x as isize {
        let min_y = min(from_y, to_y);
        let min_x = min(from_x, to_x);

        for i in 1..from_y.abs_diff(to_y) {
            if pieces[min_y + i][min_x + i].piece_kind != PieceKinds::Empty {
                return true;
            }
        }
        return false;
    }

    // ／方向に移動
    if from_x + from_y == to_x + to_y {
        if from_x > to_x {
            for i in 1..from_x - to_x {
                if pieces[from_y - i][from_x + i].piece_kind != PieceKinds::Empty {
                    return true;
                }
            }
            return false;
        } else {
            for i in 1..to_x - from_x {
                if pieces[from_y + i][from_x - i].piece_kind != PieceKinds::Empty {
                    return true;
                }
            }
            return false;
        }
    }
    return true;
}
