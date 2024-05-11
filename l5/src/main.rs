fn main() {
    
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
    use rand::prelude::*;
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
