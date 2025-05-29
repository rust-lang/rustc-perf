pub fn code() {
    println!("Hello from dep_810");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_810");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_810: {t}");
}

pub fn foo() {
    dep_289::code();
    dep_289::code_inlined();
    dep_289::code_generic(1u32);
}
