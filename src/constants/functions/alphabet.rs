// 1 is A, 0 is Z
pub fn index_to_letter(index: u8) -> char {
    ('A' as u8 + (index + 25) % 26) as char
}

pub fn letter_to_index(letter: char) -> u8 {
    (letter.to_uppercase()
        .next()
        .unwrap() as u8 - 'A' as u8 + 1) % 26
}