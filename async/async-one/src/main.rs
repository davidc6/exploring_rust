use std::future::Future;
use tokio::runtime::Runtime;
use reqwest;

struct Response(reqwest::Response);

impl Response {
    pub async fn text(self) -> String {
        self.0.text().await.unwrap()
    }
}

async fn get(url: &str) -> Response {
    Response(reqwest::get(url).await.unwrap())
}

/// Runs a single future to completion.
///
/// A new instance of tokio Runtime is spun up every time this is called.
/// This is not ideal for production use but a good learning exercise.
fn block_on<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

async fn get_page(url: &str) -> Option<String> {
    // Each await is where the control is handed back to the runtime. There is an invisible 
    // state machine that operates behind the scenes here for Rust to keep track of 
    // the state in the async block. Part of the runtime responsible for the executing 
    // the async code is the executor.
    let res = get(url).await.text().await;
    Some(res)
}

fn main() {
    // A future gets passed into the blocking function call.
    block_on(async {
        let r =  get_page("https://jsonplaceholder.typicode.com/todos/1")
            .await
            .unwrap();
        print!("{r}");
    });
}
