use crate::library::{is_inner_board, solve_color};
use crate::piece::*;

#[derive(Debug, PartialEq)]
enum CheckKinds {
    Tate,
    Yoko,
    LeftUp,
    RightUp,
    Knight,
    Pawn,
}

/// チェック状態か確認する
/// ### 戻り値:チェックの種類をまとめたリスト
/// * `pieces`: 盤の情報
/// * `is_white`: 検証したい色(白=>true, 黒=>false)
fn check_check(pieces: &Board, is_white: bool) -> Vec<CheckKinds> {
    let mut ans = Vec::with_capacity(5);
    let color = solve_color(is_white);
    let mut king_y = 99;
    let mut king_x = 99;
    // キングの位置を特定
    'outer: for i in 0..8 {
        for j in 0..8 {
            if pieces[i][j].piece_kind == PieceKinds::King {
                if pieces[i][j].color == color {
                    king_y = i;
                    king_x = j;
                    break 'outer;
                }
            }
        }
    }
    // キングを発見できなかった場合
    if king_y == 99 {
        panic!("Kingがありません");
    }

    // 縦方向を検証
    // solve_check()にて求められた回数分,ansに追加する
    for _ in 0..solve_check(
        |num| Some(pieces[num as usize][king_x]),
        PieceKinds::Rook,
        color,
    ) {
        ans.push(CheckKinds::Tate);
    }

    // 横方向を検証
    for _ in 0..solve_check(
        |num| Some(pieces[king_y][num as usize]),
        PieceKinds::Rook,
        color,
    ) {
        ans.push(CheckKinds::Yoko);
    }

    // ＼方向を検証
    // 左上
    // yかxのどちらかが0を下回れば実行しない
    for _ in 0..solve_check(
        |num| {
            if (king_y as isize) - num >= 0 && (king_x as isize) - num >= 0 {
                Some(pieces[king_y - num as usize][king_x - num as usize])
            } else {
                None
            }
        },
        PieceKinds::Bishop,
        color,
    ) {
        ans.push(CheckKinds::LeftUp);
    }
    // 右下
    // yかxのどちらかが7を上回れば実行しない
    for _ in 0..solve_check(
        |num| {
            if (king_y as isize) + num < 8 && (king_x as isize) + num < 8 {
                Some(pieces[king_y + num as usize][king_x + num as usize])
            } else {
                None
            }
        },
        PieceKinds::Bishop,
        color,
    ) {
        ans.push(CheckKinds::LeftUp);
    }

    // ／方向を検証
    // 右上
    // yが0未満 or xが8以上の場合は実行しない
    for _ in 0..solve_check(
        |num| {
            if (king_y as isize) - num >= 0 && (king_x as isize) + num < 8 {
                Some(pieces[king_y - num as usize][king_x + num as usize])
            } else {
                None
            }
        },
        PieceKinds::Bishop,
        color,
    ) {
        ans.push(CheckKinds::RightUp);
    }
    // 左下
    // xが0未満 or yが8以上の場合は実行しない
    for _ in 0..solve_check(
        |num| {
            if (king_y as isize) + num < 8 && (king_x as isize) - num >= 0 {
                Some(pieces[king_y + num as usize][king_x - num as usize])
            } else {
                None
            }
        },
        PieceKinds::Bishop,
        color,
    ) {
        ans.push(CheckKinds::RightUp);
    }
    // ナイトを検証
    for pos in 1..3 {
        for minas in 0..4 {
            let temp_y = king_y as isize - (pos * if 2 & minas == 2 { -1 } else { 1 });
            let temp_x = king_x as isize - ((3 - pos) * if 1 & minas == 1 { -1 } else { 1 });
            let y;
            let x;
            match is_inner_board(temp_y) {
                Ok(v) => y = v,
                Err(_) => continue,
            }
            match is_inner_board(temp_x) {
                Ok(v) => x = v,
                Err(_) => continue,
            }

            if pieces[y][x].piece_kind == PieceKinds::Knight {
                if pieces[y][x].color == solve_color(!is_white) {
                    ans.push(CheckKinds::Knight)
                }
            }
        }
    }

    // ポーンを検証
    let y;
    match is_inner_board(if is_white {
        king_y as isize + 1
    } else {
        king_y as isize - 1
    }) {
        Ok(v) => {
            y = v;
            match is_inner_board(king_x as isize - 1) {
                Ok(v) => {
                    let x = v;
                    if pieces[y][x].piece_kind == PieceKinds::Pawn {
                        if pieces[y][x].color == solve_color(!is_white) {
                            ans.push(CheckKinds::Pawn);
                        }
                    }
                }
                Err(_) => (),
            }
            match is_inner_board(king_x as isize + 1) {
                Ok(v) => {
                    let x = v;
                    if pieces[y][x].piece_kind == PieceKinds::Pawn {
                        if pieces[y][x].color == solve_color(!is_white) {
                            ans.push(CheckKinds::Pawn);
                        }
                    }
                }
                Err(_) => (),
            }
        }
        Err(_) => (),
    }

    return ans;
}

