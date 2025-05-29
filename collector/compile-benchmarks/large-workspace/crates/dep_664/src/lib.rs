pub fn code() {
    println!("Hello from dep_664");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_664");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_664: {t}");
}

pub fn foo() {
    dep_226::code();
    dep_226::code_inlined();
    dep_226::code_generic(1u32);
}
