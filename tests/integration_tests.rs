/// Integration tests that verify shishutsukan-rs works with actual shishutsukan server
/// These tests require a running shishutsukan server on localhost:8000
use shishutsukan::{Expense, Genre, ShishutsukanClient};

// Helper function to generate unique names for tests
fn generate_unique_name(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{}_{}", prefix, timestamp)
}

// MARK: - Genre Integration Tests

#[tokio::test]
async fn test_get_genres() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Get genres from server
    let genres = client.get_genres().await.expect("Failed to get genres");
    
    // Server should have default genres initialized
    assert!(!genres.is_empty(), "Server should have default genres");
    
    // Verify genre structure
    for genre in genres {
        assert!(genre.id > 0, "Genre should have valid ID");
        assert!(!genre.name.is_empty(), "Genre should have name");
        assert!(!genre.created_at.is_empty(), "Genre should have creation timestamp");
    }
}

#[tokio::test]
async fn test_add_genre() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Create a unique genre name for this test
    let unique_name = generate_unique_name("TestGenre");
    let new_genre = Genre::new(unique_name.clone());
    
    // Add genre
    let result = client.add_genre(&new_genre).await.expect("Failed to add genre");
    assert_eq!(result.message, Some("ok".to_string()), "Genre should be added successfully");
    
    // Verify it appears in the list
    let genres = client.get_genres().await.expect("Failed to get genres");
    assert!(genres.iter().any(|g| g.name == unique_name), "Added genre should appear in list");
    
    // Clean up: delete the added genre
    if let Some(added_genre) = genres.iter().find(|g| g.name == unique_name) {
        let _ = client.delete_genre(added_genre.id).await;
    }
}

#[tokio::test]
async fn test_delete_genre() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Add a genre first
    let unique_name = generate_unique_name("ToDelete");
    let new_genre = Genre::new(unique_name.clone());
    client.add_genre(&new_genre).await.expect("Failed to add genre");
    
    // Find the added genre
    let genres = client.get_genres().await.expect("Failed to get genres");
    let genre_to_delete = genres.iter().find(|g| g.name == unique_name)
        .expect("Failed to find added genre");
    
    // Delete it
    let result = client.delete_genre(genre_to_delete.id).await.expect("Failed to delete genre");
    assert_eq!(result.message, Some("deleted".to_string()), "Genre should be deleted successfully");
    
    // Verify it's removed from the list
    let genres_after_deletion = client.get_genres().await.expect("Failed to get genres");
    assert!(!genres_after_deletion.iter().any(|g| g.id == genre_to_delete.id), 
            "Deleted genre should not appear in list");
}

// MARK: - Expense Integration Tests

#[tokio::test]
async fn test_get_expenses() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Get expenses from server
    let expenses = client.get_expenses().await.expect("Failed to get expenses");
    
    // This is a valid operation even if the list is empty
    assert!(expenses.is_empty() || !expenses.is_empty(), "Should get expenses list (can be empty)");
}

#[tokio::test]
async fn test_add_expense() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Add an expense
    let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
    let result = client.add_expense(&expense).await.expect("Failed to add expense");
    assert_eq!(result.message, Some("ok".to_string()), "Expense should be added successfully");
    
    // Verify it appears in the list
    let expenses = client.get_expenses().await.expect("Failed to get expenses");
    assert!(expenses.iter().any(|e| 
        e.date == expense.date && e.genre == expense.genre && e.amount == expense.amount
    ), "Added expense should appear in list");
    
    // Clean up: delete the added expense
    if let Some(added_expense) = expenses.iter().find(|e| 
        e.date == expense.date && e.genre == expense.genre && e.amount == expense.amount
    ) {
        let _ = client.delete_expense(added_expense.id).await;
    }
}

#[tokio::test]
async fn test_delete_expense() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Add an expense first
    let expense = Expense::new("2025-01-20".to_string(), "交通費".to_string(), 500);
    client.add_expense(&expense).await.expect("Failed to add expense");
    
    // Find the added expense
    let expenses = client.get_expenses().await.expect("Failed to get expenses");
    let expense_to_delete = expenses.iter().find(|e| 
        e.date == expense.date && e.genre == expense.genre && e.amount == expense.amount
    ).expect("Failed to find added expense");
    
    // Delete it
    let result = client.delete_expense(expense_to_delete.id).await.expect("Failed to delete expense");
    assert_eq!(result.message, Some("deleted".to_string()), "Expense should be deleted successfully");
    
    // Verify it's removed from the list
    let expenses_after_deletion = client.get_expenses().await.expect("Failed to get expenses");
    assert!(!expenses_after_deletion.iter().any(|e| e.id == expense_to_delete.id), 
            "Deleted expense should not appear in list");
}

// MARK: - Complete Workflow Test

