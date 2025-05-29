pub fn code() {
    println!("Hello from dep_400");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_400");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_400: {t}");
}

pub fn foo() {
    dep_61::code();
    dep_61::code_inlined();
    dep_61::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
    dep_120::code();
    dep_120::code_inlined();
    dep_120::code_generic(1u32);
    dep_129::code();
    dep_129::code_inlined();
    dep_129::code_generic(1u32);
}
