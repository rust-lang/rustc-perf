pub fn code() {
    println!("Hello from dep_570");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_570");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_570: {t}");
}

pub fn foo() {
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
    dep_320::code();
    dep_320::code_inlined();
    dep_320::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
    dep_167::code();
    dep_167::code_inlined();
    dep_167::code_generic(1u32);
}
