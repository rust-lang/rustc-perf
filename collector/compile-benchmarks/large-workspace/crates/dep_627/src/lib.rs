pub fn code() {
    println!("Hello from dep_627");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_627");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_627: {t}");
}

pub fn foo() {
    dep_377::code();
    dep_377::code_inlined();
    dep_377::code_generic(1u32);
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
    dep_385::code();
    dep_385::code_inlined();
    dep_385::code_generic(1u32);
    dep_224::code();
    dep_224::code_inlined();
    dep_224::code_generic(1u32);
}
