# 海のものとも山のものとも

お勉強用。

最終的にはローカルHTMLヘルプ出力でも作ろうと思っているけどとりま Rust に慣れるところが目的。

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
            "name": "Debug executable 'local-html-help'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=local-html-help",
                    "--package=local-html-help"
                ],
                "filter": {
                    "name": "local-html-help",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'local-html-help'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=local-html-help",
                    "--package=local-html-help"
                ],
                "filter": {
                    "name": "local-html-help",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```
