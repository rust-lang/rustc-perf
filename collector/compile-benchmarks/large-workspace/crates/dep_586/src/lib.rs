pub fn code() {
    println!("Hello from dep_586");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_586");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_586: {t}");
}

pub fn foo() {
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
    dep_291::code();
    dep_291::code_inlined();
    dep_291::code_generic(1u32);
}
