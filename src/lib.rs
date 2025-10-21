//! # Shishutsukan - 支出管理APIクライアント
//!
//! **Shishutsukan**は、[sfujibijutsukan/shishutsukan](https://github.com/sfujibijutsukan/shishutsukan)
//! の支出管理WebアプリケーションのAPIクライアントライブラリです。
//!
//! ## 特徴
//!
//! - ✅ 最小限の依存関係（reqwest、serde、tokio）
//! - ✅ Async/Await対応
//! - ✅ 型安全なAPIインターフェース
//! - ✅ エラーハンドリング
//!
//! ## インストール
//!
//! `Cargo.toml` に以下を追加してください：
//!
//! ```toml
//! [dependencies]
//! shishutsukan = "0.1"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## 使い方
//!
//! ### 基本的な初期化
//!
//! ```rust
//! use shishutsukan::ShishutsukanClient;
//!
//! let client = ShishutsukanClient::new("http://localhost:8000");
//! ```
//!
//! ### 支出データの操作
//!
//! ```no_run
//! use shishutsukan::{ShishutsukanClient, Expense};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ShishutsukanClient::new("http://localhost:8000");
//!
//! // 支出データの追加
//! let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
//! let result = client.add_expense(&expense).await?;
//!
//! // 支出データの取得
//! let expenses = client.get_expenses().await?;
//! for expense in expenses {
//!     println!("{}: {} - ¥{}", expense.date, expense.genre, expense.amount);
//! }
//!
//! // 支出データの削除
//! let result = client.delete_expense(1).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### ジャンルの操作
//!
//! ```no_run
//! use shishutsukan::{ShishutsukanClient, Genre};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ShishutsukanClient::new("http://localhost:8000");
//!
//! // ジャンル一覧の取得
//! let genres = client.get_genres().await?;
//! for genre in genres {
//!     println!("{}: {}", genre.id, genre.name);
//! }
//!
//! // ジャンルの追加
//! let new_genre = Genre::new("娯楽費".to_string());
//! let result = client.add_genre(&new_genre).await?;
//!
//! // ジャンルの削除
//! let result = client.delete_genre(7).await?;
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
mod models;

pub use client::ShishutsukanClient;
pub use error::{Result, ShishutsukanError};
pub use models::{ApiMessage, Expense, ExpenseWithId, Genre, GenreWithId};
