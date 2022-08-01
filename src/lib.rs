extern "C" {
    fn des3_cbc_encrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
        piv: *const u8,
    ) -> u32;
    fn des3_cbc_decrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
        piv: *const u8,
    ) -> u32;

    fn des3_ecb_encrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
    ) -> u32;
    fn des3_ecb_decrypt(
        pout: *mut u8,
        pdata: *const u8,
        nlen: u32,
        pkey: *const u8,
        klen: u32,
    ) -> u32;
}

pub fn cbc_encrypt(
    pout: *mut u8,
    pdata: *const u8,
    nlen: u32,
    pkey: *const u8,
    klen: u32,
    piv: *const u8,
) -> u32 {
    unsafe { des3_cbc_encrypt(pout, pdata, nlen, pkey, klen, piv) }
}

pub fn cbc_decrypt(
    pout: *mut u8,
    pdata: *const u8,
    nlen: u32,
    pkey: *const u8,
    klen: u32,
    piv: *const u8,
) -> u32 {
    unsafe { des3_cbc_decrypt(pout, pdata, nlen, pkey, klen, piv) }
}

pub fn ecb_encrypt(pout: *mut u8, pdata: *const u8, nlen: u32, pkey: *const u8, klen: u32) -> u32 {
    unsafe { des3_ecb_encrypt(pout, pdata, nlen, pkey, klen) }
}

pub fn ecb_decrypt(pout: *mut u8, pdata: *const u8, nlen: u32, pkey: *const u8, klen: u32) -> u32 {
    unsafe { des3_ecb_decrypt(pout, pdata, nlen, pkey, klen) }
}