/// 直線のチェック確認
/// ### 戻り値:チェックされている回数
/// * `solve_piece`: 駒の求め方(横の場合はpiece\[固定\]\[ループで変更\]となる)
/// * `ng_piece_kind`: 隣接する場合、チェックとなる駒(縦横ならルーク,斜めならビショップ,クイーンは指定不要)
/// * `color`: 確認したい色
/// ### その他情報
/// 直線のチェックを確認する際、同じ処理が複数回行われていたので、本関数にまとめた
///
/// UT非対象(check_check()のテストにて本関数の正当性も証明できるため)
fn solve_check<F: Fn(isize) -> Option<Piece>>(
    solve_piece: F,
    ng_piece_kind: PieceKinds,
    color: Colors,
) -> usize {
    // 敵ルーク,敵クイーンのとき、-1にし、味方キングのとき1とする。
    // 1と-1が隣接していれば、true
    let mut state = 0;
    let mut ans = 0;
    for i in 0..8 {
        match solve_piece(i) {
            Some(v) => {
                let current_piece = v;
                // 味方駒
                if current_piece.color == color {
                    if current_piece.piece_kind == PieceKinds::King {
                        // 攻撃できる敵駒が横にある!チェック状態
                        if state == -1 {
                            ans += 1;
                        }
                        state = 1;
                    } else if current_piece.piece_kind != PieceKinds::Empty {
                        state = 0;
                    }
                // 敵駒
                } else if current_piece.color != Colors::Empty {
                    if current_piece.piece_kind == PieceKinds::Queen
                        || current_piece.piece_kind == ng_piece_kind
                    {
                        // 攻撃できる敵駒が横にある!チェック状態
                        if state == 1 {
                            ans += 1;
                        }
                        state = -1;
                    } else if current_piece.piece_kind != PieceKinds::Empty {
                        state = 0;
                    }
                }
            }
            None => (),
        }
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{self, debug_tools::init_null};

    #[test]
    fn check_ok_tate() {
        let mut pieces = init();
        // 縦列にクイーン・ルークがない場合
        assert_eq!(0, check_check(&pieces, true).len());

        // 下側にクイーンがあるが、キングとの間に他の駒がある場合
        pieces[5][4] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        assert_eq!(0, check_check(&pieces, true).len());

        pieces = init();
        // 上側にクイーンがあるが、キングとの間に他の駒がある場合
        pieces[2][4] = Piece::create_instance(PieceKinds::Queen, Colors::White);
        assert_eq!(0, check_check(&pieces, false).len());
    }

    #[test]
    fn check_ng_tate() {
        let mut pieces = init();

        // 下側にクイーンがあり、キングとの間に他の駒がない場合
        pieces[5][4] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        pieces[1][4] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
        assert_eq!(Vec::from([CheckKinds::Tate]), check_check(&pieces, true));

        pieces = init();
        // 上側にクイーンがあり、キングとの間に他の駒がない場合
        pieces[2][4] = Piece::create_instance(PieceKinds::Queen, Colors::White);
        pieces[6][4] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
        assert_eq!(Vec::from([CheckKinds::Tate]), check_check(&pieces, false));

        pieces = piece::debug_tools::init_null();
        // 上下にクイーン,ルークがあり、キングとの間に他の駒がない場合
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::Black);
        pieces[0][4] = Piece::create_instance(PieceKinds::Queen, Colors::White);
        pieces[7][4] = Piece::create_instance(PieceKinds::Rook, Colors::White);
        assert_eq!(
            Vec::from([CheckKinds::Tate, CheckKinds::Tate]),
            check_check(&pieces, false)
        );
    }

    #[test]
    fn check_ok_yoko() {
        let mut pieces = init();
        // 横列にクイーン・ルークがない場合
        assert_eq!(0, check_check(&pieces, true).len());

        // 左側にクイーンがあるが、キングとの間に他の駒がある場合
        pieces[0][0] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        assert_eq!(0, check_check(&pieces, true).len());

        pieces = init();
        // 上側にクイーンがあるが、キングとの間に他の駒がある場合
        pieces[7][0] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        assert_eq!(0, check_check(&pieces, true).len());
    }

    #[test]
    fn check_ng_yoko() {
        let mut pieces = init_null();
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::White);

        // 左側にクイーンがあり、キングとの間に他の駒がない場合
        pieces[4][0] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        assert_eq!(Vec::from([CheckKinds::Yoko]), check_check(&pieces, true));

        // 左右にクイーン,ルークがあり、キングとの間に他の駒がない場合
        pieces[4][7] = Piece::create_instance(PieceKinds::Rook, Colors::Black);
        assert_eq!(
            Vec::from([CheckKinds::Yoko, CheckKinds::Yoko]),
            check_check(&pieces, true)
        );

        // 右側にルークがあり、キングとの間に他の駒がない場合
        pieces[4][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
        assert_eq!(Vec::from([CheckKinds::Yoko]), check_check(&pieces, true));
    }

    #[test]
    fn check_ok_left_up() {
        let mut pieces = init_null();
        pieces[5][3] = Piece::create_instance(PieceKinds::King, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());

        // 左上にクイーンがあるが、キングとの間に他の駒がある場合
        pieces[2][0] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        pieces[4][2] = Piece::create_instance(PieceKinds::Knight, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());

        // 右下にビショップがあるが、キングとの間に他の駒がある場合
        pieces[2][0] = Piece::create_instance(PieceKinds::Bishop, Colors::Black);
        pieces[4][2] = Piece::create_instance(PieceKinds::Rook, Colors::Black);
        assert_eq!(0, check_check(&pieces, true).len());
    }

    #[test]
    fn check_ng_left_up() {
        let mut pieces = init_null();
        pieces[5][3] = Piece::create_instance(PieceKinds::King, Colors::White);

        // 左上にクイーンがあり、キングとの間に他の駒がない場合
        pieces[2][0] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        assert_eq!(Vec::from([CheckKinds::LeftUp]), check_check(&pieces, true));

        // 左上,右下にクイーン,ビショップがあり、キングとの間に他の駒がない場合
        pieces[7][5] = Piece::create_instance(PieceKinds::Bishop, Colors::Black);
        assert_eq!(
            Vec::from([CheckKinds::LeftUp, CheckKinds::LeftUp]),
            check_check(&pieces, true)
        );

        // 右下にビショップがあり、キングとの間に他の駒がない場合
        pieces[2][0] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
        assert_eq!(Vec::from([CheckKinds::LeftUp]), check_check(&pieces, true));
    }

    #[test]
    fn check_ok_right_up() {
        let mut pieces = init_null();
        pieces[4][5] = Piece::create_instance(PieceKinds::King, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());

        // 右上にクイーンがあるが、キングとの間に他の駒がある場合
        pieces[2][7] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        pieces[3][6] = Piece::create_instance(PieceKinds::Knight, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());

        // 左下にビショップがあるが、キングとの間に他の駒がある場合
        pieces[7][2] = Piece::create_instance(PieceKinds::Bishop, Colors::Black);
        pieces[6][3] = Piece::create_instance(PieceKinds::Rook, Colors::Black);
        assert_eq!(0, check_check(&pieces, true).len());
    }

    #[test]
    fn check_ng_right_up() {
        let mut pieces = init_null();
        pieces[4][5] = Piece::create_instance(PieceKinds::King, Colors::White);

        // 左上にクイーンがあり、キングとの間に他の駒がない場合
        pieces[2][7] = Piece::create_instance(PieceKinds::Queen, Colors::Black);
        assert_eq!(Vec::from([CheckKinds::RightUp]), check_check(&pieces, true));

        // 左上,右下にクイーン,ビショップがあり、キングとの間に他の駒がない場合
        pieces[7][2] = Piece::create_instance(PieceKinds::Bishop, Colors::Black);
        assert_eq!(
            Vec::from([CheckKinds::RightUp, CheckKinds::RightUp]),
            check_check(&pieces, true)
        );

        // 右下にビショップがあり、キングとの間に他の駒がない場合
        pieces[2][7] = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
        assert_eq!(Vec::from([CheckKinds::RightUp]), check_check(&pieces, true));
    }
    #[test]
    fn check_ok_knight() {
        let mut pieces = init_null();
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());
    }
    #[test]
    fn check_ng_knight() {
        let mut pieces = init_null();
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::White);

        let knight = Piece::create_instance(PieceKinds::Knight, Colors::Black);
        // ナイトがどの位置からでもチェックできることを確認
        pieces[3][2] = knight.clone();
        pieces[2][3] = knight.clone();
        pieces[3][6] = knight.clone();
        pieces[2][5] = knight.clone();
        pieces[5][2] = knight.clone();
        pieces[6][3] = knight.clone();
        pieces[5][6] = knight.clone();
        pieces[6][5] = knight.clone();
        assert_eq!(
            Vec::from([
                CheckKinds::Knight,
                CheckKinds::Knight,
                CheckKinds::Knight,
                CheckKinds::Knight,
                CheckKinds::Knight,
                CheckKinds::Knight,
                CheckKinds::Knight,
                CheckKinds::Knight,
            ]),
            check_check(&pieces, true)
        );

        // 範囲外を探索させ、クラッシュしないか確認
        pieces = init_null();
        pieces[7][7] = Piece::create_instance(PieceKinds::King, Colors::White);
        pieces[5][6] = knight.clone();
        pieces[6][5] = knight;
        assert_eq!(
            Vec::from([CheckKinds::Knight, CheckKinds::Knight]),
            check_check(&pieces, true)
        );
    }

    #[test]
    fn check_ok_pawn() {
        let mut pieces = init_null();
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());

        // 範囲外を参照しないか確認
        let mut pieces = init_null();
        pieces[0][0] = Piece::create_instance(PieceKinds::King, Colors::Black);
        assert_eq!(0, check_check(&pieces, false).len());
        let mut pieces = init_null();
        pieces[7][7] = Piece::create_instance(PieceKinds::King, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());

        let mut pieces = init_null();
        pieces[4][0] = Piece::create_instance(PieceKinds::King, Colors::Black);
        assert_eq!(0, check_check(&pieces, false).len());
        let mut pieces = init_null();
        pieces[4][7] = Piece::create_instance(PieceKinds::King, Colors::White);
        assert_eq!(0, check_check(&pieces, true).len());
    }
    #[test]
    fn check_ng_pawn() {
        let mut pieces = init_null();
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::White);
        pieces[5][3] = Piece::create_instance(PieceKinds::Pawn, Colors::Black);
        pieces[5][5] = Piece::create_instance(PieceKinds::Pawn, Colors::Black);
        assert_eq!(
            Vec::from([CheckKinds::Pawn, CheckKinds::Pawn]),
            check_check(&pieces, true)
        );
        let mut pieces = init_null();
        pieces[4][4] = Piece::create_instance(PieceKinds::King, Colors::Black);
        pieces[3][3] = Piece::create_instance(PieceKinds::Pawn, Colors::White);
        pieces[3][5] = Piece::create_instance(PieceKinds::Pawn, Colors::White);
        assert_eq!(
            Vec::from([CheckKinds::Pawn, CheckKinds::Pawn]),
            check_check(&pieces, false)
        )
    }
}
