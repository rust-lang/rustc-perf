pub fn code() {
    println!("Hello from dep_706");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_706");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_706: {t}");
}

pub fn foo() {
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
}
