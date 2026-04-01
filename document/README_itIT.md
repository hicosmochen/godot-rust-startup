# rust-godot-startup













## Capitolo 1 Punti critici aziendali

### Sezione 1 Problemi riscontrati

Da programmatore principiante, ho riscontrato i seguenti problemi durante il mio primo progetto di sviluppo con Rust e Godot. Ho passato molto tempo a risolvere una serie di strani errori causati dalla configurazione dell'ambiente. Da principiante, non avrei dovuto dedicare così tanto tempo a problemi di ambiente.

Credo che alcuni, al primo utilizzo, possano creare un repository Git Rust-Godot vuoto, scaricarlo, compilarlo ed eseguirlo.

Tuttavia, capisco che questo metodo presenta alcune limitazioni:

1. Non è adatto alla maggior parte degli sviluppatori principianti.

2. In Cina, i problemi di rete impediscono il normale accesso a repository come GitHub.

Pertanto, ho dedicato circa 10 giorni allo sviluppo di uno strumento per la compilazione di progetti con un solo clic, integrando i componenti Rust e Godot, dai comandi all'intero processo di compilazione.







### Sezione 2 Risoluzione dei problemi

È stato compilato uno strumento unificato per la compilazione di progetti Rust Godot.

Consente di creare un progetto Rust Godot rapido ed eseguibile con un solo clic, grazie alle sue operazioni di distribuzione integrate.

È sufficiente effettuare alcune semplici configurazioni.

Ad esempio:

1. Percorso di Cargo

2. Percorso di Godot

3. Area di lavoro

4. Nome della directory principale di Rust

5. Nome di gdext

6. Se creare o meno un progetto demo

Dopo aver completato le configurazioni di cui sopra, è possibile compilare un progetto Rust Godot con un solo clic.

L'intero processo richiede solo 2 minuti.









### Sezione 3 Descrizione del software

1. Questo software utilizza il sistema operativo Windows. È possibile compilare il codice per generare file eseguibili per altri sistemi operativi.

2. Questo software supporta 9 lingue, tra cui (inglese, cinese semplificato, cinese tradizionale, giapponese, coreano, tedesco, francese, spagnolo e italiano).

3. Il file di configurazione deve essere impostato una sola volta; le impostazioni successive vengono memorizzate localmente nella cache.















## Capitolo due: Introduzione all'interfaccia

### Sezione 01 Interfaccia principale

<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />

L'interfaccia principale, nella parte superiore, contiene principalmente diversi menu di opzioni dove è possibile effettuare delle scelte.







### Sezione 02 Selezione della configurazione

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />



È possibile scegliere il percorso di Rust e il percorso del file di Godot.

Dove:

1. Il percorso di Rust si riferisce alla cartella in cui si trova Cargo.

2. Il percorso di Godot si riferisce alla posizione del file eseguibile (.exe) di Godot.

Naturalmente, è anche possibile impostare la lingua. L'opzione per impostare la lingua del software si trova qui sotto.

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />














### Sezione 3 Progetto di costruzione

Nell'interfaccia del progetto, è possibile iniziare a costruire il progetto.


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />

Segui la sequenza di pulsanti per completare la compilazione del progetto. Attendi che la barra di avanzamento raggiunga il 100% per completare l'intera compilazione del progetto.





Il processo di compilazione è illustrato di seguito:

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






Al termine di tutte le operazioni, il sistema visualizzerà il messaggio "Tutte le operazioni sono completate", a indicare che il processo di compilazione è terminato.











### Sezione 4 Informazioni sul software

In caso di problemi durante l'utilizzo del software, è possibile consultare la documentazione pertinente nella sezione "Informazioni sul software".


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



Istruzioni per l'uso

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />


















## Capitolo 3 Presentazione del progetto

### Sezione 01 Documenti di progetto

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Sezione 02 file Godot

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Sezione 3: Risultati dell'esecuzione di Godot

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