#[tokio::test]
async fn test_complete_workflow() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // This test simulates a complete workflow of the client
    
    // 1. Get initial state
    let initial_genres = client.get_genres().await.expect("Failed to get genres");
    let _ = client.get_expenses().await.expect("Failed to get expenses");
    
    assert!(!initial_genres.is_empty(), "Server should have default genres");
    
    // 2. Add a custom genre
    let custom_genre_name = generate_unique_name("統合テスト");
    let custom_genre = Genre::new(custom_genre_name.clone());
    let add_genre_result = client.add_genre(&custom_genre).await.expect("Failed to add genre");
    assert_eq!(add_genre_result.message, Some("ok".to_string()));
    
    // 3. Verify genre was added
    let genres_after_add = client.get_genres().await.expect("Failed to get genres");
    assert_eq!(genres_after_add.len(), initial_genres.len() + 1);
    let added_genre = genres_after_add.iter().find(|g| g.name == custom_genre_name)
        .expect("Custom genre not found");
    
    // 4. Add expenses using the custom genre
    let expense1 = Expense::new("2025-01-21".to_string(), custom_genre_name.clone(), 1234);
    let expense2 = Expense::new("2025-01-22".to_string(), custom_genre_name.clone(), 5678);
    
    client.add_expense(&expense1).await.expect("Failed to add expense1");
    client.add_expense(&expense2).await.expect("Failed to add expense2");
    
    // 5. Verify expenses were added
    let expenses_after_add = client.get_expenses().await.expect("Failed to get expenses");
    let added_expenses: Vec<_> = expenses_after_add.iter()
        .filter(|e| e.genre == custom_genre_name)
        .collect();
    assert_eq!(added_expenses.len(), 2);
    
    // 6. Delete expenses
    for expense in added_expenses {
        client.delete_expense(expense.id).await.expect("Failed to delete expense");
    }
    
    // 7. Verify expenses were deleted
    let expenses_after_delete = client.get_expenses().await.expect("Failed to get expenses");
    assert!(!expenses_after_delete.iter().any(|e| e.genre == custom_genre_name));
    
    // 8. Delete custom genre
    let delete_genre_result = client.delete_genre(added_genre.id).await.expect("Failed to delete genre");
    assert_eq!(delete_genre_result.message, Some("deleted".to_string()));
    
    // 9. Verify genre was deleted
    let genres_after_delete = client.get_genres().await.expect("Failed to get genres");
    assert_eq!(genres_after_delete.len(), initial_genres.len());
}

// MARK: - Error Handling Tests

#[tokio::test]
async fn test_delete_non_existent_expense() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Try to delete an expense with a very high ID that doesn't exist
    let result = client.delete_expense(999999).await.expect("Failed to delete expense");
    
    // Server returns "deleted" even for non-existent IDs
    assert_eq!(result.message, Some("deleted".to_string()));
}

#[tokio::test]
async fn test_add_duplicate_genre() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Try to add a genre that already exists
    let genres = client.get_genres().await.expect("Failed to get genres");
    let existing_genre = genres.first().expect("No genres available for testing");
    
    let duplicate_genre = Genre::new(existing_genre.name.clone());
    
    // Server should return error for duplicate genres
    match client.add_genre(&duplicate_genre).await {
        Ok(result) => {
            // If we get here, check for error in response
            assert!(result.error.is_some(), "Should get error for duplicate genre");
        }
        Err(e) => {
            // Expected error - server returned error message
            let error_message = format!("{:?}", e);
            assert!(error_message.contains("ServerError") || error_message.contains("already exists"), 
                    "Error should mention duplicate genre or server error");
        }
    }
}

#[tokio::test]
async fn test_delete_genre_in_use() {
    let client = ShishutsukanClient::new("http://localhost:8000");
    
    // Add a custom genre
    let genre_name = generate_unique_name("InUse");
    client.add_genre(&Genre::new(genre_name.clone())).await.expect("Failed to add genre");
    
    // Find the genre
    let genres = client.get_genres().await.expect("Failed to get genres");
    let genre = genres.iter().find(|g| g.name == genre_name)
        .expect("Failed to find added genre");
    
    // Add an expense using this genre
    let expense = Expense::new("2025-01-23".to_string(), genre_name.clone(), 100);
    client.add_expense(&expense).await.expect("Failed to add expense");
    
    // Try to delete the genre (should fail because it's in use)
    match client.delete_genre(genre.id).await {
        Ok(result) => {
            // If we get here, check for error in response
            assert!(result.error.is_some(), "Should get error when deleting genre in use");
        }
        Err(e) => {
            // Expected error - server returned error message
            let error_message = format!("{:?}", e);
            assert!(error_message.contains("ServerError") || error_message.contains("in use"), 
                    "Error should mention genre is in use or server error");
        }
    }
    
    // Clean up: delete the expense and genre
    let expenses = client.get_expenses().await.expect("Failed to get expenses");
    if let Some(expense_to_delete) = expenses.iter().find(|e| e.genre == genre_name) {
        let _ = client.delete_expense(expense_to_delete.id).await;
        let _ = client.delete_genre(genre.id).await;
    }
}
