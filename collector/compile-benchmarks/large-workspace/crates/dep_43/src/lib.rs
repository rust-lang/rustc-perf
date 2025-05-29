pub fn code() {
    println!("Hello from dep_43");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_43");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_43: {t}");
}

pub fn foo() {
    dep_3::code();
    dep_3::code_inlined();
    dep_3::code_generic(1u32);
}
