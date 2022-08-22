use crate::{
    api::{AccountInfo, AccountStatus},
    Binance,
};
use anyhow::{anyhow, Result};

impl Binance {
    /// Get current account information.
    /// # Errors
    /// Returns [`Err`] when binance api fails
    /// Needs api key
    pub async fn get_user_account_info(
        &self,
        _timestamp: u64,
        _recv_window: Option<u64>,
    ) -> Result<AccountInfo> {
        todo!()
    }

    /// Fetches account status details.
    /// # Errors
    /// Returns [`Err`] when binance api fails
    /// Needs api key
    pub async fn get_user_account_status(&self, _timestamp: u64) -> Result<AccountStatus> {
        todo!()
    }

    /// Fetches account status details.
    /// # Errors
    /// Returns [`Err`] when binance api fails
    /// Needs api key
    pub async fn get_user_api_trading_status(
        &self,
        _timestamp: u64,
        _recv_window: Option<u64>,
    ) -> Result<()> {
        todo!()
    }
}
