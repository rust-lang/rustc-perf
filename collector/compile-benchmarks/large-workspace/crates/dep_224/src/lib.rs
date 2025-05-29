pub fn code() {
    println!("Hello from dep_224");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_224");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_224: {t}");
}

pub fn foo() {
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
    dep_77::code();
    dep_77::code_inlined();
    dep_77::code_generic(1u32);
    dep_131::code();
    dep_131::code_inlined();
    dep_131::code_generic(1u32);
}
