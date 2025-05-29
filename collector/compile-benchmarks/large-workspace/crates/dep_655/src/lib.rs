pub fn code() {
    println!("Hello from dep_655");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_655");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_655: {t}");
}

pub fn foo() {
    dep_206::code();
    dep_206::code_inlined();
    dep_206::code_generic(1u32);
    dep_262::code();
    dep_262::code_inlined();
    dep_262::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
    dep_170::code();
    dep_170::code_inlined();
    dep_170::code_generic(1u32);
}
