// Milo Tomlinson Encryption Decryption Project

mod encrypt;
mod decrypt;


use encrypt::encrypt;
use decrypt::decrypt;
use std::io;


fn main() {
    println!("Hi welcome to my Encryption and Decryption program!");
    let mut option:String = String::new();
    while !option.starts_with("3") {
        option = String::new();
        println!("Please enter the option you would like to do:\n1. Encryption of a message\n2. Decryption of a message\n3. Quit");
        io::stdin().read_line(&mut option).expect("Failed to read line");

        if option.starts_with('1') { // Encryption
            let mut message = String::new();
            println!("Please type your message to be encrypted: ");
            io::stdin().read_line(&mut message).expect("Failed to read line");
            let encrypt_mess = encrypt(message.clone().trim_end().to_string());
            println!("Encrypted Message: {}", encrypt_mess);
            cli_clipboard::set_contents(encrypt_mess).unwrap();
        }
        else if option.starts_with('2') {
            let mut message = String::new();
            println!("Please type your message to be decrypted: ");
            io::stdin().read_line(&mut message).expect("Failed to read line");
            let decrypt_mess = decrypt(message.clone().trim_end().to_string());
            println!("Decrypted Message: {}", decrypt_mess);
            cli_clipboard::set_contents(decrypt_mess).unwrap();
        }
        else if option.starts_with('3') {
            println!("Thank you for using my Encryption and Decryption program!\n");
        }
        else{
            println!("Please type in a valid option ex: \"1\" without the quotes");
        }
    }
    io::stdin().read_line(&mut option).expect("Failed to read line");
}



