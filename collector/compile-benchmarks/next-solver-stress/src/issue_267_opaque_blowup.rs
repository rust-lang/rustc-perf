// Reduced from rust-lang/trait-system-refactor-initiative#267.
//
// The opaque type used to change at each level, preventing otherwise identical
// goals from hitting the solver cache and making candidate evaluation
// exponential. Fixed by rust-lang/rust#155443.

trait Distribution<T> {}

impl Distribution<()> for u32 {}

impl<A, B> Distribution<(A, B)> for u32
where
    u32: Distribution<A>,
    u32: Distribution<B>,
{
}

trait Trait {
    type Item;
}

impl<T> Trait for Option<T>
where
    u32: Distribution<T>,
{
    type Item = T;
}

fn random_paulis() -> impl Trait<Item = ()> {
    None
}
