pub fn code() {
    println!("Hello from dep_453");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_453");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_453: {t}");
}

pub fn foo() {
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
    dep_387::code();
    dep_387::code_inlined();
    dep_387::code_generic(1u32);
}
