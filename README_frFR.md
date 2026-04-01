# rust-godot-startup













## Chapitre 1 Points sensibles

### Section 01 Problèmes rencontrés

En tant que programmeur débutant, lors de ma première expérience de développement avec Rust et Godot, j'ai rencontré les problèmes suivants. J'ai passé beaucoup de temps à résoudre une série d'erreurs étranges dues à la configuration de l'environnement lors de la création de mon premier projet. En tant que débutant, je n'aurais pas dû consacrer autant de temps à ces problèmes.

Je pense que certains développeurs, lorsqu'ils utilisent cette méthode pour la première fois, peuvent créer un dépôt Git Rust-Godot vierge, le récupérer, le compiler et l'exécuter.

Cependant, je comprends que cette approche présente des inconvénients.

1. Elle ne convient pas à la plupart des développeurs débutants.

2. En Chine, en raison de problèmes de réseau, il est impossible d'accéder normalement à des dépôts comme GitHub.

C'est pourquoi j'ai passé environ 10 jours à développer un outil permettant de créer un projet en un clic, intégrant les composants Rust et Godot, des commandes à l'ensemble du processus de compilation.







### Section 2 Résolution de problèmes

Un outil unifié de création de projets Rust et Godot a été compilé.

Il permet de créer un projet Rust et Godot rapide et fonctionnel en un seul clic grâce à ses propres opérations de déploiement.

Quelques configurations simples suffisent.

Par exemple :

1. Chemin de Cargo

2. Chemin de Godot

3. Espace de travail

4. Nom du répertoire racine Rust

5. Nom du fichier gdext

6. Création ou non d'un projet de démonstration

Une fois ces configurations effectuées, vous pouvez créer un projet Rust et Godot en un seul clic.

L'ensemble du processus ne prend que 2 minutes.









### Section 3 Description du logiciel

1. Ce logiciel utilise le système d'exploitation Windows. Vous pouvez compiler le code pour générer des fichiers exécutables compatibles avec d'autres systèmes d'exploitation.

2. Ce logiciel prend en charge 9 langues : anglais, chinois simplifié, chinois traditionnel, japonais, coréen, allemand, français, espagnol et italien.

3. Le fichier de configuration n'a besoin d'être défini qu'une seule fois ; les paramètres suivants sont enregistrés localement.















## Chapitre 2 : Introduction à l’interface

### Section 1 : Interface principale


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/5f85a5d9-392a-4b83-b374-13957f202412" />



L'interface principale, en haut, contient principalement plusieurs menus d'options où vous pouvez faire des choix.









### Section 02 Sélection de la configuration

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/f3174b2b-02e1-4758-8815-9d8b7ea81177" />




Vous pouvez choisir le chemin d'accès à Rust et celui au fichier godot.

Où :

1. Le chemin d'accès à Rust correspond au dossier où se trouve Cargo.

2. Le chemin d'accès à godot correspond à l'emplacement du fichier .exe de godot.

Vous pouvez également choisir la langue. Cliquez sur l'option correspondante ci-dessous.

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/fa97e091-6571-4a4e-8a23-5cafda870a7a" />














### Section 3 Construction du projet

Dans l'interface du projet, vous pouvez commencer à construire votre projet.


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/fac60915-6b4e-4918-8404-f07b8dcec25d" />




Suivez la séquence de boutons pour terminer la compilation du projet. Attendez que la barre de progression atteigne 100 % pour indiquer que le projet est terminé.

Le processus de compilation est illustré ci-dessous :

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/0daddaea-6e71-4318-9c3f-5a304bbf3553" />



<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/88cb767a-a5da-46f1-bff6-92f8a2bbceae" />







Une fois toutes les opérations terminées, le système affichera « Toutes les opérations sont terminées », indiquant que le processus de compilation est maintenant achevé.











### Section 4 À propos des logiciels

Si vous rencontrez des problèmes lors de l'utilisation du logiciel, vous pouvez vous référer à la documentation correspondante dans la section « À propos du logiciel ».


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/5d80e27c-7b8c-42b5-bad8-e58a4d875273" />




Instructions d'utilisation

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/89ebccd2-d83c-4346-9ce2-8dc439441b56" />


















## Chapitre 3 : Présentation du projet

### Section 1 : Fichiers du projet

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Section 02 fichier godot

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Section 3 : Résultats d’exécution de Godot

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











