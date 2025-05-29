pub fn code() {
    println!("Hello from dep_511");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_511");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_511: {t}");
}

pub fn foo() {
    dep_204::code();
    dep_204::code_inlined();
    dep_204::code_generic(1u32);
    dep_187::code();
    dep_187::code_inlined();
    dep_187::code_generic(1u32);
    dep_313::code();
    dep_313::code_inlined();
    dep_313::code_generic(1u32);
    dep_302::code();
    dep_302::code_inlined();
    dep_302::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
}
