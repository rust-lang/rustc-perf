pub fn code() {
    println!("Hello from dep_521");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_521");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_521: {t}");
}

pub fn foo() {
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_290::code();
    dep_290::code_inlined();
    dep_290::code_generic(1u32);
    dep_349::code();
    dep_349::code_inlined();
    dep_349::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
}
