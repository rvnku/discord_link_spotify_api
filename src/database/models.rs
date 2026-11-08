use jiff::Timestamp;
use toasty::Model;

#[derive(Debug, Model)]
pub struct SpotifyToken {
    #[key]
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,
}
