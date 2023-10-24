/// 盤面の型定義
pub type Board = [[Piece; 8]; 8];

/// 駒の種類・色・移動したかの情報を持つ構造体
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, PartialEq)]
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
#[derive(Clone, Copy, PartialEq)]
pub enum Colors {
    White,
    Black,
    Empty,
}

/// 初期の盤面を作成し、戻り値として返す
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
