use godot::prelude::*;
use godot::classes::{RichTextLabel, IRichTextLabel, Theme}; // 导入需要的 UI 类

use crate::secure::secure_storage::SecureStorage;

#[derive(GodotClass)]
#[class(base=RichTextLabel)] 
pub struct AboutInstructionText {
    base: Base<RichTextLabel>,
    text: GString,
}


#[godot_api]
impl IRichTextLabel for AboutInstructionText {
    fn init(base: Base<RichTextLabel>) -> Self {
        godot_print!("使用说明"); 
        Self {
            base,
            text: GString::from("en-US"),
        }
    }

    fn ready(&mut self) {
        // 将当前节点, 添加到组当中
        self.base_mut().add_to_group("listener_change_language");
        self.text = GString::from(&SecureStorage::get("current_lanague"));

        let content = self.get_content(self.text.clone());
        let mut node = self.base_mut();
        let message = format!("{}", content);
        node.set_text(&message);

        // 加载外部主题文件
        let my_theme = load::<Theme>("res://theme/main_body_label_theme.tres");
        // 应用到主节点，子节点会继承该主题
        node.set_theme(&my_theme);
    }
}

#[godot_api]
impl AboutInstructionText {

    // 改变语言
    #[func]
    fn on_change_language(&mut self, _text: GString){
        self.text = _text.clone();
        let content = self.get_content(self.text.clone());
        let mut node = self.base_mut();
        let message = format!("{}", content);
        node.set_text(&message);
    }


    #[func]
    fn get_content(&mut self, text: GString) -> String {
        // 转换为 &str 进行高效匹配
        let text_str = text.to_string();

        // match 是一个表达式，可以直接返回值给 result
        let result = match text_str.as_str() {
            "en_US" => self.get_content_by_en_us(),
            "zh_CN" => self.get_content_by_zh_cn(),
            "zh_TW" => self.get_content_by_zh_tw(),
            "ja_JP" => self.get_content_by_ja_jp(),
            "ko_KR" => self.get_content_by_ko_kr(),
            "de_DE" => self.get_content_by_de_de(),
            "fr_FR" => self.get_content_by_fr_fr(),
            "es_ES" => self.get_content_by_es_es(),
            "it_IT" => self.get_content_by_it_it(),
            _ => {
                godot_print!("数据有误: 未知的语言代码 {}", text_str);
                String::new() // 默认返回一个空字符串，保证类型一致
            }
        };
        return result;
    }

    #[func]
    fn get_content_by_en_us(&mut self) -> String {
        let content = r#"
*******************************《Instructions for Use》*************************************

1. Function Introduction:

A. Automatically creates the Rust godot project structure.

B. Automatically creates an introductory example and supports its execution.

2. Prerequisites

Before using the software, you need to perform the following configurations:

A. 【Configuration】 --> 【Rust Path】 Locate the .cargo folder. For example: C:\Users\Administrator\.cargo

B. 【Configuration】 --> 【Godot File】 Configure the Godot startup file (exe file). For example: Locate the Godot shortcut.

3. Operation Steps

After completing the prerequisite configurations, you can begin creating the project. You can choose to create a rust-godot project by selecting "Project".

A. Step 1: You need to select a workspace, which is the root directory where the project is located.

B. Step 2: You need to set the name of the rust root directory. Note that the name of the root directory is also the name of the dynamically generated DLL file.

C. Step 3: You need to set the name of gdext. This file is used to associate the dynamic library with the godot project; it is the bridge between rust and godot.

D. Step 4: You can choose to create a case study to build your first NodeHello to quickly get started with rust-godot. This option is optional.

E. Step 5: After completing all the above steps, click "Create Project". This process will take some time; please be patient.

-----------------------------------------------------《Notes》---------------------------------------------------------------------------------

- ​​1. This script was developed by cosmo and uses the rust-godot language.

- 2. Currently only tested on Windows systems; other platforms have not been specifically tested and may cause bugs.

- 3. If you are not familiar with the rust-godot code structure, it is recommended that you select "Requires Creating an Example".

- 4. The project code has been submitted to GitHub. For further support, please contact the author, cosmo.

- 5. The current version of the code only builds a debug version using `cargo build`.

- 6. If you need to release a release version:

First: You need to use `cargo build --release` to build the Rust project.

Second: Specify the path of the DLL in the `gdext` file to point to the release path.

************************************************************************************
        "#;
        return content.to_string();
    }


    #[func]
    fn get_content_by_zh_cn(&mut self) -> String {
        let content = r#"
*******************************《使用须知》*************************************

1. 功能简介：
        A. 自动创建 rust godot 项目结构
        B. 自动创建 入门案例 并且 支持运行

2. 操作前提
        您在使用的软件之前, 需要做下面的几点配置:
        A. 【配置】 --> 【rust路径】  需要找到 .cargo 的文件夹。 例如:  C:\Users\Administrator\.cargo
        B. 【配置】 --> 【Godot文件】 配置 Godot的启动文件 exe 文件。 例如: 找到 Godot 的快捷方式


3. 操作步骤
        当您做完前面的操作前提配置之后, 就可以开始 项目的创建了。您可以选择【项目】来创建 rust-godot 项目
        A. 第一步: 您需要选择工作空间, 工作空间是 项目所在的根目录
        B. 第二步: 您需要设置 RUST 根目录的名称, 需要注意的是 根目录的名称, 也是后期生成的 动态库dll文件的名称
        C. 第三步: 您需要设置 gdext 的名称, 该文件用于关联动态库 和 godot项目, 他是 rust 和 godot 的桥梁
        D. 第四步: 您可以选择创建案例, 来构建第一个 NodeHello 来快速入门 rust-godot  当然这个选项是非必须的
        E. 第五步: 当您操作完毕上面的所有步骤之后, 点击【创建项目】, 该过程将会持续一段时间, 需要您耐心等待

-----------------------------------------------------《注意事项》---------------------------------------------------------------------------------

- 1、当前脚本由开发者 cosmo 开发, 使用语言为 rust - godot 完成。
- 2、目前仅在 windows 系统中完成, 其他平台未做具体测试, 可能会引起 bug 现象
- 3、如果您不够熟悉 rust-godot 代码结构的情况下, 建议您选择 【需要创建案例】
- 4、该项目代码, 已经提交 github, 需要更多支持, 请联系作者 cosmo 
- 5、当前版本的代码, 仅仅构建了 debug 版本 debug版本采用的是 cargo build
- 6、如果您有发布 release 版本的需求 
                第一: 您需要使用  cargo build --release 构建 rust项目
                第二: 指定 gdext文件中 dll路径 指向 release 的路径

************************************************************************************
        "#;
        return content.to_string();
    }



    #[func]
    fn get_content_by_zh_tw(&mut self) -> String {
        let content = r#"
*******************************《使用須知》*************************************

1. 功能簡介：
 A. 自動建立 rust godot 專案結構
 B. 自動建立 入門案例 並且 支援運行

2. 操作前提
 您在使用的軟體之前, 需要做下面的幾點配置:
 A. 【配置】 --> 【rust路徑】 需要找到 .cargo 的資料夾。 例如: C:\Users\Administrator\.cargo
 B. 【設定】 --> 【Godot檔】 配置 Godot的啟動檔 exe 檔。 例如: 找到 Godot 的捷徑


3. 操作步驟
 當您做完前面的操作前提配置之後, 就可以開始 專案的創建了。您可以選擇【項目】來建立 rust-godot 項目
 A. 第一步: 您需要選擇工作空間, 工作空間是 專案所在的根目錄
 B. 第二步: 您需要設定 RUST 根目錄的名稱, 需要注意的是 根目錄的名稱, 也是後期產生的 動態庫dll檔案的名稱
 C. 第三步: 您需要設定 gdext 的名稱, 該檔案用於關聯動態函式庫 和 godot專案, 他是 rust 和 godot 的橋樑
 D. 第四步: 您可以選擇建立案例, 來建立第一個 NodeHello 來快速入門 rust-godot 當然這個選項是非必須的
 E. 第五步: 當您操作完畢上面的所有步驟之後, 點擊【建立專案】, 過程將會持續一段時間, 需要您耐心等待

-----------------------------------------------------《注意事項》--------------------------------------------------------------------------------

- 1、目前腳本由開發者 cosmo 開發, 使用語言為 rust - godot 完成。
- 2、目前僅在 windows 系統中完成, 其他平台未做具體測試, 可能會造成 bug 現象
- 3、如果您不夠熟悉 rust-godot 程式碼結構的情況下, 建議您選擇 【需要建立案例】
- 4、該專案代碼, 已經提交 github, 需要更多支持, 請聯絡作者 cosmo
- 5、目前版本的程式碼, 僅僅建構了 debug 版本 debug版本採用的是 cargo build
- 6、如果您有發布 release 版本的需求
 第一: 您需要使用 cargo build --release 來建立 rust項目
 第二: 指定 gdext檔案中 dll路徑 指向 release 的路徑

************************************************************************************
        "#;
        return content.to_string();
    }


    #[func]
    fn get_content_by_ja_jp(&mut self) -> String {
        let content = r#"
*******************************《使用方法》*****************************************

1. 機能紹介：

A. Rust Godotプロジェクトの構造を自動的に作成します。

B. 入門サンプルを自動的に作成し、実行をサポートします。

2. 前提条件

ソフトウェアを使用する前に、以下の設定を行う必要があります。

A. 【設定】→ 【Rustパス】 .cargoフォルダの場所を指定します。例：C:\Users\Administrator\.cargo

B. 【設定】→ 【Godotファイル】 Godotの起動ファイル（exeファイル）を設定します。例：Godotのショートカットの場所を指定します。

3. 操作手順

前提条件の設定が完了したら、プロジェクトの作成を開始できます。「プロジェクト」を選択して、rust-godotプロジェクトを作成します。

A. ステップ1：ワークスペースを選択する必要があります。ワークスペースとは、プロジェクトが配置されるルートディレクトリです。


B. ステップ2：Rustのルートディレクトリ名を設定する必要があります。ルートディレクトリ名は、動的に生成されるDLLファイルの名前にもなりますのでご注意ください。

C. ステップ3：gdextファイルの名前を設定する必要があります。このファイルは、動的ライブラリをGodotプロジェクトに関連付けるために使用され、RustとGodot間の橋渡し役となります。

D. ステップ4：Rust-Godotをすぐに使い始めるために、最初のNodeHelloを作成するケーススタディを作成することもできます。このオプションは任意です。

E. ステップ5：上記の手順をすべて完了したら、「プロジェクトの作成」をクリックしてください。この処理には時間がかかりますので、しばらくお待ちください。

-----------------------------------------------------《注意事項》---------------------------------------------------------------------------------

- 1. このスクリプトはcosmoによって開発され、Rust-Godot言語を使用しています。

- 2. 現在、Windowsシステムでのみテストされています。他のプラットフォームではテストされておらず、バグが発生する可能性があります。


- 3. rust-godotのコード構造に慣れていない場合は、「サンプルを作成する必要がある」を選択することをお勧めします。

- 4. プロジェクトコードはGitHubに提出済みです。さらにサポートが必要な場合は、作者のcosmoまでお問い合わせください。

- 5. 現在のバージョンのコードは、`cargo build`を使用してデバッグバージョンのみをビルドします。

- 6. リリースバージョンをリリースする必要がある場合：

まず、`cargo build --release`を使用してRustプロジェクトをビルドする必要があります。

次に、`gdext`ファイルでDLLのパスをリリースパスに指定してください。

************************************************************************************
        "#;
        return content.to_string();
    }



    #[func]
    fn get_content_by_ko_kr(&mut self) -> String {
        let content = r#"
*******************************《사용 설명서》*************************************

1. 기능 소개:

A. Rust Godot 프로젝트 구조를 자동으로 생성합니다.

B. 소개 예제를 자동으로 생성하고 실행을 지원합니다.

2. 사전 요구 사항

소프트웨어를 사용하기 전에 다음 설정을 구성해야 합니다.

A. 【설정】 --> 【Rust 경로】 .cargo 폴더의 위치를 ​​지정합니다. 예: C:\Users\Administrator\.cargo

B. 【설정】 --> 【Godot 파일】 Godot 시작 파일(실행 파일)을 설정합니다. 예: Godot 바로 가기의 위치를 ​​지정합니다.

3. 사용 단계

사전 설정을 완료한 후 프로젝트 생성을 시작할 수 있습니다. "프로젝트"를 선택하여 rust-godot 프로젝트를 생성할 수 있습니다.

A. 1단계: 프로젝트가 위치할 루트 디렉토리인 작업 공간을 선택합니다.

B. 2단계: Rust 루트 디렉터리의 이름을 설정해야 합니다. 루트 디렉터리의 이름은 동적으로 생성되는 DLL 파일의 이름과 동일합니다.

C. 3단계: gdext 파일의 이름을 설정해야 합니다. 이 파일은 동적 라이브러리를 Godot 프로젝트와 연결하는 데 사용되며, Rust와 Godot을 연결하는 다리 역할을 합니다.

D. 4단계: Rust-Godot을 빠르게 시작하기 위해 첫 번째 NodeHello를 빌드하는 케이스 스터디를 생성할 수 있습니다. 이 옵션은 선택 사항입니다.

E. 5단계: 위의 모든 단계를 완료한 후 "프로젝트 생성"을 클릭합니다. 이 과정은 다소 시간이 걸릴 수 있으므로 잠시 기다려 주십시오.

-----------------------------------------------------《참고》---------------------------------------------------------------------------------

- ​​​​1. 이 스크립트는 cosmo가 Rust-Godot 언어를 사용하여 개발했습니다.

- 2. 현재 Windows 시스템에서만 테스트되었습니다. 다른 플랫폼에서는 특별히 테스트되지 않았으며 버그가 발생할 수 있습니다.

- 3. rust-godot 코드 구조에 익숙하지 않다면 "예제 생성 필요"를 선택하는 것이 좋습니다.

- 4. 프로젝트 코드는 GitHub에 제출되었습니다. 추가 지원이 필요하면 작성자인 cosmo에게 문의하십시오.

- 5. 현재 버전의 코드는 `cargo build` 명령어를 사용하여 디버그 버전만 빌드할 수 있습니다.

- 6. 릴리스 버전을 배포해야 하는 경우:

첫째, `cargo build --release` 명령어를 사용하여 Rust 프로젝트를 빌드해야 합니다.

둘째, `gdext` 파일에서 DLL 경로를 릴리스 경로로 지정해야 합니다.

************************************************************************************
        "#;
        return content.to_string();
    }


    #[func]
    fn get_content_by_de_de(&mut self) -> String {
        let content = r#"
*******************************《Gebrauchsanweisung》*************************************

1. Funktionsbeschreibung:

A. Erstellt automatisch die Rust-Godot-Projektstruktur.

B. Erstellt automatisch ein einführendes Beispiel und unterstützt dessen Ausführung.

2. Voraussetzungen

Vor der Verwendung der Software müssen Sie folgende Einstellungen vornehmen:

A. 【Konfiguration】 --> 【Rust-Pfad】 Suchen Sie den Ordner .cargo. Beispiel: C:\Users\Administrator\.cargo

B. 【Konfiguration】 --> 【Godot-Datei】 Konfigurieren Sie die Godot-Startdatei (EXE-Datei). Beispiel: Suchen Sie die Godot-Verknüpfung.

3. Vorgehensweise

Nach Abschluss der erforderlichen Einstellungen können Sie mit der Projekterstellung beginnen. Wählen Sie dazu „Projekt“ aus, um ein Rust-Godot-Projekt zu erstellen.

A. Schritt 1: Wählen Sie einen Arbeitsbereich aus. Dies ist das Stammverzeichnis, in dem sich das Projekt befindet.

B. Schritt 2: Legen Sie den Namen des Rust-Stammverzeichnisses fest. Beachten Sie, dass der Name des Stammverzeichnisses auch der Name der dynamisch generierten DLL-Datei ist.

C. Schritt 3: Legen Sie den Namen der gdext-Datei fest. Diese Datei dient der Verknüpfung der dynamischen Bibliothek mit dem Godot-Projekt und bildet die Schnittstelle zwischen Rust und Godot.

D. Schritt 4: Sie können optional eine Fallstudie erstellen, um Ihr erstes NodeHello-Projekt zu entwickeln und so schnell mit rust-godot zu beginnen.

E. Schritt 5: Klicken Sie nach Abschluss aller oben genannten Schritte auf „Projekt erstellen“. Dieser Vorgang kann einige Zeit dauern. Bitte haben Sie Geduld.

-----------------------------------------------------《Hinweise》---------------------------------------------------------------------------------

- ​​1. Dieses Skript wurde von Cosmo entwickelt und verwendet die Sprache rust-godot.

- 2. Derzeit nur unter Windows getestet; andere Plattformen wurden nicht explizit getestet und können Fehler verursachen.

- 3. Wenn Sie mit der Codestruktur von rust-godot nicht vertraut sind, wählen Sie bitte „Beispiel erstellen erforderlich“.

- 4. Der Projektcode wurde auf GitHub veröffentlicht. Für weitere Unterstützung wenden Sie sich bitte an den Autor cosmo.

- 5. Die aktuelle Version des Codes erstellt mit `cargo build` nur eine Debug-Version.

- 6. So veröffentlichen Sie eine Release-Version:

Erstens: Verwenden Sie `cargo build --release`, um das Rust-Projekt zu erstellen.

Zweitens: Geben Sie in der `gdext`-Datei den Pfad der DLL an, sodass er auf das Release-Verzeichnis verweist.

************************************************************************************
        "#;
        return content.to_string();
    }


    #[func]
    fn get_content_by_fr_fr(&mut self) -> String {
        let content = r#"
*******************************《Instructions d'utilisation》*************************************

1. Présentation des fonctionnalités :

A. Crée automatiquement la structure d'un projet Rust Godot.

B. Crée automatiquement un exemple d'introduction et permet son exécution.

2. Prérequis

Avant d'utiliser le logiciel, vous devez effectuer les configurations suivantes :

A. 【Configuration】 --> 【Chemin Rust】 Localisez le dossier .cargo. Par exemple : C:\Users\Administrator\.cargo

B. 【Configuration】 --> 【Fichier Godot】 Configurez le fichier de démarrage Godot (fichier .exe). Par exemple : Localisez le raccourci Godot.

3. Étapes d'utilisation

Une fois les prérequis configurés, vous pouvez commencer à créer le projet. Choisissez de créer un projet rust-godot en sélectionnant « Projet ».

A. Étape 1 : Sélectionnez un espace de travail, c’est-à-dire le répertoire racine où se trouve votre projet.

B. Étape 2 : Indiquez le nom du répertoire racine Rust. Ce nom correspond également au nom du fichier DLL généré dynamiquement.

C. Étape 3 : Indiquez le nom du fichier gdext. Ce fichier permet d’associer la bibliothèque dynamique au projet Godot ; il sert de lien entre Rust et Godot.

D. Étape 4 : Vous pouvez créer une étude de cas pour créer votre premier NodeHello et vous familiariser rapidement avec Rust-Godot. Cette option est facultative.

E. Étape 5 : Une fois toutes les étapes précédentes terminées, cliquez sur « Créer un projet ». Ce processus peut prendre un certain temps ; veuillez patienter.

----------------------------------------------------《Remarques》---------------------------------------------------------------------------------

- ​​1. Ce script a été développé par Cosmo et utilise le langage Rust-Godot.

- 2. Testé actuellement uniquement sur les systèmes Windows. Les autres plateformes n'ont pas été testées spécifiquement et peuvent engendrer des bogues.

- 3. Si vous n'êtes pas familiarisé avec la structure du code rust-godot, il est recommandé de sélectionner « Nécessite la création d'un exemple ».

- 4. Le code du projet a été soumis sur GitHub. Pour obtenir de l'aide, veuillez contacter l'auteur, cosmo.

- 5. La version actuelle du code ne permet de générer qu'une version de débogage avec `cargo build`.

- 6. Si vous souhaitez publier une version stable :

Premièrement : utilisez `cargo build --release` pour compiler le projet Rust.

Deuxièmement : spécifiez le chemin de la DLL dans le fichier `gdext` pour pointer vers le répertoire de publication.

***********************************************************************************
        "#;
        return content.to_string();
    }


