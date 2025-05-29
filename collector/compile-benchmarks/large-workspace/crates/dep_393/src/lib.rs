pub fn code() {
    println!("Hello from dep_393");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_393");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_393: {t}");
}

pub fn foo() {
    dep_83::code();
    dep_83::code_inlined();
    dep_83::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_153::code();
    dep_153::code_inlined();
    dep_153::code_generic(1u32);
}
