//! 簡易版家計簿アプリ
//!
//! このアプリは、家計簿の登録と集計を行うためのシンプルなCLIアプリです。
//! ユーザーは、登録と集計のどちらかの機能を選択し、入力したデータをJSONファイルに保存または読み込みます。
//!
//! #### 例
//!
//! ```
//! cargo run
//! ```

use std::io;
use kakeibo_app::services;

const FILE_PATH: &str = "store/data.json";

/// main関数
///
/// アプリのエントリーポイントです。
/// ユーザーに実行したい内容の入力を求め、入力値に基づいて登録または集計の機能を実行します。
///
/// #### 例
///
/// ```
/// cargo run
/// ```
fn main() {
    let mut service_type = String::new();
    println!("実行したい内容を入力してください (0:登録, 1:集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type
                            .trim()
                            .parse()
                            .expect("数値で入力してください");

    // 入力値のバリデーション
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        services::register::run(FILE_PATH);
    } else {
        services::summarize::run(FILE_PATH);
    }
}
