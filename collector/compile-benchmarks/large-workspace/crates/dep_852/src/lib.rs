pub fn code() {
    println!("Hello from dep_852");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_852");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_852: {t}");
}

pub fn foo() {
    dep_253::code();
    dep_253::code_inlined();
    dep_253::code_generic(1u32);
    dep_173::code();
    dep_173::code_inlined();
    dep_173::code_generic(1u32);
    dep_334::code();
    dep_334::code_inlined();
    dep_334::code_generic(1u32);
}
