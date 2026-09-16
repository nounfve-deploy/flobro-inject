use crate::misc::random_hex;
pub struct SteamUser {
    pub id: String,
    pub nickname: String,
}

impl SteamUser {
    #[cfg(target_os = "linux")]
    pub fn get() -> Self {
        Self::anonymous()
    }

    pub fn anonymous() -> Self {
        Self {
            id: format!("anonymous:{}", random_hex::<4>()),
            nickname: String::new(),
        }
    }
}
