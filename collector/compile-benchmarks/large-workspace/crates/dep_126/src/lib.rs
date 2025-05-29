pub fn code() {
    println!("Hello from dep_126");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_126");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_126: {t}");
}

pub fn foo() {
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
}
