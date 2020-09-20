# Rust でお気楽非同期プログラミング
async/await キーワードを使って、おもむろに非同期処理を書いていきましょう！

## async-await: Future を直接に実行する
`Future` を `await` すると、`Future` の実行が完了するまで待機します

実行方法:
```
cargo run
```

## spawn-join: Future を spawn する
`Future` を `spawn` して非同期タスクを生成すると、バックグラウンドで実行してくれます

実行方法:
```
cargo run
```

## github-api: ネットワーク上の REST API にアクセスする
[GitHub API](https://docs.github.com/en/rest/reference/repos#get-a-repository) にアクセスして、リポジトリのスター数を調べます

実行方法:
```
cargo run
```

注意:   
- GitHub API の Rate Limit は 1 時間あたり 60 回までです。短時間にたくさん実行するとエラーが返ります
