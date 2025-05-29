pub fn code() {
    println!("Hello from dep_280");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_280");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_280: {t}");
}

pub fn foo() {
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
}
