/// 盤面の型定義
pub type Board = [[Piece; 8]; 8];

/// 駒の種類・色・移動したかの情報を持つ構造体
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub piece_kind: PieceKinds, // 駒の種類
    pub color: Colors,          // 駒の色
    pub is_moved: bool,         // まだ1回も移動していない=>false , 既に移動した=>true
}

impl Piece {
    /// インスタンスを生成する
    pub fn create_instance(piece_kind: PieceKinds, color: Colors) -> Piece {
        Piece {
            piece_kind,
            color,
            is_moved: false,
        }
    }

    /// 駒の情報から文字列を作成し、返す
    pub fn print(&self) -> String {
        let color_str = match self.color {
            Colors::White => "W".to_string(),
            Colors::Black => "B".to_string(),
            Colors::Empty => " ".to_string(),
        };
        return match self.piece_kind {
            PieceKinds::King => color_str + "K",
            PieceKinds::Queen => color_str + "Q",
            PieceKinds::Rook => color_str + "R",
            PieceKinds::Bishop => color_str + "B",
            PieceKinds::Knight => color_str + "N",
            PieceKinds::Pawn => color_str + "P",
            PieceKinds::Empty => color_str + " ",
        };
    }
}

/// 駒の種類
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKinds {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

/// 色の種類
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Colors {
    White,
    Black,
    Empty,
}

/// 初期の盤面を作成し、戻り値として返す
/// ### チェスの初期状態の盤情報
pub fn init() -> Board {
    let bk = Piece::create_instance(PieceKinds::King, Colors::Black);
    let bq = Piece::create_instance(PieceKinds::Queen, Colors::Black);
    let br = Piece::create_instance(PieceKinds::Rook, Colors::Black);
    let bb = Piece::create_instance(PieceKinds::Bishop, Colors::Black);
    let bn = Piece::create_instance(PieceKinds::Knight, Colors::Black);
    let bp = Piece::create_instance(PieceKinds::Pawn, Colors::Black);
    let wk = Piece::create_instance(PieceKinds::King, Colors::White);
    let wq = Piece::create_instance(PieceKinds::Queen, Colors::White);
    let wr = Piece::create_instance(PieceKinds::Rook, Colors::White);
    let wb = Piece::create_instance(PieceKinds::Bishop, Colors::White);
    let wn = Piece::create_instance(PieceKinds::Knight, Colors::White);
    let wp = Piece::create_instance(PieceKinds::Pawn, Colors::White);
    let empty = Piece::create_instance(PieceKinds::Empty, Colors::Empty);
    [
        [wr.clone(), wn.clone(), wb.clone(), wq, wk, wb, wn, wr],
        [
            wp.clone(),
            wp.clone(),
            wp.clone(),
            wp.clone(),
            wp.clone(),
            wp.clone(),
            wp.clone(),
            wp,
        ],
        [empty, empty, empty, empty, empty, empty, empty, empty],
        [empty, empty, empty, empty, empty, empty, empty, empty],
        [empty, empty, empty, empty, empty, empty, empty, empty],
        [empty, empty, empty, empty, empty, empty, empty, empty],
        [
            bp.clone(),
            bp.clone(),
            bp.clone(),
            bp.clone(),
            bp.clone(),
            bp.clone(),
            bp.clone(),
            bp,
        ],
        [br.clone(), bn.clone(), bb.clone(), bq, bk, bb, bn, br],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance() {
        let king = Piece::create_instance(PieceKinds::King, Colors::Black);
        assert_eq!(PieceKinds::King, king.piece_kind);
        assert_eq!(Colors::Black, king.color);
        assert_eq!(false, king.is_moved);
    }

    // 完全一致かの比較方法がわからないため保留(a == aを補償したい)
    // /// Empty以外は重複していないことを確認
    // #[test]
    // fn not_multiple() {
    //     let piece = init();
    //     for y in 0..9 {
    //         for x in 0..9 {
    //             for y2 in y..9 {
    //                 for x2 in (x + 1)..9 {
    //                     if piece[y][x].piece_kind != PieceKinds::Empty {
    //                         if std::cmp::Eq(&piece[y][x], &piece[y2][x2]) {
    //                             if !(y == y2 && x == x2) {
    //                                 panic!("同じ駒が出現 y:{}x:{} y2:{}x2:{}", y, x, y2, x2)
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
