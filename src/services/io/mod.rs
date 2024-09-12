//! データ入出力サービス
//!
//! このモジュールは、JOSNファイル`store/data.json`へのデータ入出力処理の機能を提供します。

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::models;

/// JSONファイルからデータを読み込むか、新しいデータを作成します。
/// 
/// 指定されたファイルパスからデータを読み込みます。ファイルが存在しない場合は、新しいデータ（空のベクトル）を作成します。
/// 
/// #### 例
/// 
/// ```rust
/// use kakeibo_app::services::io;
/// let file_path = "store/data.json";
/// let data = io::read_data_or_create_new_data(file_path);
/// ```
pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path);
    match file {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました")
        },
        Err(_) => {
            println!("新規ファイルを作成します");
            Vec::new()
        }
    }
}

/// JSONファイルからデータを読み込みます。データが存在しない場合はパニックになります。
/// 
/// 指定されたファイルパスからデータを読み込みます。ファイルが存在しないか、データが空の場合はパニックになります。
/// 
/// #### 例
/// 
/// ```rust
/// use kakeibo_app::services::io;
/// let file_path = "store/data.json";
/// let data = io::read_data_or_panic(file_path);
/// ```
pub fn read_data_or_panic(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path).expect("ファイルをオープンできませんでした");
    let buf_reader = BufReader::new(file);
    let data: Vec<_> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");

    if data.len() == 0 {
        panic!("データが存在しません");
    }

    data
}

/// データをJSONファイルに書き込みます。
/// 
/// 指定されたデータをJSON形式にシリアライズし、指定されたファイルパスに書き込みます。
/// 
/// #### 例
/// 
/// ```rust
/// use kakeibo_app::services::io;
/// use kakeibo_app::models::{Item, Category, IncomeCategory, ExpenseCategory};
/// use chrono::{NaiveDate, Datelike};
/// 
/// let file_path = "store/data.json";
/// let data = vec![
///     Item::new(
///         String::from("給与"),
///         Category::Income(IncomeCategory::Salary),
///         100000,
///         NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
///     ),
///     Item::new(
///         String::from("食費"),
///         Category::Expense(ExpenseCategory::Food),
///         2000,
///         NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
///     ),
/// ];
/// io::write_to_json(&data, file_path);
/// ```
pub fn write_to_json(data: &Vec<models::Item>, file_path: &str) {
    let json_data = serde_json::to_string_pretty(data).expect("JSONへのシリアライズに失敗しました");
    let mut file = File::create(file_path).expect("書き込みファイルのオープンに失敗しました");
    writeln!(file, "{}", json_data).expect("ファイルへの書き込みに失敗しました");
    println!("項目の登録が完了しました");
}