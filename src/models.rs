use serde::{Deserialize, Serialize};

/// 支出データモデル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expense {
    /// 日付（例: "2025-01-15"）
    pub date: String,
    /// ジャンル
    pub genre: String,
    /// 金額
    pub amount: i32,
}

impl Expense {
    /// 新しい支出データを作成
    pub fn new(date: String, genre: String, amount: i32) -> Self {
        Self {
            date,
            genre,
            amount,
        }
    }
}

/// ID付き支出データモデル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpenseWithId {
    /// ID
    pub id: i32,
    /// 日付
    pub date: String,
    /// ジャンル
    pub genre: String,
    /// 金額
    pub amount: i32,
}

/// ジャンルデータモデル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Genre {
    /// ジャンル名
    pub name: String,
}

impl Genre {
    /// 新しいジャンルを作成
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// ID付きジャンルデータモデル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenreWithId {
    /// ID
    pub id: i32,
    /// ジャンル名
    pub name: String,
    /// 作成日時
    #[serde(rename = "created_at")]
    pub created_at: String,
}

/// APIレスポンスメッセージ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiMessage {
    /// メッセージ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// エラー
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
