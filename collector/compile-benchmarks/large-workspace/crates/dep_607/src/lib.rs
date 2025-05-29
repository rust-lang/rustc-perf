pub fn code() {
    println!("Hello from dep_607");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_607");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_607: {t}");
}

pub fn foo() {
    dep_344::code();
    dep_344::code_inlined();
    dep_344::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
    dep_250::code();
    dep_250::code_inlined();
    dep_250::code_generic(1u32);
}
