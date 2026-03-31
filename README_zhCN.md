# rust-godot-startup













## 第一章 业务痛点

### 第01节 遇到的问题

作为一名小白程序员，在第一次使用 rust 和 godot 组合开发的过程中，遇到了下面的问题，在构建第一个项目的过程中，花费了很多的时间，去解决因为环境构建，出现的一系列奇怪的错误。对此花费了许多的时间。作为初学者来说，不应该为环境的问题花费更多的时间。



我相信，有的小伙伴为了这个问题，在初次使用的时候，可以构建一个空白的 rust-godot 结构的git仓库，直接拉取下来之后，可以进行构建，并且运行。

我理解的是这种方式可能存在一些差强人意的地方。

1、不能适配大多数初级开发者。

2、在国内，由于网络原因导致，无法正常访问 如 `github` 仓库

为此，我花费了 10天左右的时间，将 rust godot 结合的部分，进行了从指令到构建的全过程，编写了一个用于一键构建项目的工具。







### 第02节 解决问题

编译出了一套统一的 rust godot 项目构建工具。

能够通过自身的部署操作，一键构建出快速可以运行的 rust  godot 项目

您只需要做出一些简单的配置即可。

例如:

1、cargo 的路径

2、godot 的路径

3、工作空间

4、rust 根目录名称

5、gdext 的名称

6、是否需要创建 Demo 项目

只需要经过上面的几项配置，配置完毕之后，就可以一键构建出 rust godot 项目

整个过程，仅仅只需要花费 2 分钟的时间，就可以完成。









### 第03节  软件说明

1、当前软件，采用的是 Windows 操作系统，您可以通过编译代码，生成 其他操作系统的执行文件。

2、当前软件，支持 9国语言，包括有（英文、简体中文、繁体中文、日语、韩语、德语、法语、西班牙语、意大利语） 

3、配置文件，设置一次之后，后续无需继续设置，有缓存在本地文件中















## 第二章  界面介绍

### 第01节  主界面


<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


主界面, 顶部位置, 主要有一些选项菜单,可以进行一些选择.









### 第02节 配置选择

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />




您可以选择 rust的路径, 以及 godot文件的路径

其中:  

1、rust的路径指的是  cargo 所在的文件夹路径

2、godot的路径指的是 godot的 exe 文件所在



当然，您也可以设置语言。下面点击的是设置软件语言的选项。

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />














### 第03节  项目构建

在项目界面中，您可以开始构建项目


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



根据按钮的顺序，完成构建项目，等待中间，进度条达到 100% 就可以完成了整个项目的构建了。





下面显示一下构建的过程，如下：

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






当所有操作，全部完成时，会显示 `所有操作均已完成`。 表示当前已经构建完毕了。











### 第04节 关于软件

对于当前软件而言，您如果操作过程中，存在一些细节问题，可以通过 关于软件，查阅相关说明文档。


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



使用说明

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />






## 第三章 项目显示

### 第01节 项目文件

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />



### 第02节 godot 文件

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />


### 第03节 godot 运行结果

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />


