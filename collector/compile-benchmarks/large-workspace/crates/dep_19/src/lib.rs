pub fn code() {
    println!("Hello from dep_19");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_19");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_19: {t}");
}

pub fn foo() {
    dep_0::code();
    dep_0::code_inlined();
    dep_0::code_generic(1u32);
    dep_6::code();
    dep_6::code_inlined();
    dep_6::code_generic(1u32);
}
