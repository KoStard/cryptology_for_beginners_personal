fn manipulate(message: String, shift: u8) -> String {
    (&message)
        .chars()
        .filter(|x| x.is_ascii() && x.is_alphabetic())
        .map(|x| {
            (((x.to_uppercase().next().unwrap() as u8) - 'A' as u8 + shift) % 26 + 'A' as u8)
                as char
        })
        .collect()
}

pub fn encrypt(message: String, key: u8) -> String {
    manipulate(message, key)
        .chars()
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|ch| ch.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decrypt(message: String, key: u8) -> String {
    manipulate(message, 26 - key)
}

#[cfg(test)]
mod caesar_tests {
    use crate::additive::caesar::manipulations::{decrypt, encrypt};

    #[test]
    fn encrypt_works_and_chunks_are_created() {
        assert_eq!(
            encrypt("some message".into(), 3),
            "VRPHP HVVDJ H".to_owned()
        );
    }

    #[test]
    fn encrypt_skips_non_ascii_characters() {
        assert_eq!(
            encrypt("текст and some other text".to_owned(), 3),
            "DQGVR PHRWK HUWHA W".to_owned()
        );
    }

    #[test]
    fn decrypt_works() {
        assert_eq!(
            decrypt("VRPHP HVVDJ H".to_owned(), 3),
            "SOMEMESSAGE".to_owned()
        );
    }

    #[test]
    fn getting_same_message_after_encrypting_and_decrypting() {
        assert_eq!(
            decrypt(encrypt("some message".to_owned(), 3), 3),
            "SOMEMESSAGE".to_owned()
        );
    }
}
