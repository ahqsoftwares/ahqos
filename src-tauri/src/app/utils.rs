use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub enum UserType {
    Administrator,
    General,
    Restricted,
    Guest
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub role: UserType,
    pub passwd: Vec<u8>,
    pub uid: u8,
    pub theme: Theme
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub apps_theme: ThemeName,
    pub tskbr_theme: ThemeName
}

#[derive(Serialize, Deserialize)]
pub enum ThemeName {
    Dark,
    Light
}

pub type Users = Vec<User>;

pub fn default_user() -> User {
    User {
        name: "Administrator".into(),
        role: UserType::Administrator,
        passwd: vec![],
        uid: 0,
        theme: Theme {
            apps_theme: ThemeName::Light,
            tskbr_theme: ThemeName::Dark
        }
    }
}