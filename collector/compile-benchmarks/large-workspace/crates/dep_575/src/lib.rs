pub fn code() {
    println!("Hello from dep_575");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_575");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_575: {t}");
}

pub fn foo() {
    dep_402::code();
    dep_402::code_inlined();
    dep_402::code_generic(1u32);
    dep_235::code();
    dep_235::code_inlined();
    dep_235::code_generic(1u32);
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
    dep_167::code();
    dep_167::code_inlined();
    dep_167::code_generic(1u32);
}
