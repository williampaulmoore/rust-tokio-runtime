use tokio::time;
use log::Level;
use tokio::io::AsyncReadExt;

fn fib(n:i32) -> i32 {
    match n {
      0 => 0,
      1 => 1,
      n => fib(n-1) + fib(n-2),
    }
}


async fn sleep() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading data");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read {} bytes",contents.len());

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Computed fib(40");
    }).await.unwrap();
}

async fn run() {
    tokio::join!(
        sleep(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
    );
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();

    rt.block_on(future);
    let end = std::time::Instant::now();

    println!("Took {:?}", end - start);
}
