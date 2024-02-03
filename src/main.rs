// Milo Tomlinson Encryption Decryption Project

mod encrypt;
mod decrypt;



use encrypt::encrypt;
use decrypt::decrypt;

slint::include_modules!();

pub fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    ui.on_encryption({
        let ui_handle = ui.as_weak();
        move |msg| {
            let ui = ui_handle.unwrap();
            let encrypt_message = encrypt(msg.trim_end().to_string());
            cli_clipboard::set_contents(encrypt_message.clone()).unwrap();
            ui.set_output(encrypt_message.into());
        }
    });
    ui.on_decryption({
        let ui_handle = ui.as_weak();
        move |msg| {
            let ui = ui_handle.unwrap();
            let decrypt_message = decrypt(msg.trim_end().to_string());
            cli_clipboard::set_contents(decrypt_message.clone()).unwrap();
            ui.set_output(decrypt_message.into());
        }
    });


    ui.run()
}