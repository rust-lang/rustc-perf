pub fn code() {
    println!("Hello from dep_604");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_604");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_604: {t}");
}

pub fn foo() {
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
}
