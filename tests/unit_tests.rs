/// Unit tests that verify basic functionality without requiring a server
use shishutsukan::{Expense, Genre, ShishutsukanClient};

#[test]
fn test_client_instantiation() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    // Client should be created successfully
    assert!(format!("{:?}", client).contains("ShishutsukanClient"));
}

#[test]
fn test_expense_creation() {
    let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
    assert_eq!(expense.date, "2025-01-15");
    assert_eq!(expense.genre, "食費");
    assert_eq!(expense.amount, 1000);
}

#[test]
fn test_genre_creation() {
    let genre = Genre::new("娯楽費".to_string());
    assert_eq!(genre.name, "娯楽費");
}

#[test]
fn test_expense_serialization() {
    let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
    let json = serde_json::to_string(&expense).expect("Failed to serialize");
    assert!(json.contains("2025-01-15"));
    assert!(json.contains("食費"));
    assert!(json.contains("1000"));
}

#[test]
fn test_genre_serialization() {
    let genre = Genre::new("娯楽費".to_string());
    let json = serde_json::to_string(&genre).expect("Failed to serialize");
    assert!(json.contains("娯楽費"));
}

#[test]
fn test_expense_deserialization() {
    let json = r#"{"date":"2025-01-15","genre":"食費","amount":1000}"#;
    let expense: Expense = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(expense.date, "2025-01-15");
    assert_eq!(expense.genre, "食費");
    assert_eq!(expense.amount, 1000);
}

#[test]
fn test_genre_deserialization() {
    let json = r#"{"name":"娯楽費"}"#;
    let genre: Genre = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(genre.name, "娯楽費");
}