    #[func]
    fn get_content_by_es_es(&mut self) -> String {
        let content = r#"
*******************************《Instrucciones de uso》*************************************

1. Introducción a las funciones:

A. Crea automáticamente la estructura del proyecto Rust-Godot.

B. Crea automáticamente un ejemplo introductorio y permite su ejecución.

2. Requisitos previos

Antes de usar el software, debe realizar las siguientes configuraciones:

A. 【Configuración】 --> 【Ruta de Rust】 Localice la carpeta .cargo. Por ejemplo: C:\Usuarios\Administrador\.cargo

B. 【Configuración】 --> 【Archivo Godot】 Configure el archivo de inicio de Godot (archivo ejecutable). Por ejemplo: Localice el acceso directo a Godot.

3. Pasos de operación

Una vez completadas las configuraciones previas, puede comenzar a crear el proyecto. Puede crear un proyecto rust-godot seleccionando "Proyecto".

A. Paso 1: Debes seleccionar un espacio de trabajo, que es el directorio raíz donde se encuentra el proyecto.

B. Paso 2: Debes establecer el nombre del directorio raíz de Rust. Ten en cuenta que el nombre del directorio raíz también es el nombre del archivo DLL generado dinámicamente.

C. Paso 3: Debes establecer el nombre de gdext. Este archivo se utiliza para asociar la biblioteca dinámica con el proyecto Godot; es el puente entre Rust y Godot.

D. Paso 4: Puedes crear un caso práctico para construir tu primer NodeHello y empezar rápidamente con rust-godot. Esta opción es opcional.

E. Paso 5: Después de completar todos los pasos anteriores, haz clic en "Crear proyecto". Este proceso tardará un tiempo; por favor, ten paciencia.

-----------------------------------------------------《Notas》---------------------------------------------------------------------------------

- ​​1. Este script fue desarrollado por cosmo y utiliza el lenguaje rust-godot.

- 2. Actualmente solo se ha probado en sistemas Windows. Otras plataformas no se han probado específicamente y podrían causar errores.

- 3. Si no está familiarizado con la estructura del código de rust-godot, se recomienda seleccionar "Requiere crear un ejemplo".

- 4. El código del proyecto se ha subido a GitHub. Para obtener más ayuda, póngase en contacto con el autor, cosmo.

- 5. La versión actual del código solo genera una versión de depuración con `cargo build`.

- 6. Si necesita publicar una versión estable:

Primero: Debe usar `cargo build --release` para compilar el proyecto Rust.

Segundo: Especifique la ruta de la DLL en el archivo `gdext` para que apunte a la ruta de la versión estable.

************************************************************************************
        "#;
        return content.to_string();
    }


    #[func]
    fn get_content_by_it_it(&mut self) -> String {
        let content = r#"
*******************************《Istruzioni per l'uso》*************************************

1. Introduzione alle funzioni:

A. Crea automaticamente la struttura del progetto Rust Godot.

B. Crea automaticamente un esempio introduttivo e ne supporta l'esecuzione.

2. Prerequisiti

Prima di utilizzare il software, è necessario eseguire le seguenti configurazioni:

A. 【Configurazione】 --> 【Percorso di Rust】 Individuare la cartella .cargo. Ad esempio: C:\Users\Administrator\.cargo

B. 【Configurazione】 --> 【File di Godot】 Configurare il file di avvio di Godot (file .exe). Ad esempio: Individuare il collegamento a Godot.

3. Procedura operativa

Dopo aver completato le configurazioni preliminari, è possibile iniziare a creare il progetto. È possibile scegliere di creare un progetto rust-godot selezionando "Progetto".


A. Passaggio 1: È necessario selezionare un'area di lavoro, ovvero la directory principale in cui si trova il progetto.

B. Passaggio 2: È necessario impostare il nome della directory principale di Rust. Si noti che il nome della directory principale corrisponde anche al nome del file DLL generato dinamicamente.

C. Passaggio 3: È necessario impostare il nome di gdext. Questo file viene utilizzato per associare la libreria dinamica al progetto Godot; funge da ponte tra Rust e Godot.

D. Passaggio 4: È possibile scegliere di creare un case study per realizzare il primo NodeHello e iniziare rapidamente a utilizzare rust-godot. Questa opzione è facoltativa.

E. Passaggio 5: Dopo aver completato tutti i passaggi precedenti, fare clic su "Crea progetto". Questa procedura richiederà del tempo; si prega di essere pazienti.

-----------------------------------------------------《Note》---------------------------------------------------------------------------------

- ​​1. Questo script è stato sviluppato da cosmo e utilizza il linguaggio rust-godot.

- 2. Attualmente testato solo su sistemi Windows; Altre piattaforme non sono state testate specificamente e potrebbero presentare bug.

- 3. Se non hai familiarità con la struttura del codice di rust-godot, ti consigliamo di selezionare "Richiede la creazione di un esempio".

- 4. Il codice del progetto è stato inviato a GitHub. Per ulteriore supporto, contatta l'autore, cosmo.

- 5. La versione corrente del codice compila solo una versione di debug utilizzando `cargo build`.

- 6. Se devi rilasciare una versione di rilascio:

Primo: devi utilizzare `cargo build --release` per compilare il progetto Rust.

Secondo: specifica il percorso della DLL nel file `gdext` in modo che punti al percorso di rilascio.

************************************************************************************
        "#;
        return content.to_string();
    }
}