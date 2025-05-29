pub fn code() {
    println!("Hello from dep_794");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_794");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_794: {t}");
}

pub fn foo() {
    dep_384::code();
    dep_384::code_inlined();
    dep_384::code_generic(1u32);
    dep_307::code();
    dep_307::code_inlined();
    dep_307::code_generic(1u32);
    dep_244::code();
    dep_244::code_inlined();
    dep_244::code_generic(1u32);
}
