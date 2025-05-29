pub fn code() {
    println!("Hello from dep_87");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_87");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_87: {t}");
}

pub fn foo() {
    dep_13::code();
    dep_13::code_inlined();
    dep_13::code_generic(1u32);
    dep_22::code();
    dep_22::code_inlined();
    dep_22::code_generic(1u32);
    dep_41::code();
    dep_41::code_inlined();
    dep_41::code_generic(1u32);
}
