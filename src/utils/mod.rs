use std::fs;

pub fn get_secret() -> Option<String> {
    let secret = home::home_dir().unwrap().join(".chadgpt").join("secret.txt");
    fs::read_to_string(secret).ok()
}

pub fn write_secret(secret: &str) -> Result<(), std::io::Error> {
    let secret_path = home::home_dir().unwrap().join(".chadgpt").join("secret.txt");
    fs::create_dir_all(secret_path.parent().unwrap())?;
    fs::write(secret_path, secret)
}