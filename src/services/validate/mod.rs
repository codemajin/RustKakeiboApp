//! バリデーション機能
//!
//! 入力値のバリデーションを提供するモジュールです。
//! このモジュールは、簡易版家計簿アプリの入力値を検証するための機能を提供します。
//!
//! #### 例
//!
//! ```rust
//! use kakeibo_app::services::validate::InputValidator;
//!
//! let service_type = 0;
//! let register_type = 1;
//! let category_type = 2;
//!
//! InputValidator::validate_service_type(service_type);
//! InputValidator::validate_register_type(register_type);
//! InputValidator::validate_category_type(register_type, category_type);
//! ```

/// 入力値のバリデーションを行う構造体
pub struct InputValidator {}

impl InputValidator {
    /// サービスタイプの入力値を検証します。
    ///
    /// #### パニック
    /// 
    /// サービスタイプが0または1以外の場合、パニックになります。
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::services::validate::InputValidator;
    /// 
    /// let service_type = 0;
    /// InputValidator::validate_service_type(service_type);
    /// ```
    pub fn validate_service_type(service_type: u8) {
        match service_type {
            0 | 1 => {},
            _ => panic!("入力値が不正です")
        }
    }

    /// 登録種別の入力値を検証します。
    ///
    /// #### パニック
    /// 
    /// 登録種別が0または1以外の場合、パニックになります。
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::services::validate::InputValidator;
    /// 
    /// let register_type = 1;
    /// InputValidator::validate_register_type(register_type);
    /// ```
    pub fn validate_register_type(register_type: u8) {
        match register_type {
            0 | 1 => {},
            _ => panic!("登録種別の入力値が不正です")
        }
    }

    /// カテゴリタイプの入力値を検証します。
    ///
    /// #### パニック
    /// 
    /// カテゴリタイプが0、1または2以外の場合、パニックになります。
    ///
    /// #### 例
    /// 
    /// ```rust
    /// use kakeibo_app::services::validate::InputValidator;
    /// 
    /// let register_type = 1;
    /// let category_type = 2;
    /// InputValidator::validate_category_type(register_type, category_type);
    /// ```
    pub fn validate_category_type(register_type: u8, category_type: u8) {
        if register_type == 0 {
            match category_type {
                0 | 1 | 2 => {},
                _ => panic!("カテゴリ入力値が不正です")
            }
        } else {
            match category_type {
                0 | 1 | 2 => {},
                _ => panic!("カテゴリ入力値が不正です")
            }
        }
    }
}

#[cfg(test)]
mod validate_test {
    use super::*;

    #[test]
    fn test_validate_service_type_for_ok() {
        InputValidator::validate_service_type(0);
        InputValidator::validate_service_type(1);
    }

    #[test]
    #[should_panic(expected="入力値が不正です")]
    fn test_validate_service_type_for_ng() {
        InputValidator::validate_service_type(2);
    }

    #[test]
    fn test_validate_register_type_for_ok() {
        InputValidator::validate_register_type(0);
        InputValidator::validate_register_type(1);
    }

    #[test]
    #[should_panic(expected="登録種別の入力値が不正です")]
    fn test_validate_register_type_for_ng() {
        InputValidator::validate_register_type(2);
    }

    #[test]
    fn test_validate_category_type_for_ok() {
        InputValidator::validate_category_type(0, 1);
        InputValidator::validate_category_type(0, 1);
        InputValidator::validate_category_type(0, 2);
        InputValidator::validate_category_type(1, 1);
        InputValidator::validate_category_type(1, 1);
        InputValidator::validate_category_type(1, 2);
    }

    #[test]
    #[should_panic(expected="カテゴリ入力値が不正です")]
    fn test_validate_category_type_for_ng() {
        InputValidator::validate_category_type(0, 3);
    }
}
