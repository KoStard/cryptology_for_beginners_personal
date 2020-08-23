use crate::constants::functions::alphabet::{index_to_letter, letter_to_index};

// Using 2x2 matrix
pub struct HillDigraphCipher {
    key: Vec<i32>,
    inverse: Vec<i32>
}

impl HillDigraphCipher {
    pub fn new(key: [i32; 4]) -> Result<Self, String> {
        let key: Vec<i32> = key.iter().map(|x| x % 26).collect();
        let inverse = Self::get_inverse(key.clone())?;
        Ok(HillDigraphCipher {
            key,
            inverse
        })
    }

    fn get_inverse(matrix: Vec<i32>) -> Result<Vec<i32>, String> {
        let determinant = Self::get_determinant(&matrix);
        if determinant % 2 == 0 || determinant == 13 {
            return Err("Invalid determinant, can't find the inverse of it!".to_owned());
        }
        let determinant_inverse = Self::get_multiplicative_inverse(determinant).unwrap();
        let matrix_for_inverting = Self::prepare_matrix_for_inverting(matrix.clone());
        Ok(Self::multiply_matrix_by_number(matrix_for_inverting, determinant_inverse))
    }

    fn get_determinant(matrix: &Vec<i32>) -> i32 {
        Self::mod_to_range(matrix[0] * matrix[3] - matrix[1] * matrix[2])
    }

    fn mod_to_range(n: i32) -> i32 {
        (n % 26 + 26) % 26
    }

    fn get_multiplicative_inverse(a: i32) -> Option<i32> {
        (1..26i32).find(|x| x * a % 26 == 1)
    }

    fn prepare_matrix_for_inverting(matrix: Vec<i32>) -> Vec<i32> {
        vec![
            Self::mod_to_range(matrix[3]),
            Self::mod_to_range(-matrix[1]),
            Self::mod_to_range(-matrix[2]),
            Self::mod_to_range(matrix[0])
        ]
    }

    fn multiply_matrix_by_number(matrix: Vec<i32>, factor: i32) -> Vec<i32> {
        matrix.iter().map(|x| Self::mod_to_range(x * factor)).collect()
    }

    pub fn encrypt(&self, message: String) -> String {
        let message = Self::prepare_message(message);
        message
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|c| {
                let a = letter_to_index(c[0]) as i32;
                let b = letter_to_index(c[1]) as i32;
                self.encrypt_pair(a, b).into_iter()
            })
            .flatten()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|ch| ch.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn encrypt_pair(&self, a: i32, b: i32) -> Vec<char> {
        let enc_a = ((self.key[0] * a + self.key[1] * b) % 26) as u8;
        let enc_b = ((self.key[2] * a + self.key[3] * b) % 26) as u8;
        vec![index_to_letter(enc_a), index_to_letter(enc_b)]
    }

    fn prepare_message(message: String) -> String {
        let message = message
            .to_uppercase()
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<String>();
        if message.len() % 2 != 0 {
            message + "X"
        } else {
            message
        }
    }

    pub fn decrypt(&self, encrypted_message: String) -> Result<String, String> {
        if encrypted_message.len() % 2 != 0 {
            return Err("Invalid length of the message!".to_owned());
        }
        Ok(encrypted_message
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|c| {
                let enc_a = letter_to_index(c[0]) as i32;
                let enc_b = letter_to_index(c[1]) as i32;
                self.decrypt_pair(enc_a, enc_b).into_iter()
            })
            .flatten()
            .collect())
    }

    fn decrypt_pair(&self, enc_a: i32, enc_b: i32) -> Vec<char> {
        let a = ((self.inverse[0] * enc_a + self.inverse[1] * enc_b) % 26) as u8;
        let b = ((self.inverse[2] * enc_a + self.inverse[3] * enc_b) % 26) as u8;
        vec![index_to_letter(a), index_to_letter(b)]
    }
}

#[cfg(test)]
mod hill_digraph_test {
    use crate::polyalphabetic::hill_digraph::manipulations::HillDigraphCipher;

    #[test]
    fn encrypt_works() {
        let cipher = HillDigraphCipher::new([5, 3, 11, 8]).unwrap();
        assert_eq!(cipher.encrypt("book".to_owned()), "CLDS".to_owned());
    }

    #[test]
    fn test() {
        println!("{:?}", HillDigraphCipher::get_inverse(vec![5, 7, 7, 10]));
    }

    #[test]
    fn get_inverse_works() {
        assert_eq!(
            HillDigraphCipher::get_inverse(vec![40, 61, 27, 21]),
            Ok(vec![5, 9, 1, 12])
        )
    }

    #[test]
    fn decrypt_works() {
        let cipher = HillDigraphCipher::new([4, 5, 3, 6]).unwrap();
        assert_eq!(cipher.encrypt("go".to_owned()), "YG".to_owned());
        assert_eq!(cipher.decrypt("YG".to_owned()), Ok("GO".to_owned()));
    }
}
