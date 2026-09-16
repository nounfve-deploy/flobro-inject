use rand::RngExt;

pub fn random_hex<const T: usize>() -> String {
    let mut buf = [0u8; T];
    rand::rng().fill(&mut buf);
    hex::encode(buf).to_string()
}
