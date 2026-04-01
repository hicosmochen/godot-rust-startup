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


<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


L'interface principale, en haut, contient principalement plusieurs menus d'options où vous pouvez faire des choix.









### Section 02 Sélection de la configuration

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />



Vous pouvez choisir le chemin d'accès à Rust et celui au fichier godot.

Où :

1. Le chemin d'accès à Rust correspond au dossier où se trouve Cargo.

2. Le chemin d'accès à godot correspond à l'emplacement du fichier .exe de godot.

Vous pouvez également choisir la langue. Cliquez sur l'option correspondante ci-dessous.

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />













### Section 3 Construction du projet

Dans l'interface du projet, vous pouvez commencer à construire votre projet.


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



Suivez la séquence de boutons pour terminer la compilation du projet. Attendez que la barre de progression atteigne 100 % pour indiquer que le projet est terminé.

Le processus de compilation est illustré ci-dessous :

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






Une fois toutes les opérations terminées, le système affichera « Toutes les opérations sont terminées », indiquant que le processus de compilation est maintenant achevé.











### Section 4 À propos des logiciels

Si vous rencontrez des problèmes lors de l'utilisation du logiciel, vous pouvez vous référer à la documentation correspondante dans la section « À propos du logiciel ».


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



Instructions d'utilisation

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />

















## Chapitre 3 : Présentation du projet

### Section 1 : Fichiers du projet

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />













### Section 02 fichier godot

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />















### Section 3 : Résultats d’exécution de Godot

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











