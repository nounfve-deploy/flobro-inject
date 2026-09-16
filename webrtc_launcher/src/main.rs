fn main() {
    let cmd = path::absolute("./xbin/flobro").unwrap();
    if !cmd.is_file() {
        panic!("cannot find xbin/flobro in current dir")
    }
    let user = SteamUser::get();
    let inject = INJECT_TEMPLATE
        .replace("$__ID__", &user.id)
        .replace("$__NICKNAME__", &user.nickname);

    {
        // write to inject main
        let path = path::absolute("./script_inject/main.js").unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &inject).unwrap();
    }
    {
        let room = format!("{ROOM_HOST}/{}", random_hex::<8>());
        let room = urlencoding::encode(&room);

        // spawn flobro
        let status = Command::new(cmd)
            .arg(format!("flobro://open?url={room}"))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap();
        println!("{status:?}")
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

use crate::{misc::random_hex, steam_user::SteamUser};
