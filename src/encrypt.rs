use rand::Rng;

// encryption
pub fn encrypt(message: String) -> String {
    let mut rng = rand::thread_rng();

    let random_number: i32 = rng.gen_range(1..=64);

    println!("This is the random number: {}", random_number);

    let mut char_array: Vec<char> = message.chars().collect();
    for character in char_array.iter_mut() {
        *character = char_adder(*character, random_number);
    }

    let encrypt_mess:String = char_array.into_iter().collect();
    let result:String;
    if random_number < 10 {
        result = format!("{}0{}", encrypt_mess, random_number);
    }
    else{
        result = format!("{}{}", encrypt_mess, random_number);
    }


    result
}

// add int to char
pub fn char_adder(curr_char: char, amount: i32) -> char {
    let mut_char = std::char::from_u32(((curr_char as i32) + amount) as u32).unwrap_or(' ');

    mut_char
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_adder_test() {
        let result = char_adder('a', 1);
        assert_eq!(result, 'b');
    }

    #[test]
    fn char_adder_null_test(){
        let result = char_adder('\0', 1);
        assert_eq!(result, '');
    }

    #[test]
    fn char_adder_emoji_test(){
        let result = char_adder('😀', 1);
        assert_eq!(result, '😁');
    }

    #[test]
    fn char_adder_ten_test(){
        let result = char_adder('a', 10);
        assert_eq!(result, 'k');
    }

    #[test]
    fn char_adder_negative_test(){
        let result = char_adder('k', -10);
        assert_eq!(result, 'a');
    }






}