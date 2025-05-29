pub fn code() {
    println!("Hello from dep_775");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_775");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_775: {t}");
}

pub fn foo() {
    dep_318::code();
    dep_318::code_inlined();
    dep_318::code_generic(1u32);
    dep_387::code();
    dep_387::code_inlined();
    dep_387::code_generic(1u32);
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
}
