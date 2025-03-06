# Decryptor

A simple Rust-based tool to decrypt text encoded with ROT13 and Base64.

## 🚀 Features
- **Automatic Decryption:** Detects and reverses ROT13 + Base64 encoding.
- **User Input:** Asks for an encrypted key and decrypts it.
- **Error Handling:** Gracefully handles incorrect inputs.
- **Fast & Lightweight:** Runs efficiently with minimal dependencies.

## 🔧 Installation
Make sure you have Rust installed. If not, install it from [Rust Lang](https://www.rust-lang.org/).

```sh
# Clone the repository
git clone https://github.com/4nuxd/rot13-decryptor
cd rot13-decryptor

# Build the project
cargo build --release

# Run the project
cargo run
```

## 📜 Usage
Run the program and enter your encrypted text:
```sh
cargo run
```

It will prompt you to enter the encrypted key and display the decrypted result.

## 📖 Example
### **Execution:**
```sh
cargo run
```
### **Program Output:**
```
███╗   ██╗ ██████╗  ██████╗ ██████╗   A Rust-based Decryptor
████╗  ██║██╔═══██╗██╔═══██╗██╔══██╗  Version: v1.0.0
██╔██╗ ██║██║   ██║██║   ██║██████╔╝  Author: 4nuxd [Noob]
██║╚██╗██║██║   ██║██║   ██║██╔══██╗  Note: Every Action Has a Consequence
██║ ╚████║╚██████╔╝╚██████╔╝██████╔╝  GitHub: https://github.com/4nuxd/
╚═╝  ╚═══╝ ╚═════╝  ╚═════╝ ╚═════╝   Bored..? : http://bit.ly/3MTMHyU
___________________________________________________________________________

Enter the encrypted text: {Encrypted_Key}
Decrypted Text: {Decrypted_Key}
```

## 🛠 Dependencies
This project uses:
- [Base64](https://crates.io/crates/base64)
- [Rot13](https://crates.io/crates/rot13)

## 🤝 Contributing
Pull requests are welcome! Feel free to submit issues or suggest improvements.

## 📜 License
This project is licensed under the [MIT License](LICENSE).

---

🚀 Happy decoding!

