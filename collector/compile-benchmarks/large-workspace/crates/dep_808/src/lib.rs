pub fn code() {
    println!("Hello from dep_808");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_808");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_808: {t}");
}

pub fn foo() {
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_352::code();
    dep_352::code_inlined();
    dep_352::code_generic(1u32);
    dep_244::code();
    dep_244::code_inlined();
    dep_244::code_generic(1u32);
}
