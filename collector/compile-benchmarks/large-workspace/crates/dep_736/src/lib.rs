pub fn code() {
    println!("Hello from dep_736");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_736");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_736: {t}");
}

pub fn foo() {
    dep_213::code();
    dep_213::code_inlined();
    dep_213::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_211::code();
    dep_211::code_inlined();
    dep_211::code_generic(1u32);
}
