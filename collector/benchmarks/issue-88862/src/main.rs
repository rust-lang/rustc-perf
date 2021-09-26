#[derive(Debug)]
struct Error;
struct SharedPool {
    a: PoolOptions,
}

struct PoolOptions;

async fn check<'s: 'p, 'p>(
    mut _conn: Floating<'s>,
    _: &'p PoolOptions,
) -> Result<(), DecrementSizeGuard<'s>> {
    todo!()
}

impl SharedPool {
    async fn acquire(&'_ self) -> Result<Floating<'_>, Error> {
        async {
            if let conn = todo!() {
                if let d = check(conn, &self.a).await {}
            }
        }
        .await;
        todo!()
    }
}

struct G;
struct Floating<'s>(std::marker::PhantomData<dyn Fn(&'s ())>);
impl<'s> Floating<'s> {
    fn attach(self, e: &std::sync::Arc<SharedPool>) -> G { todo!() }
}

struct DecrementSizeGuard<'s>(std::marker::PhantomData<dyn Fn(&'s ())>);
struct FooDb;
impl FooDb {
    fn acquire(&self) -> impl std::future::Future<Output = Result<G, Error>> + 'static {
        let _shared: std::sync::Arc<SharedPool> = todo!();
        async move { _shared.acquire().await.map(|conn| conn.attach(&_shared)) }
    }

    async fn nested(&self) -> Result<Option<String>, ()> {
        (async move {
            match async move {
                async move {
                    (
                        async move {
                            match async move {
                                let db = FooDb;
                                let mut _conn = db.acquire().await.unwrap();
                                Ok::<_, ()>(String::default())
                            }
                            .await
                            {
                                Ok(x) => Ok(x),
                                Err0 => todo!(),
                            }
                        },
                        todo!(),
                    )
                        .0
                        .await
                }
                .await
                .map(Some)
            }
            .await
            {
                Ok(x) => Ok(x),
                Err(e) => Err(e),
            }
        },)
            .0
            .await
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    factory(Factory::handle(route));
    todo!()
}

fn factory<X, Req>(factory: X) -> BoxServiceFactory<Req>
where X: ServiceFactory<Req> + 'static {
    BoxServiceFactory(Box::new(FactoryWrapper(factory)))
}


async fn route(database: FooDb) -> Result<(), Error> {
    database.nested().await;
    todo!()
}

struct Factory<I, T, R> {
    _hnd: I,
    j: std::marker::PhantomData<(T, R)>,
}

impl<I, T, R> Factory<I, T, R> {
    fn handle(_: I) -> Self { todo!() }
}

struct K;
struct L;
impl<I, T, R> ServiceFactory<K> for Factory<I, T, R>
where
    I: N<T, R>,
    R: std::future::Future,
    R::Output: Responder,
{
    type Future = std::future::Ready<Result<Self, ()>>;
    type Service = Self;

    fn new_service(&self, _: ()) -> Self::Future { todo!() }
}

impl<I, T, R> Service<K> for Factory<I, T, R> {
    type Error = Error;
    type Future = std::future::Ready<Result<L, Error>>;

    fn poll(&self, _: &mut core::task::Context) -> core::task::Poll<Result<(), Error>> { todo!() }

    fn call(&self, m: K) -> Self::Future { todo!() }
}

trait Responder {}

impl<T, R> Responder for Result<T, R> {}

trait N<T, R>: 'static
where
    R: std::future::Future,
    R::Output: Responder, {
    fn call(&self, param: T) -> R;
}


type BoxFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T>>>;

struct BoxServiceFactory<Req>(
    Box<
        dyn ServiceFactory<
            Req,
            Service = Box<dyn Service<Req, Error = (), Future = BoxFuture<Result<(), ()>>>>,
            Future = BoxFuture<
                Result<Box<dyn Service<Req, Error = (), Future = BoxFuture<Result<(), ()>>>>, ()>,
            >,
        >,
    >,
);

trait ServiceFactory<Req> {
    type Service: Service<Req>;
    type Future;
    fn new_service(&self, _: ()) -> Self::Future;
}

trait Service<Req> {
    type Error;
    type Future;
    fn poll(&self, ctx: &mut std::task::Context) -> std::task::Poll<Result<(), Self::Error>>;
    fn call(&self, req: Req) -> Self::Future;
}

impl<S, Req> Service<Req> for Box<S>
where S: Service<Req> + ?Sized
{
    type Error = S::Error;
    type Future = S::Future;

    fn poll(&self, _: &mut std::task::Context) -> std::task::Poll<Result<(), S::Error>> { todo!() }

    fn call(&self, _: Req) -> S::Future { todo!() }
}

impl<SF, Req> ServiceFactory<Req> for FactoryWrapper<SF>
where
    SF: ServiceFactory<Req>,
    SF: 'static,
{
    type Future = BoxFuture<Result<Self::Service, ()>>;
    type Service = Box<dyn Service<Req, Error = (), Future = BoxFuture<Result<(), ()>>>>;

    fn new_service(&self, _: ()) -> Self::Future { todo!() }
}

impl<Func, A, Res> N<(A,), Res> for Func
where
    Func: Fn(A) -> Res + 'static,
    Res: std::future::Future,
    Res::Output: Responder,
{
    fn call(&self, _: (A,)) -> Res { todo!() }
}

struct FactoryWrapper<T>(T);
