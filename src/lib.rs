extern "C" {
    pub fn des3_cbc_encrypt(
        pout: *const u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
        piv: *const u8,
    ) -> u32;
    pub fn des3_cbc_decrypt(
        pout: *const u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
        piv: *const u8,
    ) -> u32;

    pub fn des3_ecb_encrypt(
        pout: *const u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
    ) -> u32;
    pub fn des3_ecb_decrypt(
        pout: *const u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
    ) -> u32;
}
