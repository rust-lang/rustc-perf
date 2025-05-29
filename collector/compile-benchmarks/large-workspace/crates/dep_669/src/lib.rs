pub fn code() {
    println!("Hello from dep_669");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_669");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_669: {t}");
}

pub fn foo() {
    dep_186::code();
    dep_186::code_inlined();
    dep_186::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
    dep_260::code();
    dep_260::code_inlined();
    dep_260::code_generic(1u32);
}
