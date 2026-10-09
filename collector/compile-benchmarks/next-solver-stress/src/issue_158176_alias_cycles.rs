#![allow(dead_code)]

// Reduced from rust-lang/rust#158176 at nesting depth 16.
struct A;
struct B;
struct C;
type Alias<T, U> = <T as Trait<U>>::Assoc;
trait Trait<T> {
    type Assoc;
}
// Keep the generated nested aliases compact.
#[rustfmt::skip]
fn foo<T>()
where
    T: Trait<A> + Trait<B> + Trait<C>,
    T: Trait<Alias<T, A>>,
    T: Trait<Alias<T, B>>,
    T: Trait<Alias<T, C>>,
    T: Trait<Alias<T, Alias<T, A>>>,
    T: Trait<Alias<T, Alias<T, B>>>,
    T: Trait<Alias<T, Alias<T, C>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, A>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, B>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, C>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, A>>>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, B>>>>>>>>>>>>>>>>>,
    T: Trait<Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, Alias<T, C>>>>>>>>>>>>>>>>>,
{}
fn main() {}
