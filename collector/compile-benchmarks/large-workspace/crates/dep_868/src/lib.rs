pub fn code() {
    println!("Hello from dep_868");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_868");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_868: {t}");
}

pub fn foo() {
    dep_202::code();
    dep_202::code_inlined();
    dep_202::code_generic(1u32);
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
    dep_353::code();
    dep_353::code_inlined();
    dep_353::code_generic(1u32);
}
