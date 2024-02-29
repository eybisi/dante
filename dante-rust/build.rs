use std::path::Path;
use std::env;
use std::fs;
use std::io::Write;

const APK_SIZE: usize = 2214453;

const fn xor_comp(bytes: &[u8]) -> [u8; APK_SIZE] {
    let key = [0xab,0xc0,0xab,0xc0,0xab,0xc0,0xab,0xc0];

    let mut out = [0; APK_SIZE];
    let mut b = 0;
    while b < bytes.len() {
        out[b] = bytes[b] ^ key[b % key.len()];
        b += 1;
    }
    out
}

fn main() -> std::io::Result<()> {
    println!("cargo:rustc-link-arg=-fno-rtti");
    println!("cargo:rustc-link-arg=-fno-exceptions");
    println!("cargo:rustc-link-lib=static=dobby");
    
    let file_contents = fs::read("src/payload/app-debug.apk").expect("Failed to read file");
    // encrypt
    let encrypted_contents = xor_comp(&file_contents);

    // Encrypt the contents
    // let encrypted_contents: Vec<u8> = file_contents.iter().map(|&byte| xor_encrypt_byte(byte)).collect();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("encrypted_file.rs");
    // Write the encrypted contents to a Rust source file
    let mut output_file = fs::File::create(dest_path).expect("Failed to create output file");
    writeln!(output_file, "pub const ENCRYPTED_FILE_CONTENTS: &[u8] = &[{}];", encrypted_contents.iter().map(|&byte| byte.to_string()).collect::<Vec<_>>().join(", ")).expect("Failed to write output file");

    Ok(())
}
