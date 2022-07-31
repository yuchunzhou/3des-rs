// strip the suffix from a slice
pub fn strip(s: &[u8], suffix: u8) -> &[u8] {
    let mut s_ = s;
    loop {
        match s_.strip_suffix(&[suffix]) {
            None => {
                return &s_;
            }
            Some(s) => {
                s_ = s;
            }
        }
    }
}

extern "C" {
    pub fn des3_cbc_encrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
        piv: *const u8,
    ) -> u32;
    pub fn des3_cbc_decrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
        piv: *const u8,
    ) -> u32;

    pub fn des3_ecb_encrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
    ) -> u32;
    pub fn des3_ecb_decrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
    ) -> u32;
}
