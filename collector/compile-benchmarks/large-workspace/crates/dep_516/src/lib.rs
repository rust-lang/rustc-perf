pub fn code() {
    println!("Hello from dep_516");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_516");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_516: {t}");
}

pub fn foo() {
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_209::code();
    dep_209::code_inlined();
    dep_209::code_generic(1u32);
}
