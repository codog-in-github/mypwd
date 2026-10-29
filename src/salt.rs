use std::{
    env, fs,
    io::{self, Write},
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

fn backup_salt(file: &Path) -> io::Result<()> {
    let datetime = chrono::Local::now().format("%Y%m%d-%H%M%S%.9f");
    let backup_path = file.with_file_name(format!("salt.{datetime}.bak"));
    let content = fs::read(file)?;
    let mut backup = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(backup_path)?;
    backup.write_all(&content)
}

fn salt_file_exists(file: &Path) -> bool {
    file.is_file()
}

pub(crate) fn clear_salt_history() -> io::Result<()> {
    let directory = SALT_FILE_PATH.parent().expect("盐文件路径应包含父目录");
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };

    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let Some(datetime) = name
            .to_str()
            .and_then(|name| name.strip_prefix("salt."))
            .and_then(|name| name.strip_suffix(".bak"))
        else {
            continue;
        };
        if datetime.len() == 25
            && chrono::NaiveDateTime::parse_from_str(datetime, "%Y%m%d-%H%M%S%.9f").is_ok()
            && entry.file_type()?.is_file()
        {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
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
    if force && salt_file_exists(file) {
        backup_salt(file)?;
    }
    write_salt_to(&salt, file)?;
    Ok(salt)
}
