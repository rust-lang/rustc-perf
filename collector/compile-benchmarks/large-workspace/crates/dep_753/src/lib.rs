pub fn code() {
    println!("Hello from dep_753");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_753");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_753: {t}");
}

pub fn foo() {
    dep_287::code();
    dep_287::code_inlined();
    dep_287::code_generic(1u32);
    dep_309::code();
    dep_309::code_inlined();
    dep_309::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_201::code();
    dep_201::code_inlined();
    dep_201::code_generic(1u32);
    dep_313::code();
    dep_313::code_inlined();
    dep_313::code_generic(1u32);
}
