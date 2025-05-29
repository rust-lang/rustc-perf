pub fn code() {
    println!("Hello from dep_286");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_286");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_286: {t}");
}

pub fn foo() {
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
    dep_127::code();
    dep_127::code_inlined();
    dep_127::code_generic(1u32);
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
}
