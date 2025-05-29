pub fn code() {
    println!("Hello from dep_41");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_41");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_41: {t}");
}

pub fn foo() {
    dep_1::code();
    dep_1::code_inlined();
    dep_1::code_generic(1u32);
}
