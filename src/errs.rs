/// このプログラムが出力するエラーの一覧
#[derive(Debug, PartialEq)]
pub enum Errs {
    StrsLengthErr,
    //FirstStrErr,
    SecondStrErr,
    //ThirdStrErr,
    CantMoveErr,
    //UnknownErr,
}

/// エラーを出力する
/// 続行可能なエラー => println!()
/// 続行不可なエラー => panic!()
pub fn print_err(e: Errs) {
    match e {
        //Errs::FirstStrErr => println!("1文字目が正しくありません。"),
        Errs::SecondStrErr => println!("2文字目が正しくありません。"),
        //Errs::ThirdStrErr => println!("3文字目が正しくありません。"),
        //Errs::ForthStrErr => println!("4文字目が正しくありません。"),
        Errs::StrsLengthErr => println!("文字数が正しくありません。"),
        Errs::CantMoveErr => println!("そのマスには動けません。"),
        //Errs::UnknownErr => panic!("不明なエラーが発生"),
    }
}
