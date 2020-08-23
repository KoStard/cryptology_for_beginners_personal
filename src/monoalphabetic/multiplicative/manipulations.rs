use crate::constants::functions::alphabet::{index_to_letter, letter_to_index};

pub struct MultiplicativeCipher {
    pub factor: u32,
    pub decryption_key: u32
}

impl MultiplicativeCipher {
    pub fn new(factor: u32) -> Result<Self, String> {
        if !Self::verify_key(factor) {
            return Err("Invalid key!".to_owned());
        }
        let decryption_key = Self::get_decryption_key(factor).unwrap();
        Ok(MultiplicativeCipher {factor, decryption_key})
    }

    fn verify_key(key: u32) -> bool {
        // For these keys we don't have inverses and won't be able to decrypt the message
        if key % 2 == 0 || key % 13 == 0 {
            false
        } else {
            true
        }
    }

    fn get_decryption_key(key: u32) -> Option<u32> {
        (1..26u32).find(|x| x * key % 26 == 1)
    }

    pub fn encrypt(&self, message: String) -> String {
        message
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| index_to_letter(((letter_to_index(c) as u32 * self.factor) % 26) as u8))
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn decrypt(&self, encrypted_message: String) -> String {
        encrypted_message
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| index_to_letter(((letter_to_index(c) as u32 * self.decryption_key) % 26) as u8))
            .collect()
    }
}


#[cfg(test)]
mod multiplicative_tests {
    use crate::monoalphabetic::multiplicative::manipulations::MultiplicativeCipher;

    #[test]
    fn fails_with_invalid_keys() {
        assert!(MultiplicativeCipher::new(2).is_err());
        assert!(MultiplicativeCipher::new(4).is_err());
        assert!(MultiplicativeCipher::new(13).is_err());
        assert!(MultiplicativeCipher::new(26).is_err());
    }

    #[test]
    fn encryption_succeeds() {
        MultiplicativeCipher::new(3)
            .unwrap()
            .encrypt("Some message to you".to_owned());
    }

    #[test]
    fn decrypt_succeeds() {
        MultiplicativeCipher::new(3)
            .unwrap()
            .decrypt("ESMOM OEECU OHSWS K".to_owned());
    }

    #[test]
    fn manipulations_work() {
        let initial_message = "This is some weird message".to_owned();
        let cipher = MultiplicativeCipher::new(3).unwrap();
        let encrypted_message = cipher.encrypt(initial_message.clone());
        assert_eq!(encrypted_message, "HXAEA EESMO QOABL MOEEC UO".to_owned());
        let decrypted_message = cipher.decrypt(encrypted_message.clone());
        assert_eq!(decrypted_message, "THISISSOMEWEIRDMESSAGE".to_owned());
    }
}
