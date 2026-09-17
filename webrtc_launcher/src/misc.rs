use std::path::PathBuf;

use rand::RngExt;

pub fn random_hex<const T: usize>() -> String {
    let mut buf = [0u8; T];
    rand::rng().fill(&mut buf);
    hex::encode(buf).to_string()
}

pub fn ensure_executable(path: PathBuf) -> std::io::Result<PathBuf> {
    if !path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            path.to_string_lossy(),
        ));
    }
    #[cfg(unix)]
    {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        fs::set_permissions(&path, perms)?;
    }
    Ok(path)
}
