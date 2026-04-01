# rust-godot-startup













## Kapitel 1: Geschäftliche Herausforderungen

### Abschnitt 01: Aufgetretene Probleme

Als Programmierneuling stieß ich bei meinen ersten Erfahrungen mit Rust und Godot in der Entwicklung auf folgende Probleme: Ich verbrachte viel Zeit damit, eine Reihe seltsamer Fehler zu beheben, die durch die Einrichtung der Entwicklungsumgebung während meines ersten Projekts verursacht wurden. Als Anfänger hätte ich nicht so viel Zeit mit Umgebungsproblemen verbringen sollen.

Ich glaube, manche Entwickler erstellen bei ihrer ersten Anwendung ein leeres Rust-Godot-Git-Repository, laden es herunter, kompilieren es und führen es aus.

Mir ist jedoch bewusst, dass dieser Ansatz einige Nachteile hat.

1. Er ist für die meisten Anfänger ungeeignet.

2. In China ist der Zugriff auf Repositories wie GitHub aufgrund von Netzwerkproblemen nicht ohne Weiteres möglich.

Deshalb entwickelte ich innerhalb von etwa zehn Tagen ein Tool für die Ein-Klick-Projekterstellung, das die Rust- und Godot-Komponenten von den Befehlen bis zum gesamten Build-Prozess integriert.







### Abschnitt 2 Problemlösung

Ein einheitliches Tool zum Erstellen von Rust-Godot-Projekten wurde kompiliert.

Es erstellt mit nur einem Klick ein schnelles und lauffähiges Rust-Godot-Projekt dank eigener Bereitstellungsfunktionen.

Sie müssen lediglich einige wenige Einstellungen vornehmen.

Zum Beispiel:

1. Cargo-Pfad

2. Godot-Pfad

3. Arbeitsbereich

4. Name des Rust-Stammverzeichnisses

5. Name von gdext

6. Soll ein Demo-Projekt erstellt werden?

Nachdem Sie die obigen Einstellungen vorgenommen haben, können Sie mit einem Klick ein Rust-Godot-Projekt erstellen.

Der gesamte Vorgang dauert nur 2 Minuten.









### Abschnitt 3 Softwarebeschreibung

1. Diese Software verwendet das Windows-Betriebssystem. Sie können den Code kompilieren, um ausführbare Dateien für andere Betriebssysteme zu erstellen.

2. Diese Software unterstützt neun Sprachen: Englisch, Vereinfachtes Chinesisch, Traditionelles Chinesisch, Japanisch, Koreanisch, Deutsch, Französisch, Spanisch und Italienisch.

3. Die Konfigurationsdatei muss nur einmalig eingerichtet werden; nachfolgende Einstellungen werden lokal zwischengespeichert.















## Kapitel 2: Einführung in die Benutzeroberfläche

### Abschnitt 01: Hauptbenutzeroberfläche


<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


Die Hauptschnittstelle oben enthält hauptsächlich mehrere Optionsmenüs, in denen Sie Auswahlen treffen können.









### Abschnitt 02 Konfigurationsauswahl

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />

Sie können den Pfad zu Rust und den Pfad zur Godot-Datei auswählen.

Bedeutung:

1. Der Rust-Pfad bezieht sich auf den Ordner, in dem sich Cargo befindet.

2. Der Godot-Pfad bezieht sich auf den Speicherort der Godot-EXE-Datei.

Selbstverständlich können Sie auch die Sprache festlegen. Klicken Sie dazu unten auf die entsprechende Option.

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />













### Abschnitt 3 Projekterstellung

In der Projektoberfläche können Sie mit der Erstellung Ihres Projekts beginnen.


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



Folgen Sie der Tastenfolge, um den Projekt-Build abzuschließen. Warten Sie, bis der Fortschrittsbalken 100 % erreicht hat, um den gesamten Projekt-Build abzuschließen.





Der Aufbauprozess wird im Folgenden dargestellt:

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






Wenn alle Operationen abgeschlossen sind, zeigt das System die Meldung „Alle Operationen sind abgeschlossen“ an, was bedeutet, dass der Build-Prozess nun beendet ist.











### Abschnitt 04 Über die Software

Sollten Sie während der Nutzung dieser Software auf detaillierte Probleme stoßen, können Sie die entsprechende Dokumentation im Abschnitt „Über die Software“ konsultieren.


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



Gebrauchsanweisung

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />

















## Kapitel 3 Projektdarstellung

### Abschnitt 01 Projektdateien

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Abschnitt 02 Godot-Datei

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Abschnitt 3: Godot-Ausführungsergebnisse

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











