pub fn code() {
    println!("Hello from dep_750");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_750");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_750: {t}");
}

pub fn foo() {
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_240::code();
    dep_240::code_inlined();
    dep_240::code_generic(1u32);
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
    dep_219::code();
    dep_219::code_inlined();
    dep_219::code_generic(1u32);
}
