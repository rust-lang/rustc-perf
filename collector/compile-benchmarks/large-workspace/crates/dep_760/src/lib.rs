pub fn code() {
    println!("Hello from dep_760");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_760");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_760: {t}");
}

pub fn foo() {
    dep_218::code();
    dep_218::code_inlined();
    dep_218::code_generic(1u32);
    dep_298::code();
    dep_298::code_inlined();
    dep_298::code_generic(1u32);
}
