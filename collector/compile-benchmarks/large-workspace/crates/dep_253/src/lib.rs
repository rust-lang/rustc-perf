pub fn code() {
    println!("Hello from dep_253");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_253");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_253: {t}");
}

pub fn foo() {
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
    dep_138::code();
    dep_138::code_inlined();
    dep_138::code_generic(1u32);
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
}
