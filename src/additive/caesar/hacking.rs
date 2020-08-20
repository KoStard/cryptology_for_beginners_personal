use crate::additive::caesar::manipulations::decrypt;

pub struct CaesarHack {
    encrypted_message: String
}

impl CaesarHack {
    pub fn new(encrypted_message: String) -> Self {
        CaesarHack {
            encrypted_message
        }
    }
    pub fn get_all_variants(self) -> Vec<(u8, String)> {
        (1..26)
            .map(|i| (i, decrypt(self.encrypted_message.clone(), i)))
            .collect()
    }
    pub fn print_all_variants(self) {
        self.get_all_variants()
            .iter()
            .for_each(|(i, t)| println!("{}: {}", i, t));
    }
}


#[cfg(test)]
mod caesar_hack_test {
    use crate::additive::caesar::hacking::CaesarHack;

    #[test]
    fn get_all_variant_contains_correct_value() {
        assert!(CaesarHack::new("QUPCV OZGTM BAOMB IXQHH I".to_owned())
            .get_all_variants()
            .contains(&(8, "IMHUNGRYLETSGETAPIZZA".to_owned())));
    }

    #[test]
    fn print_all_variant_works() {
        CaesarHack::new("QUPCV OZGTM BAOMB IXQHH I".to_owned())
            .print_all_variants();
    }

    #[test]
    fn task() {
        // 6: THECARIWOULDMOSTLIKETOHAVEISAPORSCHE
        // CaesarHack::new("ZNKIG XOCUA RJSUY ZROQK ZUNGB KOYGV UXYIN K".to_owned())
        //     .print_all_variants();

        // 4: WHENYOUGETTHEANSWERTOPROBLEMNINEPLEASECALLME
        // CaesarHack::new("KVSBM CIUSH HVSOB GKSFH CDFCP ZSABW BSDZS OGSQO ZZAS".to_owned())
        //     .print_all_variants();

        // 18: DONTFORGETTOBUYTHETICKETSTOTHECONCERT
        // CaesarHack::new("VGFLX GJYWL LGTMQ LZWLA UCWLK LGLZW UGFUW JL".to_owned())
        //     .print_all_variants();

        // 24: WHENTHEDEFENSEBLITZESWEWILLTHROWASCREENPASS
        // CaesarHack::new("UFCLR FCBCD CLQCZ JGRXC QUCUG JJRFP MUYQA PCCLN YQQ".to_owned())
        //     .print_all_variants();

        // 13: WEPLANTOVACATIONINBERMUDAINSTEADOFFLORIDATHISYEAR
        // CaesarHack::new("JRCYN AGBIN PNGVB AVAOR EZHQN VAFGR NQBSS YBEVQ NGUVF LRNE".to_owned())
        //     .print_all_variants();
    }
}

