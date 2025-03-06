use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use rot13::rot13;
use std::io::{self, Write};

fn print_banner() {
    let banner = r#"
███╗   ██╗ ██████╗  ██████╗ ██████╗   A Rust-based Decryptor
████╗  ██║██╔═══██╗██╔═══██╗██╔══██╗  Version: v1.0.0
██╔██╗ ██║██║   ██║██║   ██║██████╔╝  Author: 4nuxd [Noob]
██║╚██╗██║██║   ██║██║   ██║██╔══██╗  Note: Every Action Has a Consequence
██║ ╚████║╚██████╔╝╚██████╔╝██████╔╝  GitHub: https://github.com/4nuxd/
╚═╝  ╚═══╝ ╚═════╝  ╚═════╝ ╚═════╝   Bored..? : http://bit.ly/3MTMHyU
___________________________________________________________________________
"#;
    println!("{}", banner);
}

fn main() {
    print_banner();

    print!("Enter the encrypted text: ");
    io::stdout().flush().unwrap();

    let mut encrypted_text = String::new();
    io::stdin().read_line(&mut encrypted_text).unwrap();
    let encrypted_text = encrypted_text.trim(); 

    let rot13_decoded = rot13(encrypted_text);

    let base64_decoded = match BASE64_STANDARD.decode(&rot13_decoded) {
        Ok(decoded) => decoded,
        Err(e) => {
            println!("Base64 decoding failed: {}", e);
            return;
        }
    };

    let decoded_text = match String::from_utf8(base64_decoded) {
        Ok(text) => text,
        Err(e) => {
            println!("UTF-8 conversion failed: {}", e);
            return;
        }
    };

    let final_text = rot13(&decoded_text);

    println!("Decrypted Text: {}", final_text);
}
