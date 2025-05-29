pub fn code() {
    println!("Hello from dep_548");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_548");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_548: {t}");
}

pub fn foo() {
    dep_286::code();
    dep_286::code_inlined();
    dep_286::code_generic(1u32);
}
