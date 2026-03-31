use serde::{Deserialize, Serialize};
use steam_vent::auth::{
    AuthConfirmationHandler, DeviceConfirmationHandler, FileGuardDataStore,
    SharedSecretAuthConfirmationHandler,
};
use steam_vent::{Connection, ServerList};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SteamAuthError {
    #[error("server discovery failed: {0}")]
    Discovery(String),
    #[error("login failed: {0}")]
    Login(String),
    #[error("no access token received after login")]
    NoToken,
}

/// Credentials for Steam login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamCredentials {
    pub username: String,
    pub password: String,
    /// Base64-encoded shared secret for TOTP generation.
    pub shared_secret: Option<String>,
}

/// Result of a successful Steam login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub access_token: String,
    pub steam_id: String,
    pub account_name: String,
}

/// Perform a full Steam login via the Steam network protocol using `steam-vent`.
///
/// This replaces the previous HTTP-based polling approach which suffered from
/// infinite wait issues. The `steam-vent` library handles the full authentication
/// flow (RSA encryption, session creation, Steam Guard TOTP, and token exchange)
/// over the native Steam CM protocol with proper timeouts.
pub fn login(credentials: &SteamCredentials) -> Result<LoginResult, SteamAuthError> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| SteamAuthError::Login(format!("failed to create async runtime: {e}")))?;

    rt.block_on(login_async(credentials))
}

async fn login_async(credentials: &SteamCredentials) -> Result<LoginResult, SteamAuthError> {
    let server_list = ServerList::discover()
        .await
        .map_err(|e| SteamAuthError::Discovery(e.to_string()))?;

    let guard_data_store = FileGuardDataStore::new("steam_guard_tokens.json".into());

    let connection = match &credentials.shared_secret {
        Some(secret) => {
            let handler = SharedSecretAuthConfirmationHandler::new(secret)
                .or(DeviceConfirmationHandler);

            Connection::login(
                &server_list,
                &credentials.username,
                &credentials.password,
                guard_data_store,
                handler,
            )
            .await
            .map_err(|e| SteamAuthError::Login(e.to_string()))?
        }
        None => {
            Connection::login(
                &server_list,
                &credentials.username,
                &credentials.password,
                guard_data_store,
                DeviceConfirmationHandler,
            )
            .await
            .map_err(|e| SteamAuthError::Login(e.to_string()))?
        }
    };

    let access_token = connection
        .access_token()
        .ok_or(SteamAuthError::NoToken)?
        .to_string();

    let steam_id = connection.steam_id().steam64().to_string();

    Ok(LoginResult {
        access_token,
        steam_id,
        account_name: credentials.username.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_serialization() {
        let creds = SteamCredentials {
            username: "testuser".into(),
            password: "testpass".into(),
            shared_secret: Some("dGVzdHNlY3JldA==".into()),
        };
        let json = serde_json::to_string(&creds).unwrap();
        let parsed: SteamCredentials = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.username, "testuser");
        assert_eq!(parsed.shared_secret.as_deref(), Some("dGVzdHNlY3JldA=="));
    }

    #[test]
    fn test_login_result_serialization() {
        let result = LoginResult {
            access_token: "access_test".into(),
            steam_id: "76561198012345678".into(),
            account_name: "testuser".into(),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("access_token"));
        assert!(json.contains("76561198012345678"));
    }
}
