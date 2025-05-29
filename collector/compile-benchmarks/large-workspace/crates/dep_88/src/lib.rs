pub fn code() {
    println!("Hello from dep_88");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_88");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_88: {t}");
}

pub fn foo() {
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
}
