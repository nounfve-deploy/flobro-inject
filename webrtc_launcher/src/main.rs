fn main() {
    let cmd = path::absolute("./xbin/flobro").unwrap();
    let cmd = ensure_executable(cmd).unwrap();

    let user = SteamUser::get();
    let random_unique = random_hex::<8>();
    let inject_path = path::absolute(format!("main.{random_unique}.js")).unwrap();

    {
        // write to inject main
        let inject = INJECT_TEMPLATE
            .replace("$__ID__", &user.id)
            .replace("$__NICKNAME__", &user.nickname);
        fs::write(&inject_path, &inject).unwrap();
    }
    {
        let room = format!("{ROOM_HOST}/{random_unique}");
        let room = urlencoding::encode(&room);
        let inject = inject_path.to_string_lossy().into_owned();
        let inject = urlencoding::encode(&inject);

        // spawn flobro
        Command::new(cmd)
            .arg(format!("flobro://open?url={room}&inject={inject}"))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
    }
}

const ROOM_HOST: &str = "https://p2p.mirotalk.com/join";
const INJECT_TEMPLATE: &str = include_str!("../../dist/main.js");

pub mod misc;
pub mod steam_user;
use std::{
    fs,
    path::{self},
    process::{Command, Stdio},
};

use crate::{
    misc::{ensure_executable, random_hex},
    steam_user::SteamUser,
};
