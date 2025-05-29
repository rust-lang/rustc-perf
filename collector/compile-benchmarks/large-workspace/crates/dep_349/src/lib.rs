pub fn code() {
    println!("Hello from dep_349");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_349");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_349: {t}");
}

pub fn foo() {
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
}
