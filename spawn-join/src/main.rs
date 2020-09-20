use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() {
    // async ブロックを実行します
    let handle = tokio::spawn(async {
        delay_for(Duration::from_millis(1000)).await;
        println!("1 sec elapsed");
    });

    // 即座に実行されます
    println!("hello, world");


    // async ブロックの実行完了まで待機します
    let _ = handle.await.unwrap();
}
