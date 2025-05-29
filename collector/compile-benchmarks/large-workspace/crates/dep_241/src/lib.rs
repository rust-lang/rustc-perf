pub fn code() {
    println!("Hello from dep_241");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_241");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_241: {t}");
}

pub fn foo() {
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
    dep_60::code();
    dep_60::code_inlined();
    dep_60::code_generic(1u32);
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
}
