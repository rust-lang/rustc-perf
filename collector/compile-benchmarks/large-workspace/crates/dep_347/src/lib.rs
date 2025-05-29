pub fn code() {
    println!("Hello from dep_347");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_347");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_347: {t}");
}

pub fn foo() {
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_114::code();
    dep_114::code_inlined();
    dep_114::code_generic(1u32);
}
