# rust-godot-startup













## 第一章 業務痛點

### 第01節 遇到的問題

作為一名小白程式設計師，在第一次使用 rust 和 godot 組合開發的過程中，遇到了下面的問題，在構建第一個專案的過程中，花費了很多的時間，去解決因為環境構建，出現的一系列奇怪的錯誤。對此花費了許多的時間。作為初學者來說，不應該為環境的問題花費更多的時間。



我相信，有的小夥伴為了這個問題，在初次使用的時候，可以構建一個空白的 rust-godot 結構的git倉庫，直接拉取下來之後，可以進行構建，並且運行。

我理解的是這種方式可能存在一些差強人意的地方。

1.不能適配大多數初級開發者。

2.在國內，因網路原因導致，無法正常存取 如 `github` 倉庫

為此，我花了 10天左右的時間，將 rust godot 結合的部分，進行了從指令到構建的整個過程，編寫了一個用於一鍵構建專案的工具。







### 第02節 解決問題

編譯出了一套統一的 rust godot 專案建置工具。

能夠透過自身的部署操作，一鍵建置出快速可運行的 rust godot 項目

您只需要做出一些簡單的配置。

例如:

1、cargo 的路徑

2、godot 的路徑

3.工作空間

4、rust 根目錄名稱

5、gdext 的名稱

6.是否需要建立 Demo 項目

只需要經過上面的幾個配置，配置完畢之後，就可以一鍵建構出 rust godot 項目

整個過程，只需要花費 2 分鐘的時間，就可以完成。









### 第03節 軟體說明

1.目前軟體，採用的是 Windows 作業系統，您可以透過編譯程式碼，產生 其他作業系統的執行檔。

2.目前軟體，支援 9國語言，包括有（英文、簡體中文、繁體中文、日文、韓文、德文、法文、西班牙文、義大利文）

3、配置文件，設置一次之後，後續無需繼續設置，有緩存在本地文件中















## 第二章 介面介紹

### 第01節 主介面


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/fda18f1d-cae5-4cc2-97d3-a0c5c5131320" />



主介面, 頂部位置, 主要有一些選項選單,可以進行一些選擇.









### 第02節 配置選擇

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/da38a176-c9bb-4aec-9fa8-ab102c43aa26" />




您可以選擇 rust的路徑, 以及 godot檔的路徑

其中:

1.rust的路徑指的是 cargo 所在的資料夾路徑

2、godot的路徑指的是 godot的 exe 檔案所在



當然，您也可以設定語言。下面點選的是設定軟體語言的選項。

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/767f8d11-f342-40dc-be14-42acebe94057" />














### 第03節 專案構建

在專案介面中，您可以開始建立項目


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/41013f10-cb03-4174-9da5-055f66868f90" />




根據按鈕的順序，完成建置項目，等待中間，進度條達到 100% 就可以完成了整個專案的建置了。





下面顯示建置的過程，如下：

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/9931a9de-894b-44d0-88e9-00de2bb31b16" />



<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/84ab8dfd-d815-4542-b42b-acda8e947b7e" />







當所有操作，全部完成時，會顯示 `所有操作已完成`。 表示當前已經建構完畢了。











### 第04節 關於軟體

對於目前軟體而言，您如果操作過程中，存在一些細節問題，可以透過 關於軟體，查閱相關說明文件。


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/a70e8cbb-e705-4cf5-b4ae-40eeb4dc4207" />




使用說明

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/1c8cc651-243d-4d5c-9739-65d3fe70f8a1" />
















## 第三章 項目顯示

### 第01節 專案文件

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### 第02節 godot 文件

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### 第03節 godot 運行結果

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











