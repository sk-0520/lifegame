# 海のものとも山のものとも

お勉強用。

最終的にはライフゲームでも作ろうと思っているけどとりま Rust に慣れるところが目的。

---

.vscode/launch.json

```jsonc
{
    // IntelliSense を使用して利用可能な属性を学べます。
    // 既存の属性の説明をホバーして表示します。
    // 詳細情報は次を確認してください: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'lifegame'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=lifegame",
                    "--package=lifegame"
                ],
                "filter": {
                    "name": "lifegame",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'lifegame'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=lifegame",
                    "--package=lifegame"
                ],
                "filter": {
                    "name": "lifegame",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```
