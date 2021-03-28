#[no_mangle]
extern "C" fn drone_log_is_enabled(_port: u8) -> bool {
    false
}

#[no_mangle]
extern "C" fn drone_log_write_bytes(_port: u8, _buffer: *const u8, _count: usize) {}

#[no_mangle]
extern "C" fn drone_log_write_u8(_port: u8, _value: u8) {}

#[no_mangle]
extern "C" fn drone_log_write_u16(_port: u8, _value: u16) {}

#[no_mangle]
extern "C" fn drone_log_write_u32(_port: u8, _value: u32) {}

#[no_mangle]
extern "C" fn drone_log_flush() {}

#[no_mangle]
extern "C" fn drone_self_reset() -> ! {
    // TODO implement self resetting sequence.
    #[allow(clippy::empty_loop)]
    loop {}
}
