pub fn code() {
    println!("Hello from dep_566");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_566");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_566: {t}");
}

pub fn foo() {
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
    dep_348::code();
    dep_348::code_inlined();
    dep_348::code_generic(1u32);
    dep_209::code();
    dep_209::code_inlined();
    dep_209::code_generic(1u32);
    dep_262::code();
    dep_262::code_inlined();
    dep_262::code_generic(1u32);
}
