pub fn code() {
    println!("Hello from dep_556");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_556");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_556: {t}");
}

pub fn foo() {
    dep_325::code();
    dep_325::code_inlined();
    dep_325::code_generic(1u32);
}
