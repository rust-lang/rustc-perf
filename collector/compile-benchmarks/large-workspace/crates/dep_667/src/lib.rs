pub fn code() {
    println!("Hello from dep_667");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_667");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_667: {t}");
}

pub fn foo() {
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
}
