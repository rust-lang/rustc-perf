pub fn code() {
    println!("Hello from dep_840");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_840");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_840: {t}");
}

pub fn foo() {
    dep_346::code();
    dep_346::code_inlined();
    dep_346::code_generic(1u32);
}
