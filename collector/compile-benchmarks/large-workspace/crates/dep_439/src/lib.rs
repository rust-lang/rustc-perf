pub fn code() {
    println!("Hello from dep_439");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_439");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_439: {t}");
}

pub fn foo() {
    dep_350::code();
    dep_350::code_inlined();
    dep_350::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
}
