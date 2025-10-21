# Integration Test Scripts

このディレクトリには、shishutsukan-rsの統合テストを実行するためのスクリプトが含まれています。

## `run-integration-tests.sh`

このスクリプトは、統合テストを実行するためのものです。自動的に以下のことを行います：

1. [shishutsukan](https://github.com/sfujibijutsukan/shishutsukan)リポジトリをクローンします
2. Python依存関係（FastAPI、uvicorn）をインストールします
3. ポート8000でshishutsukanサーバーを起動します
4. サーバーの準備ができるまで待機します
5. 統合テストを実行します
6. テスト終了後、サーバーを停止して一時ファイルをクリーンアップします

### 前提条件

- Python 3.10以上
- Git
- curl
- Rust および Cargo

### 使い方

```bash
# リポジトリのルートディレクトリから実行
./scripts/run-integration-tests.sh
```

### CI/CD

このスクリプトは、GitHub Actionsワークフローでも使用されています。詳細は `.github/workflows/test.yml` を参照してください。

## 統合テストについて

統合テストは `tests/integration_tests.rs` にあり、以下のテストケースが含まれています：

### ジャンル管理テスト
- `test_get_genres` - ジャンル一覧の取得
- `test_add_genre` - ジャンルの追加
- `test_delete_genre` - ジャンルの削除

### 支出管理テスト
- `test_get_expenses` - 支出データ一覧の取得
- `test_add_expense` - 支出データの追加
- `test_delete_expense` - 支出データの削除

### ワークフローテスト
- `test_complete_workflow` - 完全なワークフローの統合テスト

### エラーハンドリングテスト
- `test_delete_non_existent_expense` - 存在しない支出データの削除
- `test_add_duplicate_genre` - 重複するジャンルの追加
- `test_delete_genre_in_use` - 使用中のジャンルの削除

これらのテストは、実際のshishutsukanサーバーに対して実行され、APIクライアントの動作を検証します。
