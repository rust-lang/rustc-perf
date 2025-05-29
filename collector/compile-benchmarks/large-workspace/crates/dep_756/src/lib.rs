pub fn code() {
    println!("Hello from dep_756");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_756");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_756: {t}");
}

pub fn foo() {
    dep_365::code();
    dep_365::code_inlined();
    dep_365::code_generic(1u32);
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
}
