use crate::encrypt::char_adder;

pub fn decrypt(message: String) -> String {
    let mut char_array: Vec<char> = message.chars().collect();
    let ones_place:i32 = char_array.pop().unwrap().to_digit(10).unwrap() as i32;
    let tens_place:i32 = char_array.pop().unwrap().to_digit(10).unwrap() as i32;
    let amount:i32 = -1 * ((ones_place)+ (tens_place * 10));
    //println!("Number dycr: {} num: {}", one_char, ones_place);
    for character in char_array.iter_mut() {
        *character = char_adder(*character, amount);
    }

    let result = char_array.into_iter().collect();

    result

}