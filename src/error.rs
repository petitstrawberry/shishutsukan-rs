use thiserror::Error;

/// Shishutsukanクライアントのエラー型
#[derive(Error, Debug)]
pub enum ShishutsukanError {
    /// 無効なURL
    #[error("無効なURLです")]
    InvalidUrl,

    /// HTTPエラー
    #[error("HTTPエラー: {0}")]
    HttpError(u16),

    /// ネットワークエラー
    #[error("ネットワークエラー: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// デコードエラー
    #[error("デコードエラー: {0}")]
    DecodingError(#[from] serde_json::Error),

    /// サーバーエラー
    #[error("サーバーエラー: {0}")]
    ServerError(String),
}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, ShishutsukanError>;
