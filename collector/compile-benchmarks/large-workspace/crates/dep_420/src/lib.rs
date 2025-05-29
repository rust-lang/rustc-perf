pub fn code() {
    println!("Hello from dep_420");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_420");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_420: {t}");
}

pub fn foo() {
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
}
