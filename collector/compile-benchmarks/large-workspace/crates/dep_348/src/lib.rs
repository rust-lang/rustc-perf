pub fn code() {
    println!("Hello from dep_348");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_348");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_348: {t}");
}

pub fn foo() {
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
}
