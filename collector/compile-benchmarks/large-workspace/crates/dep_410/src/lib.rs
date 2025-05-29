pub fn code() {
    println!("Hello from dep_410");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_410");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_410: {t}");
}

pub fn foo() {
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
    dep_173::code();
    dep_173::code_inlined();
    dep_173::code_generic(1u32);
    dep_210::code();
    dep_210::code_inlined();
    dep_210::code_generic(1u32);
}
