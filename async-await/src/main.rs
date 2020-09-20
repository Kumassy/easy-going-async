use tokio::time::delay_for;
use std::time::Duration;

async fn hello() {
    delay_for(Duration::from_millis(1000)).await;
    println!("1 sec elapsed");
}

#[tokio::main]
async fn main() {
    // async 関数を実行し、Future の実行が完了するまで待機します
    hello().await;

    // hello() が完了したあとに実行されます
    println!("hello, world");
}
