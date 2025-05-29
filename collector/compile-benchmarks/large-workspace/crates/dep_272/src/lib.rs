pub fn code() {
    println!("Hello from dep_272");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_272");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_272: {t}");
}

pub fn foo() {
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
    dep_125::code();
    dep_125::code_inlined();
    dep_125::code_generic(1u32);
}
