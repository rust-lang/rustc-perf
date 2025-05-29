pub fn code() {
    println!("Hello from dep_796");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_796");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_796: {t}");
}

pub fn foo() {
    dep_340::code();
    dep_340::code_inlined();
    dep_340::code_generic(1u32);
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
}
