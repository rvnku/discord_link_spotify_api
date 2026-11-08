use crate::errors::AppError;
use serde::{Serialize, de::DeserializeOwned};
use tower_sessions::Session;

#[async_trait::async_trait]
pub trait SessionExt {
    async fn set<T: Serialize + Send>(&self, key: &str, value: T) -> Result<(), AppError>;

    async fn take<T: DeserializeOwned + Send + 'static>(
        &self,
        key: &str,
    ) -> Result<Option<T>, AppError>;

    async fn take_matching(&self, key: &str, expected: &str, err: AppError)
    -> Result<(), AppError>;
}

#[async_trait::async_trait]
impl SessionExt for Session {
    async fn set<T: Serialize + Send>(&self, key: &str, value: T) -> Result<(), AppError> {
        self.insert(key, value)
            .await
            .map_err(|e| AppError::Session(e.to_string()))
    }

    async fn take<T: DeserializeOwned + Send + 'static>(
        &self,
        key: &str,
    ) -> Result<Option<T>, AppError> {
        self.remove::<T>(key)
            .await
            .map_err(|e| AppError::Session(e.to_string()))
    }

    async fn take_matching(
        &self,
        key: &str,
        expected: &str,
        err: AppError,
    ) -> Result<(), AppError> {
        self.remove::<String>(key)
            .await
            .map_err(|e| AppError::Session(e.to_string()))?
            .filter(|actual| actual == expected)
            .map(|_| ())
            .ok_or(err)
    }
}
