pub fn code() {
    println!("Hello from dep_836");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_836");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_836: {t}");
}

pub fn foo() {
    dep_305::code();
    dep_305::code_inlined();
    dep_305::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
}
