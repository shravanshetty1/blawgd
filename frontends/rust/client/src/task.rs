use anyhow::Result;
use std::future::Future;

pub fn spawn_local<I>(task: I)
where
    I: Future<Output = Result<()>> + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        // TODO handle error
        task.await.unwrap()
    })
}
