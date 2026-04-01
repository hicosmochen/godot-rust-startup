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


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/08a6505f-b13e-4c01-b0a5-48d9aa353e3e" />



Die Hauptschnittstelle oben enthält hauptsächlich mehrere Optionsmenüs, in denen Sie Auswahlen treffen können.









### Abschnitt 02 Konfigurationsauswahl

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/bedb4d57-63f6-49e3-985c-9d4c787e11e8" />


Sie können den Pfad zu Rust und den Pfad zur Godot-Datei auswählen.

Bedeutung:

1. Der Rust-Pfad bezieht sich auf den Ordner, in dem sich Cargo befindet.

2. Der Godot-Pfad bezieht sich auf den Speicherort der Godot-EXE-Datei.

Selbstverständlich können Sie auch die Sprache festlegen. Klicken Sie dazu unten auf die entsprechende Option.

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/f2d8dc85-85b1-419d-a579-cc636d211280" />














### Abschnitt 3 Projekterstellung

In der Projektoberfläche können Sie mit der Erstellung Ihres Projekts beginnen.


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/994d3d1f-ed55-4ced-8027-ed57bac052e3" />




Folgen Sie der Tastenfolge, um den Projekt-Build abzuschließen. Warten Sie, bis der Fortschrittsbalken 100 % erreicht hat, um den gesamten Projekt-Build abzuschließen.





Der Aufbauprozess wird im Folgenden dargestellt:

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/7db00dd5-69a2-402d-bb34-29f35e2db513" />


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/7736c4f2-5dd9-4146-99e8-3d4d5072d4bc" />







Wenn alle Operationen abgeschlossen sind, zeigt das System die Meldung „Alle Operationen sind abgeschlossen“ an, was bedeutet, dass der Build-Prozess nun beendet ist.











### Abschnitt 04 Über die Software

Sollten Sie während der Nutzung dieser Software auf detaillierte Probleme stoßen, können Sie die entsprechende Dokumentation im Abschnitt „Über die Software“ konsultieren.


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/9d5eec9b-9ff2-46c5-acda-785f45f8d335" />




Gebrauchsanweisung

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/6197b81e-52e0-4d92-bc57-14397247f78f" />


















## Kapitel 3 Projektdarstellung

### Abschnitt 01 Projektdateien

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Abschnitt 02 Godot-Datei

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Abschnitt 3: Godot-Ausführungsergebnisse

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











