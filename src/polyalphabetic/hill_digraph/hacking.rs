use crate::constants::functions::alphabet::letter_to_index;
use crate::polyalphabetic::hill_digraph::manipulations::HillDigraphCipher;

pub struct HillDigraphCipherHack {
    encrypted_message: String
}

impl HillDigraphCipherHack {
    pub fn new(encrypted_message: String) -> Self {
        HillDigraphCipherHack { encrypted_message: Self::prepare_message(encrypted_message) }
    }

    fn prepare_message(message: String) -> String {
        message
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect()
    }

    /// When you know what content the decrypted message can have, you can provide it as a crib,
    /// which can help to decrypt the message
    pub fn check_with_crib(&self, crib: String) -> Vec<([i32; 4], String)> {
        (0..=(self.encrypted_message.len() - crib.len()))
            .map(|i| self.check_position_with_crib(crib.clone(), i))
            .flatten()
            .collect()
    }

    fn check_position_with_crib(&self, mut crib: String, mut position: usize) -> Vec<([i32; 4], String)> {
        let initial_position = position.clone();
        let initial_crib = crib.clone();
        if position % 2 == 1 {
            position += 1;
            crib.remove(0);
        }
        if crib.len() % 2 == 1 {
            crib.remove(crib.len() - 1);
        }
        let all_checks = self.encrypted_message
            .chars()
            .skip(position)
            .take(crib.len())
            .map(letter_to_index)
            .collect::<Vec<u8>>()
            .chunks(2)
            .zip(crib.chars().map(letter_to_index).collect::<Vec<u8>>().chunks(2))
            .map(|(enc, crb)| {
                (
                    (crb[0], crb[1], enc[0]),  // a*ei + b*e(i+1) = ci
                    (crb[0], crb[1], enc[1]),  // c*ei + d*e(i+1) = c(i+1)
                )
            })
            .collect::<Vec<((u8, u8, u8), (u8, u8, u8))>>();
        let all_a_b_checks = all_checks.iter().map(|x| x.0).collect::<Vec<(u8, u8, u8)>>();
        let all_c_d_checks = all_checks.iter().map(|x| x.1).collect::<Vec<(u8, u8, u8)>>();

        let all_a_b_pairs = Self::get_all_pairs(&all_a_b_checks);
        let all_c_d_pairs = Self::get_all_pairs(&all_c_d_checks);

        let all_possible_keys = all_a_b_pairs
            .into_iter()
            .map(|(a, b)| {
                all_c_d_pairs
                    .clone()
                    .into_iter()
                    .map(move |(c, d)| {
                        (a, b, c, d)
                    })
                    .filter(|(a, b, c, d)|
                        Self::validate_key(*a, *b, *c, *d)
                    )
            })
            .flatten()
            .collect::<Vec<(i32, i32, i32, i32)>>();

        all_possible_keys
            .into_iter()
            .map(|(a, b, c, d)| {
                ([a, b, c, d], HillDigraphCipher::new([a, b, c, d])
                    .unwrap()
                    .decrypt(self.encrypted_message.clone()))
            })
            .filter(|x| x.1.is_ok())
            .map(|x| (x.0, x.1.unwrap()))
            .filter(|x| x.1.chars().skip(initial_position).take(initial_crib.len()).collect::<String>() == initial_crib)
            .collect()
    }

    fn validate_key(a: i32, b: i32, c: i32, d: i32) -> bool {
        let determinant = Self::get_determinant(a, b, c, d);
        determinant % 2 == 1 && determinant != 13
    }

    fn get_all_pairs(checks: &Vec<(u8, u8, u8)>) -> Vec<(i32, i32)> {
        (0..26)
            .map(|a| (0..26).map(move |b| (a, b)))
            .flatten()
            .filter(|(a, b)| {
                checks
                    .iter()
                    .all(|(e1, e2, c)| {
                        Self::mod_to_range(a * (*e1 as i32) + b * (*e2 as i32)) == *c as i32
                    })
            })
            .collect()
    }

    fn get_determinant(a: i32, b: i32, c: i32, d: i32) -> i32 {
        Self::mod_to_range(a * d - b * c)
    }

    fn mod_to_range(n: i32) -> i32 {
        (n % 26 + 26) % 26
    }
}

#[cfg(test)]
mod hill_digraph_cipher_hack {
    use crate::polyalphabetic::hill_digraph::hacking::HillDigraphCipherHack;

    #[test]
    fn test() {
        // CMOWL KURLO DPPMM GROBD UTOTF YSNIL HQ - SCHWA
        // IFMRSCHWASRTISABSENTLETSCUTCLASS - [7 3 3 2]
        // BQGIN CDMDN CXPSR XMYSX GZ - mall
        // MEETYOUATTHEMALLATNINE - [5 3 9 6]
        HillDigraphCipherHack::new("CMOWL KURLO DPPMM GROBD UTOTF YSNIL HQ".to_owned())
            .check_with_crib("SCHWA".to_uppercase().to_owned())
            .iter()
            .for_each(|x|
                println!("{} - [{} {} {} {}]", x.1, x.0[0], x.0[1], x.0[2], x.0[3]));
    }

    #[test]
    fn check_with_crib_contains_correct_option() {
        HillDigraphCipherHack::new("KMYEM UPAUO AHOJR YUKTT CACQC XXIYE DKSTQ ZXDAW".to_owned())
            .check_with_crib("STEVE".to_owned())
            .contains(&([5, 3, 9, 6], "IFSTEVEWANTSTOKEEPTHEJOBHEMUSTWORKHARDER".to_owned()));
    }
}
