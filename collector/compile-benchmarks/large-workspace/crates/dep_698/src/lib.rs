pub fn code() {
    println!("Hello from dep_698");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_698");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_698: {t}");
}

pub fn foo() {
    dep_276::code();
    dep_276::code_inlined();
    dep_276::code_generic(1u32);
}
