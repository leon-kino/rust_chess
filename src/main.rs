mod check;
mod errs;
mod i_o;
mod library;
mod movements;
mod piece;

use crate::errs::Errs;

/// メイン関数
fn main() {
    let mut pieces = piece::init(); // 駒の情報が入る変数
    let mut input_str;
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
                // 今回の処理のせいでチェックされていないかを確認
                if check::check_check(&v, index % 2 == 0).len() != 0 {
                    i_o::show_board(&pieces);
                    errs::print_err(Errs::CantMoveErr);
                    index -= 1;
                } else {
                    // OKの場合は、処理を行った後に表示する
                    pieces = v;
                    i_o::show_board(&pieces);
                }
            }
            Err(e) => {
                // Errの場合は先に表示してからエラーを表示
                i_o::show_board(&pieces);
                errs::print_err(e);
                index -= 1;
            }
        }

        input_str.clear();
        index += 1;
    }
}
