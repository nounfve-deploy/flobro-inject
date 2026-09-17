use crate::misc::random_hex;

#[derive(Debug)]
pub struct SteamUser {
    pub id: String,
    pub nickname: String,
}

impl SteamUser {
    #[cfg(target_os = "linux")]
    pub fn get() -> Self {
        Self::anonymous()
    }

    #[cfg(target_os = "windows")]
    pub fn get() -> Self {
        if let Some(id) = Self::uid_from_registry() {
            Self {
                id: format!("steam:{id}"),
                nickname: String::new(),
            }
        } else {
            Self {
                id: String::new(),
                nickname: "nologin".into(),
            }
        }
    }

    pub fn anonymous() -> Self {
        Self {
            id: format!("anonymous:{}", random_hex::<4>()),
            nickname: String::new(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn uid_from_registry() -> Option<u64> {
        use winreg::RegKey;
        use winreg::enums::HKEY_CURRENT_USER;
        let key = RegKey::predef(HKEY_CURRENT_USER);
        let key = key
            .open_subkey(r"SOFTWARE\Valve\Steam\ActiveProcess")
            .ok()?;
        let val = key.get_raw_value("ActiveUser").ok()?.to_string();
        if let id = val.parse::<u64>().ok()?
            && id != 0
        {
            Some(id)
        } else {
            None
        };
    }
}
