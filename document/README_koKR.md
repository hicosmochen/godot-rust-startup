# rust-godot-startup













## 1장 비즈니스 문제점

### 01 겪었던 문제점

초보 프로그래머로서 Rust와 Godot을 함께 사용하여 개발하는 첫 경험에서 다음과 같은 문제에 직면했습니다. 첫 프로젝트를 구축하는 과정에서 환경 설정 문제로 인해 발생하는 일련의 이상한 오류를 해결하는 데 많은 시간을 허비했습니다. 초보자가 환경 문제에 그렇게 많은 시간을 허비해서는 안 됐습니다.

처음 Rust와 Godot을 사용할 때 빈 Git 저장소를 생성하고, pull하고, 빌드하고 실행하는 방법을 사용하는 개발자도 있을 것입니다.

하지만 이러한 접근 방식에는 몇 가지 단점이 있습니다.

1. 대부분의 초보 개발자에게는 적합하지 않습니다.

2. 중국에서는 네트워크 문제로 인해 `GitHub`와 같은 저장소에 정상적으로 접속할 수 없습니다.

따라서 저는 Rust와 Godot 구성 요소를 통합하고, 명령어부터 전체 빌드 프로세스까지 지원하는 원클릭 프로젝트 빌드 도구를 개발하는 데 약 10일을 투자했습니다.







### 섹션 2 문제 해결

통합 Rust Godot 프로젝트 빌드 도구가 완성되었습니다.

이 도구는 자체 배포 기능을 통해 단 한 번의 클릭으로 빠르고 실행 가능한 Rust Godot 프로젝트를 빌드할 수 있습니다.

몇 가지 간단한 설정만 하면 됩니다.

예시:

1. Cargo 경로

2. Godot 경로

3. 워크스페이스

4. Rust 루트 디렉토리 이름

5. gdext 이름

6. 데모 프로젝트 생성 여부

위 설정을 완료하면 단 한 번의 클릭으로 Rust Godot 프로젝트를 빌드할 수 있습니다.

전체 과정은 단 2분밖에 걸리지 않습니다.









### 섹션 3 소프트웨어 설명

1. 이 소프트웨어는 Windows 운영 체제를 사용합니다. 다른 운영 체제용 실행 파일을 생성하려면 코드를 컴파일하십시오.

2. 이 소프트웨어는 영어, 중국어 간체, 중국어 번체, 일본어, 한국어, 독일어, 프랑스어, 스페인어, 이탈리아어를 포함한 9개 언어를 지원합니다.

3. 설정 파일은 한 번만 설정하면 되며, 이후 설정은 로컬에 저장됩니다.















## 제2장: 인터페이스 소개

### 제1절: 메인 인터페이스


<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


상단에 있는 메인 인터페이스에는 주로 여러 옵션 메뉴가 있으며, 여기에서 선택을 할 수 있습니다.









### 第02节 配置选择

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />



Rust 설치 경로와 Godot 실행 파일 경로를 선택할 수 있습니다.

다음과 같이 경로를 지정하세요.

1. Rust 설치 경로는 Cargo가 설치된 폴더를 의미합니다.

2. Godot 실행 파일(.exe)의 위치를 의미합니다.

물론 언어도 설정할 수 있습니다. 아래 옵션을 클릭하여 소프트웨어 언어를 설정하세요.

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />













### 섹션 3 프로젝트 구축

프로젝트 인터페이스에서 프로젝트 구축을 시작할 수 있습니다.


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



아래 그림과 같이 버튼 순서를 따라 프로젝트 빌드를 완료하세요. 진행률 표시줄이 100%에 도달하면 프로젝트가 완료된 것입니다.

빌드 과정은 아래와 같습니다.

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






모든 작업이 완료되면 시스템에 "모든 작업이 완료되었습니다"라는 메시지가 표시되어 빌드 프로세스가 완료되었음을 나타냅니다.











### 제4장 소프트웨어에 대하여

소프트웨어 사용 중 문제가 발생하면 "소프트웨어 정보" 섹션의 관련 문서를 참조하십시오.


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />

사용 설명서

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />


















## 제3장 프로젝트 발표

### 섹션 01 프로젝트 문서

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### 섹션 02 고도 파일

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### 섹션 3: Godot 실행 결과

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











