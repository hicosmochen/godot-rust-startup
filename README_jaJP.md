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


<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


上部にあるメインインターフェースには、主にいくつかのオプションメニューがあり、そこで選択を行うことができます。









### セクション02 構成の選択

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />



RustのインストールパスとGodotファイルのパスを指定できます。

説明：

1. RustのパスはCargoがインストールされているフォルダを指します。

2. GodotのパスはGodotの実行ファイル（.exe）の場所を指します。

もちろん、言語設定も可能です。ソフトウェアの言語設定オプションは、下のリンクをクリックしてください。

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />













### 第3章 プロジェクトの構築

プロジェクトインターフェースで、プロジェクトの構築を開始できます。


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



プロジェクトのビルドを完了するには、ボタンの指示に従ってください。プログレスバーが100%に達するまでお待ちください。





ビルドプロセスは以下のとおりです。

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






すべての操作が完了すると、システムは「すべての操作が完了しました」と表示し、ビルドプロセスが完了したことを示します。











### 第4項 ソフトウェアについて

本ソフトウェアの操作中に詳細な問題が発生した場合は、「ソフトウェアについて」の項にある関連ドキュメントを参照してください。


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



使用説明書

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />











## 第3章 プロジェクト表示

### 第1節 プロジェクトファイル

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### セクション02 ゴドーファイル

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />













### 第3章：Godotの実行結果

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />

