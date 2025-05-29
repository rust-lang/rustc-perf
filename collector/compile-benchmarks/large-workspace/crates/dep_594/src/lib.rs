pub fn code() {
    println!("Hello from dep_594");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_594");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_594: {t}");
}

pub fn foo() {
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
    dep_251::code();
    dep_251::code_inlined();
    dep_251::code_generic(1u32);
    dep_195::code();
    dep_195::code_inlined();
    dep_195::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
}
