use libc;

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

#[derive(Debug)]
pub struct Result<'a>(pub &'a [u8]);

impl Drop for Result<'_> {
    fn drop(&mut self) {
        let ptr = self.0.as_ptr();
        unsafe {
            libc::free(ptr as *mut libc::c_void);
        }
    }
}

pub fn cbc_encrypt<'b, 'a: 'b>(data: &'a [u8], key: &'a [u8]) -> Result<'b> {
    unsafe {
        let mut n = data.len();
        if data.len() % 8 != 0 {
            n += 8 - data.len() % 8;
        }

        let out = libc::calloc(n, 1) as *mut u8;
        des3_cbc_encrypt(
            out,
            data.as_ptr(),
            data.len() as u32,
            key.as_ptr(),
            key.len() as u32,
            0 as *const u8,
        );
        Result(std::slice::from_raw_parts(out, n))
    }
}

pub fn cbc_decrypt<'b, 'a: 'b>(data: &'a [u8], key: &'a [u8]) -> Option<Result<'b>> {
    unsafe {
        let out = libc::calloc(data.len(), 1) as *mut u8;
        let n = des3_cbc_decrypt(
            out,
            data.as_ptr(),
            data.len() as u32,
            key.as_ptr(),
            key.len() as u32,
            0 as *const u8,
        );
        if n == 0 {
            Some(Result(std::slice::from_raw_parts(
                out,
                libc::strlen(out as *const i8),
            )))
        } else {
            None
        }
    }
}

pub fn ecb_encrypt<'b, 'a: 'b>(data: &'a [u8], key: &'a [u8]) -> Result<'b> {
    unsafe {
        let mut n = data.len();
        if data.len() % 8 != 0 {
            n += 8 - data.len() % 8;
        }

        let out = libc::calloc(n, 1) as *mut u8;
        des3_ecb_encrypt(
            out,
            data.as_ptr(),
            data.len() as u32,
            key.as_ptr(),
            key.len() as u32,
        );
        Result(std::slice::from_raw_parts(out, n))
    }
}

pub fn ecb_decrypt<'b, 'a: 'b>(data: &'a [u8], key: &'a [u8]) -> Option<Result<'b>> {
    unsafe {
        let out = libc::calloc(data.len(), 1) as *mut u8;
        let n = des3_ecb_decrypt(
            out,
            data.as_ptr(),
            data.len() as u32,
            key.as_ptr(),
            key.len() as u32,
        );
        if n == 0 {
            Some(Result(std::slice::from_raw_parts(
                out,
                libc::strlen(out as *const i8),
            )))
        } else {
            None
        }
    }
}
