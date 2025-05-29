pub fn code() {
    println!("Hello from dep_362");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_362");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_362: {t}");
}

pub fn foo() {
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_150::code();
    dep_150::code_inlined();
    dep_150::code_generic(1u32);
}
