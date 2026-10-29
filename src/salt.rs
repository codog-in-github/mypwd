use std::{
    env, fs, io,
    path::{Path, PathBuf},
    sync::LazyLock,
};

static SALT_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let home = env::home_dir().expect("无法获取用户主目录");
    home.join(".mypwd").join("salt")
});

fn write_salt_to(salt_content: &str, file: &Path) -> io::Result<()> {
    fs::write(file, salt_content)
}

fn read_salt_from(file: &Path) -> io::Result<String> {
    fs::read_to_string(file)
}

fn salt_file_exists(file: &Path) -> bool {
    file.is_file()
}

fn generate_salt(length: usize) -> String {
    let mut salt = String::with_capacity(length);

    for _ in 0..length {
        let character = rand::random_range(b'!'..=b'~');
        salt.push(character as char);
    }

    salt
}

pub(crate) fn init_salt(salt: Option<&str>, force: bool) -> io::Result<String> {
    let file = SALT_FILE_PATH.as_path();
    if !force && salt_file_exists(file) {
        return read_salt_from(file);
    }

    let salt = match salt {
        Some(value) => value.to_string(),
        None => generate_salt(32),
    };

    fs::create_dir_all(file.parent().expect("盐文件路径应包含父目录"))?;
    write_salt_to(&salt, file)?;
    Ok(salt)
}
