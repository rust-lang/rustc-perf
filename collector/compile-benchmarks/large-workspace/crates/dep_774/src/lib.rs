pub fn code() {
    println!("Hello from dep_774");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_774");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_774: {t}");
}

pub fn foo() {
    dep_223::code();
    dep_223::code_inlined();
    dep_223::code_generic(1u32);
}
