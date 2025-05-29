pub fn code() {
    println!("Hello from dep_316");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_316");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_316: {t}");
}

pub fn foo() {
    dep_66::code();
    dep_66::code_inlined();
    dep_66::code_generic(1u32);
}
