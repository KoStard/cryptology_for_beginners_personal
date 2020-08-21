use crate::constants::functions::alphabet::{index_to_letter, letter_to_index};

pub struct AffineCipher {
    // a*p+b = C mod 26 - encrypting
    pub a: u32,
    pub b: u32,
    // c*C + d = p mod 26 - decrypting
    pub c: u32,
    pub d: u32,
}

impl AffineCipher {
    pub fn new(a: u32, b: u32) -> Result<Self, String> {
        if !Self::verify_key(a) {
            return Err("Invalid key!".to_owned());
        }
        let (c, d) = Self::get_decryption_key(a, b).unwrap();
        Ok(AffineCipher { a, b, c, d })
    }

    fn verify_key(key: u32) -> bool {
        // For these keys we don't have inverses and won't be able to decrypt the message
        if key % 2 == 0 || key % 13 == 0 {
            false
        } else {
            true
        }
    }

    fn get_decryption_key(a: u32, b: u32) -> Option<(u32, u32)> {
        let inverse = Self::get_multiplicative_inverse(a)?;
        Some((
            inverse,
            ((((-((b * inverse) as i32)) % 26) + 26) % 26) as u32,
        ))
    }

    fn get_multiplicative_inverse(a: u32) -> Option<u32> {
        (1..26u32).find(|x| x * a % 26 == 1)
    }

    pub fn encrypt(&self, message: String) -> String {
        message
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| index_to_letter(((letter_to_index(c) as u32 * self.a + self.b) % 26) as u8))
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
            .map(|c| {
                index_to_letter(((letter_to_index(c) as u32 * self.c + self.d) % 26) as u8)
            })
            .collect()
    }
}

#[cfg(test)]
mod affine_test {
    use crate::affine::manipulations::AffineCipher;

    #[test]
    fn encrypt_works() {
        let cipher = AffineCipher::new(239, 152).unwrap();
        assert_eq!(cipher.encrypt("drink water".to_owned()), "PHONY GARUH".to_owned());
    }

    #[test]
    fn decrypt_works() {
        let cipher = AffineCipher::new(239, 152).unwrap();
        assert_eq!(cipher.decrypt("PHONY GARUH".to_owned()), "DRINKWATER".to_owned());
    }
}
