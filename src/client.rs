use crate::error::{Result, ShishutsukanError};
use crate::models::{ApiMessage, Expense, ExpenseWithId, Genre, GenreWithId};
use reqwest::Client;

/// 支出管理APIクライアント
#[derive(Debug, Clone)]
pub struct ShishutsukanClient {
    base_url: String,
    client: Client,
}

impl ShishutsukanClient {
    /// 新しいクライアントを作成
    ///
    /// # Arguments
    ///
    /// * `base_url` - APIのベースURL（例: "http://localhost:8000"）
    ///
    /// # Examples
    ///
    /// ```
    /// use shishutukan::ShishutsukanClient;
    ///
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }

    /// カスタムHTTPクライアントを使用して新しいクライアントを作成
    ///
    /// # Arguments
    ///
    /// * `base_url` - APIのベースURL
    /// * `client` - カスタムreqwestクライアント
    pub fn with_client(base_url: impl Into<String>, client: Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
        }
    }

    // MARK: - Expense APIs

    /// 支出データを追加
    ///
    /// # Arguments
    ///
    /// * `expense` - 追加する支出データ
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use shishutukan::{ShishutsukanClient, Expense};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
    /// let result = client.add_expense(&expense).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_expense(&self, expense: &Expense) -> Result<ApiMessage> {
        let url = format!("{}/expenses", self.base_url);

        let response = self.client.post(&url).json(expense).send().await?;

        self.validate_response(&response)?;

        let message: ApiMessage = response.json().await?;
        if let Some(error) = message.error.as_ref() {
            return Err(ShishutsukanError::ServerError(error.clone()));
        }

        Ok(message)
    }

    /// 支出データの一覧を取得
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use shishutukan::ShishutsukanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// let expenses = client.get_expenses().await?;
    /// for expense in expenses {
    ///     println!("{}: {} - ¥{}", expense.date, expense.genre, expense.amount);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_expenses(&self) -> Result<Vec<ExpenseWithId>> {
        let url = format!("{}/expenses", self.base_url);

        let response = self.client.get(&url).send().await?;

        self.validate_response(&response)?;

        let expenses: Vec<ExpenseWithId> = response.json().await?;
        Ok(expenses)
    }

    /// 支出データを削除
    ///
    /// # Arguments
    ///
    /// * `id` - 削除する支出データのID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use shishutukan::ShishutsukanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// let result = client.delete_expense(1).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_expense(&self, id: i32) -> Result<ApiMessage> {
        let url = format!("{}/expenses/{}", self.base_url, id);

        let response = self.client.delete(&url).send().await?;

        self.validate_response(&response)?;

        let message: ApiMessage = response.json().await?;
        if let Some(error) = message.error.as_ref() {
            return Err(ShishutsukanError::ServerError(error.clone()));
        }

        Ok(message)
    }

    // MARK: - Genre APIs

    /// ジャンルの一覧を取得
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use shishutukan::ShishutsukanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// let genres = client.get_genres().await?;
    /// for genre in genres {
    ///     println!("{}: {}", genre.id, genre.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_genres(&self) -> Result<Vec<GenreWithId>> {
        let url = format!("{}/genres", self.base_url);

        let response = self.client.get(&url).send().await?;

        self.validate_response(&response)?;

        let genres: Vec<GenreWithId> = response.json().await?;
        Ok(genres)
    }

    /// ジャンルを追加
    ///
    /// # Arguments
    ///
    /// * `genre` - 追加するジャンルデータ
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use shishutukan::{ShishutsukanClient, Genre};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// let genre = Genre::new("娯楽費".to_string());
    /// let result = client.add_genre(&genre).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_genre(&self, genre: &Genre) -> Result<ApiMessage> {
        let url = format!("{}/genres", self.base_url);

        let response = self.client.post(&url).json(genre).send().await?;

        self.validate_response(&response)?;

        let message: ApiMessage = response.json().await?;
        if let Some(error) = message.error.as_ref() {
            return Err(ShishutsukanError::ServerError(error.clone()));
        }

        Ok(message)
    }

    /// ジャンルを削除
    ///
    /// # Arguments
    ///
    /// * `id` - 削除するジャンルのID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use shishutukan::ShishutsukanClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ShishutsukanClient::new("http://localhost:8000");
    /// let result = client.delete_genre(7).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_genre(&self, id: i32) -> Result<ApiMessage> {
        let url = format!("{}/genres/{}", self.base_url, id);

        let response = self.client.delete(&url).send().await?;

        self.validate_response(&response)?;

        let message: ApiMessage = response.json().await?;
        if let Some(error) = message.error.as_ref() {
            return Err(ShishutsukanError::ServerError(error.clone()));
        }

        Ok(message)
    }

    // MARK: - Helper Methods

    fn validate_response(&self, response: &reqwest::Response) -> Result<()> {
        let status = response.status();
        if !status.is_success() {
            return Err(ShishutsukanError::HttpError(status.as_u16()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ShishutsukanClient::new("http://localhost:8000");
        assert_eq!(client.base_url, "http://localhost:8000");
    }

    #[test]
    fn test_expense_model() {
        let expense = Expense::new("2025-01-15".to_string(), "食費".to_string(), 1000);
        assert_eq!(expense.date, "2025-01-15");
        assert_eq!(expense.genre, "食費");
        assert_eq!(expense.amount, 1000);
    }

    #[test]
    fn test_genre_model() {
        let genre = Genre::new("娯楽費".to_string());
        assert_eq!(genre.name, "娯楽費");
    }
}
