pub fn code() {
    println!("Hello from dep_479");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_479");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_479: {t}");
}

pub fn foo() {
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
}
