# rust-godot-startup













## 第1章 ビジネス上の課題

### 第1節 遭遇した問題

プログラミング初心者として、RustとGodotを初めて組み合わせて開発を行った際、以下のような問題に直面しました。最初のプロジェクト構築中に、環境設定に起因する一連の奇妙なエラーの解決に多くの時間を費やしました。初心者としては、環境問題にこれほど時間をかけるべきではありませんでした。

この方法を初めて使う開発者の中には、RustとGodotのGitリポジトリを新規作成し、プル、ビルド、実行できる人もいるでしょう。

しかし、このアプローチにはいくつかの欠点があることを理解しています。

1. ほとんどの初心者開発者には適していません。

2. 中国では、ネットワークの問題により、GitHubなどのリポジトリに正常にアクセスすることができません。

そこで、コマンドからビルドプロセス全体に至るまで、RustとGodotのコンポーネントを統合し、ワンクリックでプロジェクトを構築できるツールの開発に約10日間を費やしました。







### セクション2 問題解決

統一されたRust Godotプロジェクトビルドツールがコンパイルされました。

このツールを使えば、独自のデプロイ操作により、ワンクリックで高速かつ実行可能なRust Godotプロジェクトをビルドできます。

必要な設定はわずかです。

例：

1. Cargoパス

2. Godotパス

3. ワークスペース

4. Rustルートディレクトリ名

5. gdext名

6. デモプロジェクトを作成するかどうか

上記の設定を完了すれば、ワンクリックでRust Godotプロジェクトをビルドできます。

全プロセスはわずか2分で完了します。









### 第3章 ソフトウェアの説明

1. 本ソフトウェアはWindowsオペレーティングシステムを使用します。他のオペレーティングシステム用の実行可能ファイルを生成するために、コードをコンパイルすることができます。

2. 本ソフトウェアは、英語、簡体字中国語、繁体字中国語、日本語、韓国語、ドイツ語、フランス語、スペイン語、イタリア語を含む9言語をサポートしています。

3. 設定ファイルは一度設定すればよく、以降の設定はローカルにキャッシュされます。















## 第2章：インターフェースの概要

### 第1節：メインインターフェース


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/f3fa0072-2688-45a6-ab36-de4d9b67b9de" />



上部にあるメインインターフェースには、主にいくつかのオプションメニューがあり、そこで選択を行うことができます。









### セクション02 構成の選択

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/b2abb049-6e8a-4c1f-8a29-4b41387b6b51" />




RustのインストールパスとGodotファイルのパスを指定できます。

説明：

1. RustのパスはCargoがインストールされているフォルダを指します。

2. GodotのパスはGodotの実行ファイル（.exe）の場所を指します。

もちろん、言語設定も可能です。ソフトウェアの言語設定オプションは、下のリンクをクリックしてください。

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/3821fe35-fdb3-43d0-9565-079eb3cd1835" />














### 第3章 プロジェクトの構築

プロジェクトインターフェースで、プロジェクトの構築を開始できます。


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/f03907e2-c0ef-44f1-983e-00feb938bf85" />




プロジェクトのビルドを完了するには、ボタンの指示に従ってください。プログレスバーが100%に達するまでお待ちください。





ビルドプロセスは以下のとおりです。

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/a031c2cd-8328-4f22-8b18-606112121d1c" />



<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/f7d39f58-3520-40dc-b383-e92f2b93c78f" />







すべての操作が完了すると、システムは「すべての操作が完了しました」と表示し、ビルドプロセスが完了したことを示します。











### 第4項 ソフトウェアについて

本ソフトウェアの操作中に詳細な問題が発生した場合は、「ソフトウェアについて」の項にある関連ドキュメントを参照してください。


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/817f5af3-f76c-49d9-ae1d-9b72528bdc5b" />




使用説明書

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/ebca62a5-3da9-4ce3-8d0e-e68cecd55955" />












## 第3章 プロジェクト表示

### 第1節 プロジェクトファイル

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### セクション02 ゴドーファイル

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />













### 第3章：Godotの実行結果

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />

