//! 家計簿データ定義
//!
//! このモジュールは、データ構造の定義やデータ操作のための機能を実装したものです。

use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike};

/// 税金カテゴリを表す列挙型
///
/// これは、収入のカテゴリを表します。
/// - `Salary`: 給与
/// - `Bonus`: ボーナス
/// - `Other`: その他の収入
///
/// #### 例
/// 
/// ```rust
/// use kakeibo_app::models::IncomeCategory;
/// 
/// let salary = IncomeCategory::Salary;
/// let bonus = IncomeCategory::Bonus;
/// let other = IncomeCategory::Other;
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Bonus,
    Other,
}

/// 経費カテゴリを表す列挙型
///
/// これは、支出のカテゴリを表します。
/// - `Food`: 食費
/// - `Hobby`: 趣味
/// - `Other`: その他の支出
///
/// #### 例
/// 
/// ```rust
/// use kakeibo_app::models::ExpenseCategory;
/// 
/// let food = ExpenseCategory::Food;
/// let hobby = ExpenseCategory::Hobby;
/// let other = ExpenseCategory::Other;
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenseCategory {
    Food,
    Hobby,
    Other,
}

/// カテゴリを表す列挙型
///
/// これは、収入と支出のカテゴリをまとめたものです。
/// - `Income`: 収入カテゴリ
/// - `Expense`: 支出カテゴリ
///
/// #### 例
/// 
/// ~~~rust
/// use kakeibo_app::models::{Category, IncomeCategory, ExpenseCategory};
/// 
/// let income = Category::Income(IncomeCategory::Salary);
/// let expense = Category::Expense(ExpenseCategory::Food);
/// ~~~
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

/// 項目を表す構造体
///
/// これは、家計簿アプリの項目を表します。
/// - `name`: 項目の名前
/// - `category`: 項目のカテゴリ
/// - `price`: 項目の金額
/// - `date`: 項目の日付
///
/// #### 例
/// 
/// ```rust
/// use kakeibo_app::models::{Item, Category, IncomeCategory};
/// use chrono::{NaiveDate, Datelike};
/// 
/// let item = Item::new(
///     String::from("給与"),
///     Category::Income(IncomeCategory::Salary),
///     100000,
///     NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
}

impl Item {
    /// 新しい項目を作成する
    ///
    /// #### 引数
    /// 
    /// - `name`: 項目の名前
    /// - `category`: 項目のカテゴリ
    /// - `price`: 項目の金額
    /// - `date`: 項目の日付
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::models::{Item, Category, IncomeCategory};
    /// use chrono::{NaiveDate, Datelike};
    /// 
    /// let item = Item::new(
    ///     String::from("給与"),
    ///     Category::Income(IncomeCategory::Salary),
    ///     100000,
    ///     NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    /// );
    /// ```
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Item { name, category, price, date }
    }

    /// カテゴリを取得する
    ///
    /// #### 引数
    /// 
    /// - `register_type`: 登録タイプ（0: 収入, 1: 支出）
    /// - `category_type`: カテゴリタイプ（0, 1, 2）
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::models::{Item, Category, IncomeCategory};
    /// 
    /// let category = Item::get_category(0, 0);
    /// assert_eq!(category, Category::Income(IncomeCategory::Salary));
    /// ```
    pub fn get_category(register_type: u8, category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategory::Salary),
                1 => Category::Income(IncomeCategory::Bonus),
                2 => Category::Income(IncomeCategory::Other),
                _ => panic!("不正なカテゴリ種別です")
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpenseCategory::Food),
                1 => Category::Expense(ExpenseCategory::Hobby),
                2 => Category::Expense(ExpenseCategory::Other),
                _ => panic!("不正なカテゴリ種別です")
            }
        }
    }

    /// 年を取得する
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::models::{Item, Category, IncomeCategory};
    /// use chrono::{NaiveDate, Datelike};
    /// 
    /// let item = Item::new(
    ///     String::from("給与"),
    ///     Category::Income(IncomeCategory::Salary),
    ///     100000,
    ///     NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    /// );
    /// assert_eq!(item.get_year(), 2023);
    /// ```
    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    /// 月を取得する
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::models::{Item, Category, IncomeCategory};
    /// use chrono::{NaiveDate, Datelike};
    /// 
    /// let item = Item::new(
    ///     String::from("給与"),
    ///     Category::Income(IncomeCategory::Salary),
    ///     100000,
    ///     NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    /// );
    /// assert_eq!(item.get_month(), 1);
    /// ```
    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    /// 月の最初の日を取得する
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::models::{Item, Category, IncomeCategory};
    /// use chrono::{NaiveDate, Datelike};
    /// 
    /// let item = Item::new(
    ///     String::from("給与"),
    ///     Category::Income(IncomeCategory::Salary),
    ///     100000,
    ///     NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
    /// );
    /// assert_eq!(item.get_first_day(), NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    /// ```
    pub fn get_first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.get_year(), self.get_month(), 1).unwrap()
    }

    /// まとめのための金額を取得する
    ///
    /// 収入の場合は正の値、支出の場合は負の値を返します。
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::models::{Item, Category, IncomeCategory, ExpenseCategory};
    /// use chrono::{NaiveDate, Datelike};
    /// 
    /// let income_item = Item::new(
    ///     String::from("給与"),
    ///     Category::Income(IncomeCategory::Salary),
    ///     100000,
    ///     NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    /// );
    /// let expense_item = Item::new(
    ///     String::from("食費"),
    ///     Category::Expense(ExpenseCategory::Food),
    ///     5000,
    ///     NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    /// );
    /// assert_eq!(income_item.get_price_for_summary(), 100000);
    /// assert_eq!(expense_item.get_price_for_summary(), -5000);
    /// ```
    pub fn get_price_for_summary(&self) -> i32 {
        match self.category {
            Category::Income(_) => self.price as i32,
            Category::Expense(_) => -1 * self.price as i32,
        }
    }
}