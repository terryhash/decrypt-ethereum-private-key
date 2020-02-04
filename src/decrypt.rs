use crate::data_process::Data;

pub fn decrypt(data: &Data, key: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    use aes_ctr::Aes128Ctr;
    use aes_ctr::stream_cipher::generic_array::GenericArray;
    use aes_ctr::stream_cipher::{
        NewStreamCipher, SyncStreamCipher
    };

    // Note that the aes_key must be 16 bytes, but the Ethereum keyfile key derivation algorithm
    // uses scrypt to derive a 32 byte key from a user-supplied password.
    // Use the first 16 bytes of this derived key.
    let aes_key = GenericArray::from_slice(key.split_at(16).0);
    let initialization_vector = GenericArray::from_slice(&data.iv);
    let mut ct = (data.ct).clone();
    let mut cipher = Aes128Ctr::new(&aes_key, &initialization_vector);
    cipher.apply_keystream(&mut ct);
    Ok(ct.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_key_decryption() {
        // Values from sample keyfile
        let ct = vec![
            0x05, 0x0d, 0x93, 0xd6, 0xa4, 0xe3, 0x96, 0xa0,
            0xcb, 0x74, 0xd0, 0x21, 0xd0, 0xde, 0x9b, 0x1e,
            0xd7, 0x86, 0x0c, 0x0f, 0xd8, 0x43, 0xb2, 0x8a,
            0xce, 0xfb, 0xd3, 0xdc, 0x61, 0x31, 0x4a, 0x19,
        ];
        let salt = vec![
            0xb0, 0x4d, 0xcc, 0xcf, 0x35, 0x1d, 0xba, 0x67,
            0x46, 0x0e, 0x5b, 0xf3, 0x22, 0x49, 0x3a, 0xb2,
            0x5b, 0x4e, 0x1b, 0x31, 0x4d, 0xf9, 0x70, 0x50,
            0x3e, 0xd4, 0x3c, 0x39, 0x21, 0x66, 0xd4, 0xc8,
        ];
        let iv = vec![
            0x6a, 0xa1, 0xde, 0x28, 0xf8, 0xf4, 0x3a, 0x52,
            0x2e, 0x6a, 0xc9, 0x87, 0xc1, 0x8b, 0xf6, 0x6e,
        ];
        let password = b"password123".to_vec();
        let private_key = vec![
            0x82, 0x63, 0x39, 0x60, 0xe2, 0xa7, 0x25, 0xab,
            0x64, 0x10, 0x67, 0xa1, 0x2b, 0x05, 0xfc, 0xae,
            0xca, 0x86, 0x0d, 0x45, 0xba, 0x78, 0x5f, 0x63,
            0x43, 0x18, 0x49, 0x02, 0x61, 0xe5, 0xd1, 0xa1,
        ];
        let key = vec![
            0x5a, 0xe6, 0xf8, 0x78, 0x53, 0x37, 0x64, 0x5b,
            0x7c, 0xed, 0xd5, 0x3f, 0x71, 0x28, 0x63, 0xb7,
            0x0c, 0xc0, 0x61, 0x5f, 0x48, 0xf1, 0x8a, 0x3e,
            0x27, 0xa8, 0xf9, 0x22, 0xed, 0xc1, 0x3a, 0x84,
        ];
        let data = Data {
            ct: ct,
            salt: salt, 
            mac: vec![],
            iv: iv,
            password: password,
            n: 262144, 
            r: 8,
            p: 1,
            dklen: 32,
            maxmem: 2000000000
        };
        assert_eq!(private_key, decrypt(&data, &key).unwrap());
    }
}

