pub fn code() {
    println!("Hello from dep_396");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_396");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_396: {t}");
}

pub fn foo() {
    dep_116::code();
    dep_116::code_inlined();
    dep_116::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
}
