pub fn code() {
    println!("Hello from dep_35");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_35");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_35: {t}");
}

pub fn foo() {
    dep_6::code();
    dep_6::code_inlined();
    dep_6::code_generic(1u32);
    dep_2::code();
    dep_2::code_inlined();
    dep_2::code_generic(1u32);
}
