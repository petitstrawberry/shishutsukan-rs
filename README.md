# shishutsukan-rs

**shishutsukan-rs**は、[sfujibijutsukan/shishutsukan](https://github.com/sfujibijutsukan/shishutsukan) の支出管理WebアプリケーションのAPIクライアントライブラリです（Rust版）。

## 特徴

- ✅ 最小限の依存関係（reqwest、serde、tokio）
- ✅ Async/Await対応
- ✅ 型安全なAPIインターフェース
- ✅ エラーハンドリング
- ✅ 包括的なドキュメント

## インストール

`Cargo.toml` に以下を追加してください：

```toml
[dependencies]
shishutsukan = "0.1"
tokio = { version = "1", features = ["full"] }
```

## 使い方

### 基本的な初期化

```rust
use shishutsukan::ShishutsukanClient;

let client = ShishutsukanClient::new("http://localhost:8000");
```

### 支出データの操作

#### 支出データの追加

```rust
use shishutsukan::{ShishutsukanClient, Expense};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShishutsukanClient::new("http://localhost:8000");
    let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
    let result = client.add_expense(&expense).await?;
    println!("{:?}", result.message);
    Ok(())
}
```

#### 支出データの取得

```rust
use shishutsukan::ShishutsukanClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShishutsukanClient::new("http://localhost:8000");
    let expenses = client.get_expenses().await?;
    for expense in expenses {
        println!("{}: {} - ¥{}", expense.date, expense.genre, expense.amount);
    }
    Ok(())
}
```

#### 支出データの削除

```rust
use shishutsukan::ShishutsukanClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShishutsukanClient::new("http://localhost:8000");
    let expense_id = 1;
    let result = client.delete_expense(expense_id).await?;
    println!("{:?}", result.message);
    Ok(())
}
```

### ジャンルの操作

#### ジャンル一覧の取得

```rust
use shishutsukan::ShishutsukanClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShishutsukanClient::new("http://localhost:8000");
    let genres = client.get_genres().await?;
    for genre in genres {
        println!("{}: {}", genre.id, genre.name);
    }
    Ok(())
}
```

#### ジャンルの追加

```rust
use shishutsukan::{ShishutsukanClient, Genre};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShishutsukanClient::new("http://localhost:8000");
    let new_genre = Genre::new("娯楽費".to_string());
    let result = client.add_genre(&new_genre).await?;
    println!("{:?}", result.message);
    Ok(())
}
```

#### ジャンルの削除

```rust
use shishutsukan::ShishutsukanClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShishutsukanClient::new("http://localhost:8000");
    let genre_id = 7;
    let result = client.delete_genre(genre_id).await?;
    println!("{:?}", result.message);
    Ok(())
}
```

### エラーハンドリング

```rust
use shishutsukan::{ShishutsukanClient, ShishutsukanError};

#[tokio::main]
async fn main() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    match client.get_expenses().await {
        Ok(expenses) => {
            println!("取得した支出データ: {} 件", expenses.len());
        }
        Err(ShishutsukanError::InvalidUrl) => {
            eprintln!("無効なURLです");
        }
        Err(ShishutsukanError::HttpError(status)) => {
            eprintln!("HTTPエラー: {}", status);
        }
        Err(ShishutsukanError::ServerError(msg)) => {
            eprintln!("サーバーエラー: {}", msg);
        }
        Err(e) => {
            eprintln!("その他のエラー: {}", e);
        }
    }
}
```

## API仕様

### 支出管理

| メソッド | 説明 | 戻り値 |
|---------|------|--------|
| `add_expense(&self, expense: &Expense)` | 支出データを追加 | `Result<ApiMessage>` |
| `get_expenses(&self)` | 支出データ一覧を取得 | `Result<Vec<ExpenseWithId>>` |
| `delete_expense(&self, id: i32)` | 支出データを削除 | `Result<ApiMessage>` |

### ジャンル管理

| メソッド | 説明 | 戻り値 |
|---------|------|--------|
| `get_genres(&self)` | ジャンル一覧を取得 | `Result<Vec<GenreWithId>>` |
| `add_genre(&self, genre: &Genre)` | ジャンルを追加 | `Result<ApiMessage>` |
| `delete_genre(&self, id: i32)` | ジャンルを削除 | `Result<ApiMessage>` |

## データモデル

### Expense
```rust
pub struct Expense {
    pub date: String,    // 日付（例: "2025-01-15"）
    pub genre: String,   // ジャンル
    pub amount: i32,     // 金額
}
```

### ExpenseWithId
```rust
pub struct ExpenseWithId {
    pub id: i32,         // ID
    pub date: String,    // 日付
    pub genre: String,   // ジャンル
    pub amount: i32,     // 金額
}
```

### Genre
```rust
pub struct Genre {
    pub name: String,    // ジャンル名
}
```

### GenreWithId
```rust
pub struct GenreWithId {
    pub id: i32,              // ID
    pub name: String,         // ジャンル名
    pub created_at: String,   // 作成日時
}
```

### ApiMessage
```rust
pub struct ApiMessage {
    pub message: Option<String>,  // メッセージ
    pub error: Option<String>,    // エラー
}
```

## 設計思想

### アーキテクチャ

shishutsukan-rsは以下の設計原則に従っています：

1. **最小限の依存関係**: 必要最小限のクレート（reqwest、serde、tokio）のみを使用
2. **型安全**: すべてのAPIレスポンスを適切な型にマッピング
3. **Async/Await**: Rustのネイティブな async/await を使用
4. **エラーハンドリング**: thiserrorを使った明示的なエラー型による堅牢なエラー処理

### ファイル構成

```
src/
├── lib.rs            # モジュールエントリポイント
├── client.rs         # メインAPIクライアント
├── models.rs         # データモデル定義
└── error.rs          # エラー型定義
```

## テスト

### ユニットテスト

```bash
cargo test
```

### ドキュメントテスト

```bash
cargo test --doc
```

### ビルド

```bash
cargo build --release
```

## ライセンス

MIT License

Copyright (c) 2025 petitstrawberry

## 関連リンク

- [Shishutsukan WebアプリケーションRepos](https://github.com/sfujibijutsukan/shishutsukan)
- [ShishutsukanKit (Swift版)](https://github.com/petitstrawberry/ShishutsukanKit)

## 貢献

バグ報告や機能要望は [Issues](https://github.com/petitstrawberry/shishutsukan-rs/issues) からお願いします。
