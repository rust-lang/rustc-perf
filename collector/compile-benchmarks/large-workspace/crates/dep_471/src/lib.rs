pub fn code() {
    println!("Hello from dep_471");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_471");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_471: {t}");
}

pub fn foo() {
    dep_217::code();
    dep_217::code_inlined();
    dep_217::code_generic(1u32);
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
    dep_394::code();
    dep_394::code_inlined();
    dep_394::code_generic(1u32);
    dep_309::code();
    dep_309::code_inlined();
    dep_309::code_generic(1u32);
    dep_349::code();
    dep_349::code_inlined();
    dep_349::code_generic(1u32);
}
