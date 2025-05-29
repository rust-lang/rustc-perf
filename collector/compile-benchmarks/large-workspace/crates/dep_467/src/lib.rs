pub fn code() {
    println!("Hello from dep_467");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_467");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_467: {t}");
}

pub fn foo() {
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
    dep_287::code();
    dep_287::code_inlined();
    dep_287::code_generic(1u32);
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
}
