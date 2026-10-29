const CHARSET: &str =
    "ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub(crate) fn md5_with_salt(content: &str, salt: &str) -> md5::Digest {
    let salted_content = format!("{content}{salt}");
    md5::compute(salted_content)
}

pub(crate) fn md5_to_charset(digest: md5::Digest) -> String {
    let charset = CHARSET.as_bytes();
    let mut result = String::with_capacity(16);

    for byte in digest.0 {
        let index = byte as usize % charset.len();
        result.push(charset[index] as char);
    }

    result
}
