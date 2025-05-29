pub fn code() {
    println!("Hello from dep_117");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_117");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_117: {t}");
}

pub fn foo() {
    dep_59::code();
    dep_59::code_inlined();
    dep_59::code_generic(1u32);
}
