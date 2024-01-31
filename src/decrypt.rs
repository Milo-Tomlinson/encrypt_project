use crate::encrypt::char_adder;

pub fn decrypt(message: String) -> String {
    let mut char_array: Vec<char> = message.chars().collect();
    let ones_place:i32 = char_array.pop().unwrap().to_digit(10).unwrap() as i32;
    let tens_place:i32 = char_array.pop().unwrap().to_digit(10).unwrap() as i32;
    let amount:i32 = -1 * ((ones_place)+ (tens_place * 10));

    for character in char_array.iter_mut() {
        *character = char_adder(*character, amount);
    }

    let result = char_array.into_iter().collect();

    result

}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decryption_test() {
        let result = decrypt("Zw~~2i~v18".to_string());
        assert_eq!(result, "Hello World\r\n");
    }

    #[test]
    fn special_characters_decrypt_test() {
        let result = decrypt("181P:533DHIAD16".to_string());
        assert_eq!(result, "!(!@*%##48914\r\n");
    }

    #[test]
    fn emoji_decrypt_test() {
        let result = decrypt("😌😁😋01".to_string());
        assert_eq!(result, "😋😀😊\r\n");
    }

}