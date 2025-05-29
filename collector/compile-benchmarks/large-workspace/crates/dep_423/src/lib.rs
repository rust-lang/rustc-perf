pub fn code() {
    println!("Hello from dep_423");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_423");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_423: {t}");
}

pub fn foo() {
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
    dep_229::code();
    dep_229::code_inlined();
    dep_229::code_generic(1u32);
    dep_304::code();
    dep_304::code_inlined();
    dep_304::code_generic(1u32);
    dep_290::code();
    dep_290::code_inlined();
    dep_290::code_generic(1u32);
}
