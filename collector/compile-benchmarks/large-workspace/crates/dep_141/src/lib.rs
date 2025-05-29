pub fn code() {
    println!("Hello from dep_141");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_141");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_141: {t}");
}

pub fn foo() {
    dep_55::code();
    dep_55::code_inlined();
    dep_55::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
    dep_14::code();
    dep_14::code_inlined();
    dep_14::code_generic(1u32);
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
}
