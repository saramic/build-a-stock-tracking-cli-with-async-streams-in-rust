use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path("hello").map(|| format!("Hello, World!"));

    println!("listening on http://127.0.0.1:1337\nbut only serving http://127.0.0.1:1337/hello");
    warp::serve(hi).run(([127, 0, 0, 1], 1337)).await
}
