// Milo Tomlinson Encryption Decryption Project

mod encrypt;
mod decrypt;


use encrypt::encrypt;
use decrypt::decrypt;
use std::io;


fn main() {
    //The user types in the message that they would like to encrypt
    let mut message = String::new();
    println!("Please type your message to be encrypted: ");
    io::stdin().read_line(&mut message).expect("Failed to read line");

    //The encrypt method is called with a clone of the message so it can encrypt
    let encrypt_mess = encrypt(message.clone());
    let decrypt_mess = decrypt(encrypt_mess.clone());
    println!("Original: {}\nEncrypt: {}\nDecrypt: {}", message, encrypt_mess, decrypt_mess);
    cli_clipboard::set_contents(encrypt_mess).unwrap();
    // This is here so the file doesn't close after completing until user is ready
    println!("press enter to quit");
    io::stdin().read_line(&mut message).expect("Failed to read line");
    println!("test {}", 'c' as i32);
}



