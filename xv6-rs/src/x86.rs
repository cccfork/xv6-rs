#[no_mangle]
pub unsafe extern "C" fn cli() {
    asm!("cli");
}
