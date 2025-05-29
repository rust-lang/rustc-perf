pub fn code() {
    println!("Hello from dep_318");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_318");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_318: {t}");
}

pub fn foo() {
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_154::code();
    dep_154::code_inlined();
    dep_154::code_generic(1u32);
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
}
