pub fn code() {
    println!("Hello from dep_694");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_694");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_694: {t}");
}

pub fn foo() {
    dep_200::code();
    dep_200::code_inlined();
    dep_200::code_generic(1u32);
}
