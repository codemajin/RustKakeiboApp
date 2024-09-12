//! 集計サービス
//!
//! このモジュールは、登録済みのデータから各月ごとに集計を行う機能を提供します。

use std::collections::{BTreeSet, BTreeMap};

use chrono::{Datelike, NaiveDate};

use crate::{models, services};

/// 家計簿の集計を実行する。
///
/// この関数は、指定されたファイルパスから家計簿データを読み込み、各月ごとの収支の集計結果を表示します。
///
/// #### 例
///
/// ```rust
/// use kakeibo_app::services;
/// services::summarize::run("store/data.json");
/// ```
pub fn run(file_path: &str) {
    println!("家計簿の集計を行います");
    let data = services::io::read_data_or_panic(file_path);

    let target_dates: BTreeSet<NaiveDate> = get_target_dates(&data);
    let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();

    for date in target_dates {
        let filterd_data = get_filtered_data(&data, date);
        let sum = summarize_data(&filterd_data);
        result_table.insert(date, sum);
    } 

    print_table(result_table);
}

/// 家計簿データから対象の年月の集合を取得する。
///
/// この関数は、家計簿データから各項目の年月を取得し、重複を除去した集合を返します。
fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| {
        item.get_first_day()
    }).collect();
    target_dates
}

/// 家計簿データから指定された年月のデータを抽出する。
///
/// この関数は、家計簿データから指定された年月に一致する項目を抽出し、ベクタとして返します。
fn get_filtered_data(data: &Vec<models::Item>, first_date: NaiveDate) -> Vec<&models::Item> {
    let filtered_data: Vec<_> = data.iter().filter(|item| {
        (item.get_year() == first_date.year()) && (item.get_month() == first_date.month())
    }).collect();
    filtered_data
}

/// 家計簿データの金額を集計する。
///
/// この関数は、家計簿データの金額を合計し、集計結果を返します。
fn summarize_data(data: &Vec<&models::Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary();
    }
    sum
}

/// 日付を "年/月" の形式でフォーマットする。
///
/// この関数は、指定された日付を "年/月" の形式でフォーマットし、文字列として返します。
fn format_date(date: NaiveDate) -> String {
    format!("{}/{}", date.year(), date.month())
}

/// 金額を符号付きでフォーマットする。
///
/// この関数は、指定された金額を符号付きでフォーマットし、文字列として返します。正の金額にはプラス記号が付きます。
fn format_price(price: i32) -> String {
    if price > 0 {
        format!("+{}", price)
    } else {
        format!("{}", price)
    }
}

/// 集計結果を表形式で出力する。
///
/// この関数は、集計結果を "年/月 の収支は +/-金額 円でした" の形式で出力します。
fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let date = format_date(result.0);
        let price = format_price(result.1);
        println!("{}の収支は{}円でした", date, price);
    }
}

#[cfg(test)]
mod summarize_test {
    use super::*;

    fn get_test_data() -> Vec<models::Item> {
        vec![
            super::models::Item::new(
                "新年会".to_string(),
                models::Category::Expense(models::ExpenseCategory::Food),
                5000,
                NaiveDate::from_ymd_opt(2022, 1, 10).unwrap()
            ),
            super::models::Item::new(
                "給料".to_string(),
                models::Category::Income(models::IncomeCategory::Salary),
                300000,
                NaiveDate::from_ymd_opt(2022, 1, 20).unwrap()
            ),
            super::models::Item::new(
                "旅行".to_string(),
                models::Category::Expense(models::ExpenseCategory::Hobby),
                100000,
                NaiveDate::from_ymd_opt(2022, 1, 30).unwrap()
            ),
            super::models::Item::new(
                "外食".to_string(),
                models::Category::Expense(models::ExpenseCategory::Food),
                3000,
                NaiveDate::from_ymd_opt(2022, 2, 15).unwrap()
            ),
            super::models::Item::new(
                "歓迎会".to_string(),
                models::Category::Expense(models::ExpenseCategory::Other),
                10000,
                NaiveDate::from_ymd_opt(2022, 4, 15).unwrap()
            ),
        ]
    }

    #[test]
    fn test_get_target_dates() {
        let test_data = get_test_data();
        let mut expected = BTreeSet::new();
        expected.insert(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
        expected.insert(NaiveDate::from_ymd_opt(2022, 2, 1).unwrap());
        expected.insert(NaiveDate::from_ymd_opt(2022, 4, 1).unwrap());

        assert_eq!(get_target_dates(&test_data), expected);
    }

    #[test]
    fn test_get_filtered_data() {
        let test_data = get_test_data();
        let first_date = NaiveDate::from_ymd_opt(2022, 4, 20).unwrap();
        let expected = vec![&test_data[4]];

        assert_eq!(get_filtered_data(&test_data, first_date), expected);
    }

    #[test]
    fn test_summarize_data() {
        let data = get_test_data();
        let test_data = vec![&data[0], &data[1], &data[2]];
        let expected: i32 = data[0..=2].iter().map(|item| item.get_price_for_summary()).sum();

        assert_eq!(summarize_data(&test_data), expected);
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2022, 4, 20).unwrap();
        let expected = "2022/4";

        assert_eq!(format_date(date), expected);
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(1000), "+1000");
        assert_eq!(format_price(-1000), "-1000");
    }
}