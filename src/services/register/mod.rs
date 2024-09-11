//! 登録サービス
//!
//! このモジュールは、データをJOSNファイル`store/data.json`に登録する機能を提供します。

use std::io;
use std::str::FromStr;
use chrono::NaiveDate;

use crate::models;
use crate::services;

/// 家計簿アプリの登録機能を提供します。
///
/// この関数は、ユーザーからの入力を受け付け、データをJSONファイルに登録します。
///
/// #### 例
/// 
/// ```rust
/// run("store/data.json");
/// ```
///
/// #### 詳細
/// 
/// この関数は以下の手順で動作します。
/// 1. ユーザーに登録種別（収入または支出）を尋ねる。
/// 2. ユーザーに品目名を尋ねる。
/// 3. ユーザーにカテゴリ種別を尋ねる。
/// 4. ユーザーに金額を尋ねる。
/// 5. ユーザーに日付を尋ねる。
/// 6. 入力された情報をもとに、`Item`インスタンスを作成する。
/// 7. JSONファイルから既存のデータを読み込む。
/// 8. 新しい`Item`インスタンスをデータに追加する。
/// 9. 更新されたデータをJSONファイルに書き込む。
///
/// #### 注意
/// 
/// この関数は、ユーザーからの入力が正しい形式であることを前提としています。
/// 不正な入力があった場合、プログラムはパニックになります。
pub fn run(file_path: &str) {
    println!("収支の登録を行います");
    let register_type = input_register_type();
    let name = input_name();
    let category_type = input_category_type(register_type);
    let price = input_price();
    let date = input_date();
    let category = models::Item::get_category(register_type, category_type);

    let item = models::Item::new(name, category, price, date);
    println!("登録情報: {:?}", item);

    let mut data = services::io::read_data_or_create_new_data(file_path);
    data.push(item);
    services::io::write_to_json(&data, file_path);
}

/// ユーザーに登録種別（収入または支出）を尋ね、数値で返します。
///
/// #### 例
/// 
/// ```rust
/// let register_type = input_register_type();
/// println!("登録種別: {}", register_type);
/// ```
///
/// #### 注意
/// 
/// この関数は、ユーザーからの入力が正しい形式であることを前提としています。
/// 不正な入力があった場合、プログラムはパニックになります。
fn input_register_type() -> u8 {
    println!("登録種別を入力してください (0:収入, 1:支出)");
    let mut register_type = String::new();
    io::stdin().read_line(&mut register_type).expect("登録種別の入力に失敗しました");
    let register_type: u8 = register_type
                                .trim()
                                .parse()
                                .expect("登録種別は数値で入力してください");

    // バリデーション
    services::validate::InputValidator::validate_register_type(register_type);

    register_type
}

/// ユーザーに品目名を尋ね、文字列で返します。
///
/// #### 例
/// 
/// ```rust
/// let name = input_name();
/// println!("品目名: {}", name);
/// ```
///
/// #### 注意
/// 
/// この関数は、ユーザーからの入力が正しい形式であることを前提としています。
/// 不正な入力があった場合、プログラムはパニックになります。
fn input_name() -> String {
    println!("品目名を入力してください");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("品目名の入力に失敗しました");

    name.trim().to_string()
}

/// ユーザーにカテゴリ種別を尋ね、数値で返します。
///
/// #### 例
/// 
/// ```rust
/// let register_type = 1;
/// let category_type = input_category_type(register_type);
/// println!("カテゴリ種別: {}", category_type);
/// ```
///
/// #### 注意
/// 
/// この関数は、ユーザーからの入力が正しい形式であることを前提としています。
/// 不正な入力があった場合、プログラムはパニックになります。
fn input_category_type(register_type: u8) ->  u8 {
    println!("カテゴリを入力してください");
    if register_type == 0 {
        println!("(0:給与, 1:ボーナス, 2:その他)");
    } else {
        println!("(0:食費, 1:趣味, 2:その他)");
    }

    let mut category_type = String::new();
    io::stdin().read_line(&mut category_type).expect("カテゴリ種別の入力に失敗しました");
    let category_type: u8 = category_type
                                .trim()
                                .parse()
                                .expect("カテゴリ種別は数値で入力してください");

    // バリデーション
    services::validate::InputValidator::validate_category_type(register_type, category_type);

    category_type
}

/// ユーザーに金額を尋ね、数値で返します。
///
/// #### 例
/// 
/// ```rust
/// let price = input_price();
/// println!("金額: {}", price);
/// ```
///
/// #### 注意
/// 
/// この関数は、ユーザーからの入力が正しい形式であることを前提としています。
/// 不正な入力があった場合、プログラムはパニックになります。
fn input_price() -> u32 {
    println!("金額を入力してください");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("金額の入力に失敗しました");

    price.trim().parse().expect("金額は数値で入力してください")
}

/// ユーザーに日付を尋ね、`NaiveDate`オブジェクトで返します。
///
/// #### 例
/// 
/// ```rust
/// let date = input_date();
/// println!("日付: {}", date);
/// ```
///
/// #### 注意
/// 
/// この関数は、ユーザーからの入力が正しい形式であることを前提としています。
/// 不正な入力があった場合、プログラムはパニックになります。
fn input_date() -> NaiveDate {
    println!("日付を入力してください");
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("日付の入力に失敗しました");
    NaiveDate::from_str(&date).expect("日付はyyyy-mm-ddの形式で入力してください")
}