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
/// ### Ok : 移動後の駒
/// ### Err: 移動元と移動先の間に駒があった場合
/// * `pieces`: 駒の配置
/// * `from_y`: 移動元のy座標
/// * `from_x`: 移動元のx座標
/// * `to_y`: 移動先のy座標
/// * `to_x`: 移動先のx座標
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

/// ### 移動元と移動先の間に駒がある=> ture , 駒はない=>false
/// * `pieces`: 駒の配置
/// * `from_y`: 移動元のy座標
/// * `from_x`: 移動元のx座標
/// * `to_y`: 移動先のy座標
/// * `to_x`: 移動先のx座標
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
                if pieces[from_y + i][from_x - i].piece_kind != PieceKinds::Empty {
                    return true;
                }
            }
            return false;
        } else {
            for i in 1..to_x - from_x {
                if pieces[from_y - i][from_x + i].piece_kind != PieceKinds::Empty {
                    return true;
                }
            }
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::piece::init;

    use super::*;
    use crate::piece::Piece;
    #[test]
    fn move_test() {
        let solve = move_piece(init(), 1, 0, 3, 0);
        let mut ans = init();
        // 移動先と移動元の駒の種類が等しいこと
        ans[3][0] = ans[1][0];
        // 移動先の駒はis_movedがtrueになっていること
        ans[3][0].is_moved = true;
        // 移動元の駒はEmptyになっていること
        ans[1][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);

        assert_eq!(Ok(ans), solve);
    }
    #[test]
    fn tate_ok() {
        let mut pieces = init();
        pieces[2][0] = Piece::create_instance(PieceKinds::Rook, Colors::White);
        assert_eq!(false, is_between(&pieces, 0, 2, 5, 2));
    }
    #[test]
    fn tate_ng() {
        let mut pieces = init();
        pieces[2][0] = Piece::create_instance(PieceKinds::Rook, Colors::White);
        pieces[2][3] = Piece::create_instance(PieceKinds::Pawn, Colors::White);
        assert_eq!(true, is_between(&pieces, 0, 2, 5, 2));
    }
    #[test]
    fn yoko_ok() {
        let mut pieces = init();
        pieces[2][0] = Piece::create_instance(PieceKinds::Rook, Colors::White);
        assert_eq!(false, is_between(&pieces, 0, 2, 0, 5));
    }
    #[test]
    fn yoko_ng() {
        let mut pieces = init();
        pieces[2][0] = Piece::create_instance(PieceKinds::Rook, Colors::White);
        pieces[4][0] = Piece::create_instance(PieceKinds::Pawn, Colors::White);
        assert_eq!(true, is_between(&pieces, 0, 2, 0, 5));
    }
    #[test]
    fn left_up_ok() {
        let mut pieces = init();
        pieces[2][0] = Piece::create_instance(PieceKinds::Bishop, Colors::White);
        assert_eq!(false, is_between(&pieces, 0, 2, 2, 4));
    }
    #[test]
    fn left_up_ng() {
        let mut pieces = init();
        pieces[2][0] = Piece::create_instance(PieceKinds::Bishop, Colors::White);
        pieces[3][1] = Piece::create_instance(PieceKinds::Pawn, Colors::White);
        assert_eq!(true, is_between(&pieces, 0, 2, 2, 4));
    }
    #[test]
    fn right_up_ok() {
        let mut pieces = init();
        pieces[5][0] = Piece::create_instance(PieceKinds::Bishop, Colors::White);
        assert_eq!(false, is_between(&pieces, 0, 5, 3, 2));
    }
    #[test]
    fn right_up_ng() {
        let mut pieces = init();
        pieces[5][0] = Piece::create_instance(PieceKinds::Bishop, Colors::White);
        pieces[4][1] = Piece::create_instance(PieceKinds::Pawn, Colors::White);
        assert_eq!(true, is_between(&pieces, 0, 5, 3, 2));
    }
}
