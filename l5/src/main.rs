use rand::prelude::*;
use rand_pcg::Pcg64;
use itertools::Itertools;

fn main() {
    let bank_numbers = gen_bank_numbers(10);
    let key = b"Very Good Key";
    let mut cryptograms = Vec::new();
    for bank_number in &bank_numbers {
        let cryptogram = rc4(key, bank_number.as_bytes());
        cryptograms.push(cryptogram);
    }
    for (c0, c1) in cryptograms.iter().tuple_combinations() {
        let xored: Vec<u8> = c0.iter().zip(c1.iter()).map(|(i0, i1)| i0 ^ i1).collect();
        println!("{:?}", xored[2..10].to_vec()); 

    }
}

fn gen_bank_numbers(q: usize) -> Vec<String> {
    let mut bank_numbers: Vec<String> = Vec::new();
    let numery_rozliczeniowe: [[u8; 8]; 5] = [
        [1, 0, 1, 0, 0, 0, 0, 0], // NBP
        [1, 1, 6, 0, 0, 0, 0, 6], // Millenium 
        [1, 0, 5, 0, 0, 0, 0, 2], // ING
        [2, 1, 2, 0, 0, 0, 0, 1], // Santander
        [1, 0, 2, 0, 0, 0, 0, 3], // PKO BP
    ]; 
    let mut rng = Pcg64::seed_from_u64(2137);
    for nr  in numery_rozliczeniowe {
        for _ in 0..q {
            let mut bank_number = String::new();
            let mut client_number = [0u8; 16];
            for i in 0..16 {
                client_number[i] = rng.gen_range(0..10);
            }
            let mut tmp: u128 = 212500;
            for i in 0..8 {
                tmp += nr[i] as u128 * 10u128.pow((7 - i + 21) as u32);
            }
            for i in 0..16 {
                tmp += client_number[i] as u128 * 10u128.pow((15 - i + 5) as u32);
            }
            tmp = tmp % 97;
            tmp = 98 - tmp;
            bank_number.push_str(format!("{:02}", tmp).as_str());
            for i in 0..8 {
                bank_number.push_str(nr[i].to_string().as_str());
            }
            for i in 0..16 {
                bank_number.push_str(client_number[i].to_string().as_str());
            }
            bank_numbers.push(bank_number);
        }
    }
    bank_numbers
}

pub fn calculte_nr_control_number(nr: [u8; 7]) -> u8 {
    let weights = [3, 9, 7, 1, 3, 9, 7];
    let mut sum = 0;
    for i in 0..7 {
        sum += nr[i] as u16 * weights[i] as u16;
    }
    (10 - (sum % 10) as u8) % 10
}

pub fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    // KSA
    let mut s: [u8; 256] = [0u8; 256];
    for i in 0..256 {
        s[i] = i as u8;
    }
    let mut j = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }

    // PRGA
    let mut i = 0;
    let mut j = 0;
    let mut ciphertext = Vec::new();
    for byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[((s[i] as usize + s[j] as usize) % 256) as usize];
        ciphertext.push(byte ^ k);
    }
    ciphertext
}

pub fn uses_same_key(ciphertext0: &[u8], ciphertext1: &[u8] ) -> bool {
    for i in 0..ciphertext0.len().min(ciphertext1.len()) {
        if (ciphertext0[i] ^ ciphertext1[i]) >= 0x80 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decipher() {
        let key0 = b"Very Good Key";
        let key1 = b"WOW This Is a Key";
        for _ in 0..1024 {
            let mut data = [0u8; 1024];
            thread_rng().fill_bytes(&mut data);
            let ciphertext0 = rc4(key0, &data);
            let ciphertext1 = rc4(key1, &data);
            let plaintext0 = rc4(key0, &ciphertext0);
            let plaintext1 = rc4(key1, &ciphertext1);
            assert_eq!(data, plaintext0[..]);
            assert_eq!(data, plaintext1[..]);
        }
    }
    #[test]
    fn test_same_key() {
        let key = b"Very Good Key";
        let key1 = b"Very Bad Key";
        let data0 = b"BLAHBLAH";
        let data1 = b"HellO, World!";
        let data2 = b"afuyawevijcyorld!";
        let ciphertext0 = rc4(key, data0);
        let ciphertext1 = rc4(key, data1);
        let ciphertext2 = rc4(key1, data2);
        assert!(uses_same_key(&ciphertext0, &ciphertext1));
        assert!(!uses_same_key(&ciphertext0, &ciphertext2));
        assert!(!uses_same_key(&ciphertext1, &ciphertext2));
    }
}
