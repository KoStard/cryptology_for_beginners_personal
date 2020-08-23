use crate::monoalphabetic::affine::manipulations::AffineCipher;
use crate::constants::functions::alphabet::{letter_to_index};
use std::collections::{HashMap, HashSet};


pub struct AffineCipherHack {
    encrypted_message: String,
}

impl AffineCipherHack {
    pub fn new(encrypted_message: String) -> Self {
        AffineCipherHack { encrypted_message: encrypted_message.chars().filter(|c|c.is_alphabetic()).collect() }
    }

    pub fn get_all_options_based_on_common_letters(&self) -> Vec<(u32, u32, String)> {
        self.get_all_options_based_on_common_letters_with_depth(1)
    }

    pub fn get_all_options_based_on_common_letters_with_depth(&self, depth: usize) -> Vec<(u32, u32, String)> {
        self.find_most_common_letter(depth)
            .iter()
            // this can be changed to check the T letter too!
            .map(|c| self.try_with_guess(letter_to_index(*c) as u32, letter_to_index('e') as u32))
            .flatten()
            .collect()
    }

    fn find_most_common_letter(&self, depth: usize) -> Vec<char> {
        let mut map: HashMap<char, u32> = HashMap::new();
        self.encrypted_message
            .chars()
            .for_each(|c| if map.contains_key(&c) {
                map.insert(c, map.get(&c).unwrap() + 1);
            } else {
                map.insert(c, 1);
            });
        let mut sorted_map = map.into_iter()
            .collect::<Vec<(char, u32)>>();
        sorted_map.sort_by(|a, b| a.1.cmp(&b.1));
        let mut best_ones = HashSet::new();
        sorted_map
            .iter()
            .rev()
            .for_each(|x| if best_ones.len() < depth {
                best_ones.insert(x.1);
            });
        sorted_map.iter()
            .filter(|x| best_ones.contains(&x.1))
            .map(|x| x.0)
            .collect()
    }

    /**
    When you have a guess that can be used to decipher the message, give it here
    encrypted - the letter alphabetic index
    guess - the guessed value alphabetic index
    */
    pub fn try_with_guess(&self, encrypted: u32, guess: u32) -> Vec<(u32, u32, String)> {
        (1..26)
            .filter(|a| a % 2 != 0 && *a != 13)
            .map(|a| (a, Self::get_b(encrypted, guess, a)))
            .map(|(a, b)| (a, b, self.check_a_and_b(a, b)))
            .collect()
    }

    fn check_a_and_b(&self, a: u32, b: u32) -> String {
        AffineCipher::new(a, b)
            .unwrap()
            .decrypt(self.encrypted_message.clone())
    }

    fn get_b(encrypted: u32, guess: u32, a: u32) -> u32 {
        (encrypted + 26 - (a * guess) % 26) % 26
    }
}

#[cfg(test)]
mod affine_cipher_hack_test {
    use crate::monoalphabetic::affine::hacking::AffineCipherHack;
    use crate::constants::functions::alphabet::letter_to_index;

    #[test]
    fn test() {
        /*
        QPIFY EPKLX YYPRX XYSXX UXWSV IYRTS XIHPF YVNXH PFKYK ZWYPY SXOPP
        7 15 DONUTFORGETTOSEETHEELEPHANTSWHENYOUTAKEYOURTRIPTOTHEZOO

        AEKHF FYOKK FJKHK EOFUX OEUXO QYNAK LWFAT ATHFK HRYMA FHKUX ZMUOQ WCJUO FJKKU XZAKX YHKO
        17 4 IWENTTOSEETHENEWSTARSWARSMOVIEBUTIDIDNTENJOYITNEARLYASMUCHASTHEEARLIERONES

        FVMHA FMCFL MZCYQ NQPFV MCGVQ QHNMO CTATM ZGQNF AYNMJ
        AHAZS MMZZQ ZFVAF GAICM JYFFQ DMZMT ZYNFM JGQCF YNSFV
        MGHID AHQFQ PKQNM U
        3 24 THELATESTVERSIONOFTHESCHOOLNEWSPAPERCONTAINEDALARGEERRORTHATCAUSEDITTOBEREPRINTEDCOSTINGTHECLUBALOTOFMONEY

        NTYNC NSOGN XGNGQ NSNSN UIGEX GFNXG NGSMX GTUQZ TGQGF
        NQCNX SNXGM SFNSO GWKGQ NUCFQ
        23 22 TRYTOTAKETHETESTATATIMEWHENTHETEACHERISPRESENTSOTHATHECANTAKEQUESTIONS
        */
        for message in
            AffineCipherHack::new("NTYNC NSOGN XGNGQ NSNSN UIGEX GFNXG NGSMX GTUQZ TGQGF
NQCNX SNXGM SFNSO GWKGQ NUCFQ".to_owned())
                .get_all_options_based_on_common_letters_with_depth(1) {
            println!("{} {} {}", message.0, message.1, message.2);
        }
    }

    #[test]
    fn try_with_guess_contains_correct_option() {
        assert!(AffineCipherHack::new("RPIID XHIGGG MPOOH UIQVA GONIV QDXYI PQNII AEPRY IIWGOT T".to_owned())
            .try_with_guess(letter_to_index('I') as u32, letter_to_index('e') as u32)
            .contains(&(11, 6, "THEENDLESSSCHOOLYEARISOVERANDWEHAVEEIGHTWEEKSOFF".to_owned())))
    }

    #[test]
    fn find_most_common_letters_works() {
        assert_eq!(AffineCipherHack::new("Hello there".to_owned())
                       .find_most_common_letter(1), vec!['e']);
    }
}
