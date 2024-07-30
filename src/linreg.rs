use tfhe::prelude::*;

pub struct HELinearRegression {
    a: tfhe::FheInt16,
    b: tfhe::FheInt16,
}

impl HELinearRegression {
    pub fn new() -> HELinearRegression {
        HELinearRegression {
            a: tfhe::FheInt16::generate_oblivious_pseudo_random(
                tfhe::Seed(0),
                tfhe::SignedRandomizationSpec::FullSigned,
            ),
            b: tfhe::FheInt16::generate_oblivious_pseudo_random(
                tfhe::Seed(0),
                tfhe::SignedRandomizationSpec::FullSigned,
            ),
        }
    }

    pub fn fit(&mut self, x: &Vec<tfhe::FheInt16>, y: &Vec<tfhe::FheInt16>) {
        let sx: tfhe::FheInt16 = x.iter().sum();
        let sy: tfhe::FheInt16 = y.iter().sum();
        let sxx = inner_product(x, x);
        let sxy = inner_product(x, y);
        let n: i16 = x.len().try_into().unwrap();
        // a = (n * sxy - sx * sy) / (n * sxx - sx * sx)
        self.a = (n * &sxy - &sx * &sy) / (n * &sxx - &sx * &sx);
        // b = (sy - a * sx) / n
        self.b = (&sy - &self.a * &sx) / n;
    }

    pub fn predict(&self, x: &tfhe::FheInt16) -> tfhe::FheInt16 {
        return &self.a * x + &self.b;
    }

    pub fn decrypt_params(&self, client_key: &tfhe::ClientKey) -> (i16, i16) {
        let dec_a: i16 = self.a.decrypt(client_key);
        let dec_b: i16 = self.b.decrypt(client_key);
        return (dec_a, dec_b);
    }
}

fn inner_product(a: &Vec<tfhe::FheInt16>, b: &Vec<tfhe::FheInt16>) -> tfhe::FheInt16 {
    let prod: Vec<tfhe::FheInt16> = a.iter().zip(b.iter()).map(|(x, y)| x * y).collect();
    return prod.iter().sum();
}
