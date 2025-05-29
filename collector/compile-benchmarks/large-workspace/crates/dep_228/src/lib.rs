pub fn code() {
    println!("Hello from dep_228");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_228");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_228: {t}");
}

pub fn foo() {
    dep_148::code();
    dep_148::code_inlined();
    dep_148::code_generic(1u32);
}
