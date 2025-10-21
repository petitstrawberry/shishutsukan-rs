use shishutukan::{Expense, Genre, ShishutsukanClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // クライアントの初期化
    let client = ShishutsukanClient::new("http://localhost:8000");

    println!("=== 支出管理APIクライアントの使用例 ===\n");

    // 支出データの追加
    println!("1. 支出データを追加");
    let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
    match client.add_expense(&expense).await {
        Ok(result) => println!("   追加成功: {:?}", result.message),
        Err(e) => println!("   エラー: {}", e),
    }

    // 支出データの取得
    println!("\n2. 支出データ一覧を取得");
    match client.get_expenses().await {
        Ok(expenses) => {
            println!("   取得した支出データ: {} 件", expenses.len());
            for expense in expenses.iter().take(5) {
                println!(
                    "   - ID: {}, 日付: {}, ジャンル: {}, 金額: ¥{}",
                    expense.id, expense.date, expense.genre, expense.amount
                );
            }
        }
        Err(e) => println!("   エラー: {}", e),
    }

    // ジャンル一覧の取得
    println!("\n3. ジャンル一覧を取得");
    match client.get_genres().await {
        Ok(genres) => {
            println!("   取得したジャンル: {} 件", genres.len());
            for genre in genres.iter().take(5) {
                println!(
                    "   - ID: {}, 名前: {}, 作成日時: {}",
                    genre.id, genre.name, genre.created_at
                );
            }
        }
        Err(e) => println!("   エラー: {}", e),
    }

    // ジャンルの追加
    println!("\n4. ジャンルを追加");
    let new_genre = Genre::new("娯楽費".to_string());
    match client.add_genre(&new_genre).await {
        Ok(result) => println!("   追加成功: {:?}", result.message),
        Err(e) => println!("   エラー: {}", e),
    }

    println!("\n=== 完了 ===");

    Ok(())
}
