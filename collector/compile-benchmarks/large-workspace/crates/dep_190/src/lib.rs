pub fn code() {
    println!("Hello from dep_190");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_190");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_190: {t}");
}

pub fn foo() {
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
}
