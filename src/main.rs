use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheInt16};

mod linreg;

fn main() {
    let config = ConfigBuilder::default().build();

    // Client-side
    println!("Generating keys.");
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    // Generate data
    let a = 2;
    let b = 5;
    let x: Vec<i16> = (-5..5).collect();
    let y: Vec<i16> = x.iter().map(|i| a * i + b).collect();

    println!("Sampling from: y = {}x + {}", a, b);
    println!("X: {:?}", x);
    println!("Y: {:?}", y);

    // Encrypt vectors
    println!("Encrypting vectors.");
    let enc_x: Vec<FheInt16> = x
        .into_iter()
        .map(|i| FheInt16::encrypt(i, &client_key))
        .collect();
    let enc_y: Vec<FheInt16> = y
        .into_iter()
        .map(|i| FheInt16::encrypt(i, &client_key))
        .collect();

    // Perform linear regression
    let mut helr = linreg::HELinearRegression::new();
    helr.fit(&enc_x, &enc_y);
    let (fa, fb) = helr.decrypt_params(&client_key);

    // Decrypt vectors
    let dec_x: Vec<i16> = enc_x.into_iter().map(|i| i.decrypt(&client_key)).collect();
    let dec_y: Vec<i16> = enc_y.into_iter().map(|i| i.decrypt(&client_key)).collect();

    println!("Decrypted Vectors");
    println!("X: {:?}", dec_x);
    println!("Y: {:?}", dec_y);
    println!("Regression result: {}x + {}", fa, fb);
}
