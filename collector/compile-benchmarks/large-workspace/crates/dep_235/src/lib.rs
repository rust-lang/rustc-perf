pub fn code() {
    println!("Hello from dep_235");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_235");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_235: {t}");
}

pub fn foo() {
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
    dep_153::code();
    dep_153::code_inlined();
    dep_153::code_generic(1u32);
    dep_114::code();
    dep_114::code_inlined();
    dep_114::code_generic(1u32);
    dep_108::code();
    dep_108::code_inlined();
    dep_108::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
}
