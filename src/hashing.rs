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

// 输入为 md5_to_charset 生成的 16 位 ASCII 字符串。
pub(crate) fn ensure_character_mix(password: String, digest: md5::Digest) -> String {
    let category = |byte: u8| {
        if byte.is_ascii_alphabetic() {
            0
        } else if byte.is_ascii_digit() {
            1
        } else {
            2
        }
    };
    let mut bytes = password.into_bytes();
    let mut counts = [0; 3];
    for &byte in &bytes {
        counts[category(byte)] += 1;
    }

    for missing in 0..3 {
        if counts[missing] != 0 {
            continue;
        }

        let positions: Vec<usize> = (0..bytes.len())
            .filter(|&index| counts[category(bytes[index])] > 1)
            .collect();
        let characters: Vec<u8> = CHARSET
            .bytes()
            .filter(|&byte| category(byte) == missing)
            .collect();
        let position = positions[digest.0[missing] as usize % positions.len()];
        let character = characters[digest.0[missing + 3] as usize % characters.len()];

        counts[category(bytes[position])] -= 1;
        bytes[position] = character;
        counts[missing] += 1;
    }

    bytes.into_iter().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_character_mix(password: &str) {
        assert_eq!(password.len(), 16);
        assert!(password.bytes().any(|byte| byte.is_ascii_alphabetic()));
        assert!(password.bytes().any(|byte| byte.is_ascii_digit()));
        assert!(password.bytes().any(|byte| byte.is_ascii_punctuation()));
        assert!(
            password
                .chars()
                .all(|character| CHARSET.contains(character))
        );
    }

    #[test]
    fn fills_missing_categories_without_removing_unique_categories() {
        let digest = md5_with_salt("text", "salt");
        for original in [
            "AAAAAAAAAAAAAAAA",
            "2222222222222222",
            "!!!!!!!!!!!!!!!!",
            "A222222222222222",
            "!AAAAAAAAAAAAAAA",
            "2!!!!!!!!!!!!!!!",
        ] {
            let result = ensure_character_mix(original.to_string(), digest);
            assert_character_mix(&result);
            assert_eq!(result, ensure_character_mix(original.to_string(), digest));
        }
    }

    #[test]
    fn preserves_password_that_already_has_all_categories() {
        let original = "Abcdefghjkmnpq2!";
        let digest = md5_with_salt("text", "salt");
        assert_eq!(ensure_character_mix(original.to_string(), digest), original);
    }

    #[test]
    fn generated_passwords_have_all_categories() {
        for index in 0..256 {
            let digest = md5_with_salt(&index.to_string(), "salt");
            let result = ensure_character_mix(md5_to_charset(digest), digest);
            assert_character_mix(&result);
        }
    }
}
