# Rust製簡易家計簿アプリ (CLI版)

[Rust](https://www.rust-lang.org/ja)の学習用に作成したCLIアプリケーションです。

## 環境構築

[公式ページ](https://www.rust-lang.org/ja/tools/install)を参考にRustをインストールします。Linux環境であれば、以下のコマンドでインストールできます。

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 実行方法

実行する前に、本ディレクトリ直下に新規でディレクトリ `store` を作成します。

```shell
$ mkdir store
```

以下のコマンドを実行することで、アプリケーションを実行できます。

```shell
$ cargo run
```

また、以下のコマンドを実行すると、ドキュメンテーションコメントからAPIリファレンスを作成できます。

```shell
$ cargo doc --no-deps <--document-private-items>
```