use std::future::Future;
use std::process::Output;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

lazy_static! {
    static ref RUNTIME: Runtime = setup_tokio_runtime();
}

fn setup_tokio_runtime() -> Runtime {
    Runtime::new().unwrap()
}

pub fn ensure_ready() {
    let handle = RUNTIME.spawn(async {
        10
    });
    RUNTIME.block_on(handle).expect("can complete simple task");
}

pub fn in_parallel<A, F>(body: F) -> JoinHandle<A>
    where A: Send + 'static,
        F: FnOnce() -> A + Send + 'static
{
    RUNTIME.spawn_blocking(body)
}

pub fn block_on<A, Fut>(body: Fut) -> A
    where
        A: Send + 'static,
        Fut: Future<Output=A> + Send + 'static {
    RUNTIME.block_on(body)
}