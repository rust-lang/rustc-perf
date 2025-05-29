pub fn code() {
    println!("Hello from dep_345");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_345");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_345: {t}");
}

pub fn foo() {
    dep_118::code();
    dep_118::code_inlined();
    dep_118::code_generic(1u32);
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
}
