use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rstats::Vecg;
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
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let x: Vec<i16> = (-5..=5).map(|i| 10 * i).collect();
    let y: Vec<i16> = x
        .iter()
        .map(|i| a * i + b + rng.gen_range(-3..=3))
        .collect();
    let testx: Vec<i16> = (0..10).map(|_| rng.gen_range(-50..=50)).collect();
    let testy: Vec<i16> = testx.iter().map(|i| a * i + b).collect();

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
    let enc_testx: Vec<FheInt16> = testx
        .clone()
        .into_iter()
        .map(|i| FheInt16::encrypt(i, &client_key))
        .collect();

    // Perform linear regression
    println!("Performing homomorphic linear regression.");
    let mut helr = linreg::HELinearRegression::new();
    helr.fit(&enc_x, &enc_y);
    let (fa, fb) = helr.decrypt_params(&client_key);

    let enc_pred: Vec<FheInt16> = enc_testx.iter().map(|x| helr.predict(x)).collect();

    // Decrypt vectors
    let dec_x: Vec<i16> = enc_x.into_iter().map(|i| i.decrypt(&client_key)).collect();
    let dec_y: Vec<i16> = enc_y.into_iter().map(|i| i.decrypt(&client_key)).collect();
    let dec_pred: Vec<i16> = enc_pred
        .into_iter()
        .map(|i| i.decrypt(&client_key))
        .collect();
    let mse = dec_pred.vdistsq(&testy) / (dec_pred.len() as f64);

    println!("Decrypted Vectors");
    println!("X: {:?}", dec_x);
    println!("Y: {:?}", dec_y);
    println!("Test X: {:?}", &testx);
    println!("Test Y: {:?}", testy);
    println!("Prediction: {:?}", dec_pred);
    println!("MSE: {:.3}", mse);
    println!("Regression result: {}x + {}", fa, fb);
}
