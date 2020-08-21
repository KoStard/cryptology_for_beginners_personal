use crate::multiplicative::manipulations::MultiplicativeCipher;

pub struct MultiplicativeCipherHack {
    encrypted_message: String,
}

impl MultiplicativeCipherHack {
    pub fn new(encrypted_message: String) -> Self {
        MultiplicativeCipherHack { encrypted_message }
    }

    pub fn get_all_variants(&self) -> Vec<(u32, String)> {
        (1..26)
            .filter(|k| k % 2 != 0 && k % 13 != 0)
            .map(|possible_key| {
                (
                    possible_key,
                    MultiplicativeCipher::new(possible_key)
                        .unwrap()
                        .decrypt(self.encrypted_message.clone()),
                )
            })
            .collect()
    }

    pub fn print_all_variants(&self) {
        self.get_all_variants()
            .iter()
            .for_each(|(possible_key, message)| println!("{}: {}", possible_key, message));
    }
}

#[cfg(test)]
mod multiplicative_cipher_hack_test {
    use crate::multiplicative::hacking::MultiplicativeCipherHack;

    #[test]
    fn get_all_variants_contains_correct_option() {
        assert!(MultiplicativeCipherHack::new(
            "YQDIU SOWJG MGQQQ TWPGF UMGQX BGTKY FUUV".to_owned()
        )
        .get_all_variants()
        .contains(&(17u32, "CANYOUGIVEMEAAARIDEHOMEAFTERSCHOOL".to_owned())))
    }

    /*
    CIMOG FSXTS SIKDS OYCDD YCVRQ MEFSX TIVOC HNECV XO
    M 9: IAMSURETHEEAGLESWILLWINBYMORETHANSIXPOINTS

    KYVZE KVIEV KJZKV ZJNNN UFKDZ CBJYR BVUFK TFD
    A 17: THEINTERNETSITEISWWWDOTMILKSHAKEDOTCOM

    DLANA GIUQN AUDIL COCHD TDCHG QLDKL UHHAR IGJUD DAH \
    M 21: THEREISAGREATSHOWONTVTONIGHTCHANNELSIXATTEN

    HESGD ONVDQ FNDRN TSADR TQDSN STQMN EEZKK ZOOKH ZMBDR
    A 25: IFTHEPOWERGOESOUTBESURETOTURNOFFALLAPPLIANCES

    WQHWE QFWQF AQGRY DQLSY XQSGY NUKLV DVGFK VYARY NTKTT
    M 25: CIRCUITCITYISHAVINGABIGSALEONDVDSTODAYHALFOFF
    */
}
