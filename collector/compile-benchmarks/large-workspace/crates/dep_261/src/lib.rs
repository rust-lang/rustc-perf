pub fn code() {
    println!("Hello from dep_261");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_261");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_261: {t}");
}

pub fn foo() {
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_120::code();
    dep_120::code_inlined();
    dep_120::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
}
