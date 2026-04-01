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

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/b9ba9013-4042-493d-8733-032ac9078eea" />


L'interfaccia principale, nella parte superiore, contiene principalmente diversi menu di opzioni dove è possibile effettuare delle scelte.







### Sezione 02 Selezione della configurazione

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/38fc4003-3faa-4874-927a-3bbf4126d7b6" />




È possibile scegliere il percorso di Rust e il percorso del file di Godot.

Dove:

1. Il percorso di Rust si riferisce alla cartella in cui si trova Cargo.

2. Il percorso di Godot si riferisce alla posizione del file eseguibile (.exe) di Godot.

Naturalmente, è anche possibile impostare la lingua. L'opzione per impostare la lingua del software si trova qui sotto.

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/fc22ab09-80ca-469b-8649-44e0a93a6149" />















### Sezione 3 Progetto di costruzione

Nell'interfaccia del progetto, è possibile iniziare a costruire il progetto.


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/e581d0cb-be1f-48db-9797-56c798cb9f63" />


Segui la sequenza di pulsanti per completare la compilazione del progetto. Attendi che la barra di avanzamento raggiunga il 100% per completare l'intera compilazione del progetto.





Il processo di compilazione è illustrato di seguito:

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/2a0a6cce-6a5e-4e83-9640-f670984a024d" />



<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/70885e0d-26a2-469a-8c5e-255fdd20024e" />







Al termine di tutte le operazioni, il sistema visualizzerà il messaggio "Tutte le operazioni sono completate", a indicare che il processo di compilazione è terminato.











### Sezione 4 Informazioni sul software

In caso di problemi durante l'utilizzo del software, è possibile consultare la documentazione pertinente nella sezione "Informazioni sul software".


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/54cd828e-551d-46d4-a4b5-2e7eaad12c4a" />




Istruzioni per l'uso

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/89fd44a6-aeb7-4cf1-88a7-f570b4d5249c" />



















## Capitolo 3 Presentazione del progetto

### Sezione 01 Documenti di progetto

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Sezione 02 file Godot

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Sezione 3: Risultati dell'esecuzione di Godot

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











