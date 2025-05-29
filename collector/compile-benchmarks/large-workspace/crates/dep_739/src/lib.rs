pub fn code() {
    println!("Hello from dep_739");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_739");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_739: {t}");
}

pub fn foo() {
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
    dep_184::code();
    dep_184::code_inlined();
    dep_184::code_generic(1u32);
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
    dep_351::code();
    dep_351::code_inlined();
    dep_351::code_generic(1u32);
}
