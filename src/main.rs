mod i_o;

use std::{
    cmp::{max, min},
    usize,
};

type Board = [[Piece; 8]; 8];

fn main() {
    let mut pieces = init(); // 駒の情報が入る変数
    let mut input_str = Default::default(); // 入力文字列を格納する変数
    let mut index = 0; // 何ターン目か（偶数=>白のターン,奇数=>黒のターン）

    i_o::show_board(&pieces);

    loop {
        println!("\n{}のターンです", {
            if index % 2 == 0 {
                '白'
            } else {
                '黒'
            }
        });
        input_str = i_o::input();

        let rtn = i_o::analize_str(input_str.chars().collect::<Vec<char>>(), &pieces, index);

        match rtn {
            Ok(v) => {
                // OKの場合は、処理を行った後に表示する
                pieces = v;
                i_o::show_board(&pieces);
            }
            Err(e) => {
                // Errの場合は先に表示してからエラーを表示
                i_o::show_board(&pieces);
                print_err(e);
                index -= 1;
            }
        }

        input_str.clear();
        index += 1;
    }
}

/// 初期の盤面を作成し、戻り値として返す
fn init() -> Board {
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

/// エラーを出力する
/// 続行可能なエラー => println!()
/// 続行不可なエラー => panic!()
fn print_err(e: Errs) {
    match e {
        Errs::FirstStrErr => println!("1文字目が正しくありません。"),
        Errs::SecondStrErr => println!("2文字目が正しくありません。"),
        Errs::ThirdStrErr => println!("3文字目が正しくありません。"),
        //Errs::ForthStrErr => println!("4文字目が正しくありません。"),
        Errs::StrsLengthErr => println!("文字数が正しくありません。"),
        Errs::CantMoveErr => println!("そのマスには動けません。"),
        Errs::UnknownErr => panic!("不明なエラーが発生"),
    }
}

/// 駒の種類・色・移動したかの情報を持つ構造体
#[derive(Clone, Copy)]
pub struct Piece {
    piece_kind: PieceKinds, // 駒の種類
    color: Colors,          // 駒の色
    is_moved: bool,         // まだ1回も移動していない=>false , 既に移動した=>true
}

impl Piece {
    /// インスタンスを生成する
    fn create_instance(piece_kind: PieceKinds, color: Colors) -> Piece {
        Piece {
            piece_kind,
            color,
            is_moved: false,
        }
    }

    /// 駒の情報から文字列を作成し、返す
    fn print(&self) -> String {
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
enum PieceKinds {
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
enum Colors {
    White,
    Black,
    Empty,
}

/// 引数がtrue => White
///     false => Black
fn solve_color(is_white: bool) -> Colors {
    if is_white {
        Colors::White
    } else {
        Colors::Black
    }
}

pub enum Errs {
    StrsLengthErr,
    FirstStrErr,
    SecondStrErr,
    ThirdStrErr,
    CantMoveErr,
    UnknownErr,
}

fn king<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
    // todo 動こうとしたマスに敵の駒の攻撃がないか
    // todo キャスリング
    todo!()
}
fn queen<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
    // todo 動こうとしたマスが直線上にあるか
    // todo 動こうとしたマスがなにかの駒で防がれていないか
    todo!()
}
fn rook<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
    // todo 動こうとしたマスが直線上にあるか
    // todo 動こうとしたマスがなにかの駒で防がれていないか
    // todo キャスリング
    todo!()
}
fn bishop<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
    // todo 動こうとしたマスが直線上にあるか
    // todo 動こうとしたマスがなにかの駒で防がれていないか
    todo!()
}
fn knight<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
    // todo 動きたいマスに動くことができるか
    todo!()
}
fn pawn<'a>(pieces: &Board, strs: &Vec<char>, is_white: bool) -> Result<Board, Errs> {
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
                            let mut input_str = String::new();
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

/// アルファベットを数字に変換する
fn alphabet_to_number(alph: &char) -> Result<usize, Errs> {
    match alph {
        'a'..='h' => Ok(*alph as usize - 'a' as usize),
        'A'..='H' => Ok(*alph as usize - 'A' as usize),
        _ => Err(Errs::SecondStrErr),
    }
}

/// 指定された座標にある駒の色を特定する
/// 駒なし => 0
/// 同じ色の駒 => 1
/// 異なる色の駒 => 2
fn judge_exist(pieces: &Board, x: usize, y: usize, is_white: bool) -> usize {
    let color = solve_color(is_white);

    if pieces[y][x].color == Colors::Empty {
        0
    } else if color == pieces[y][x].color {
        1
    } else {
        2
    }
}

/// 受け取った数字がマスの範囲内 => usizeに変換
/// 受け取った数字がマスの範囲外 => Errs::CantMoveErrを返す
fn is_inner_board(num: isize) -> Result<usize, Errs> {
    if num < 0 || num > 7 {
        Err(Errs::CantMoveErr)
    } else {
        Ok(num as usize)
    }
}

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
