mod os;
mod init_dirs;
mod utils;

use std::fs;
use serde_json::{
    from_str,
    to_string
};

pub use os::*;
pub use init_dirs::*;
pub use utils::*;

#[tauri::command(async)]
pub fn get_user_config() -> Option<Users> {
    let users_file = format!(
        "{}/users.json", get_programdata()?
    );

    let string = fs::read_to_string(&users_file).unwrap_or("l".into());

    if let Ok(data) = from_str::<Users>(&string) {
        return Some(data);
    } else {
        let users: Users = vec![default_user()];

        fs::write(&users_file, to_string(&users).ok()?).ok()?;

        return Some(users);
    }
}