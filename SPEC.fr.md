# Cage

> Définition personnelle: Une version libre de MsBuild, basé sur WebAssembly, sans programmes externes et lisible par des humains, sans avoir besoin de Visual Studio, sans écriture bizad..., bref MsBuild en bien.

Ce dépôt présent les spécification de cage: système de compilation et d'empaquetage indépendante de la plateforme et basé sur le web assembly.

Deux fichiers sont présents:
	- un fichier de configuration qui explique les étapes à suivre pour créer le fichier. Écrit par les développeur et lu par l'outils de création.
	- un fichier contenant les version précises des dépendances et compilateurs utilisés, lus et écrit par l'outils de création.

# Objectifs:
	- Aucun appel de programmes externes ou utilisation de ressources de l'ordinateur en dehors du dépôt => complètement indépendant de la plateforme hôte.
	- Décentralisé => pas de dépôt centrale // à par DNS

peut faire:
	- différentes étapes de compilation
	- extrait la documentation
	- langage server / pas d'IDE spécifique
	- les étapes non dépendantes peuvent être concurentes et être lancé sur des ordinateurs différents.
	- génération de code grâce à des compilateur et des

Ce que l'outils ne fait pas
	- Déploiment car très spécifique, les utilisateurs n'en ont pas besoin.

Utilisation massive de l'UTF8.

Création de proggramme de teste.

Une implémenatation d'un orchetrateur peut tous à fait être embarqué dans un programme plus large qui récupère les artéfacte finaux et les publie par exemple.


Il y une base de donnée clé valeur, et un cache. le cache peut être sur l'ordinateur et spécifique à un utilisateur ou partagé.

Les tâches peuvent être supprimées plusieurs fois lors de leur exécution et être exécutées plus tard sur un ordinateur potentiellement différent. Cela est à l'appréciation de l'orchestrateur.

Il faudra pour l'empacketage, que l'on puisse accéder à des informations secrètes comme des clés de chiffrage. Mais cela casse aussi le concepte de ce système qui est conçus pour ne dépendre que des fichiers présent et pas des programmes installés, pour de l'architecture de l'ordinateur. Si on ne dépend que des clés de chiffrement ça peut être simple car on poura les généré au besoin.

Comment peut on définir une architecture pour qu'un utilisateur puisse compiler un programme vers celle-ci.

Les modules WASM exportent plusieurs fonctions qui sont exportées. Chacune est appelée séparément, et peut en appeler et attendre quelles ai fini (ou récupère le résultat mis en cache.) Si une fonction appelle des fonction fille, lorsque la fonction principale a fini, les fonctions filles continues leurs opérations sans avoir besoin de la fonction mère.

Il serai cool de pouvoir faire une version où la sortie est servi par un server web, pour les partie web.

Le fichier de configuration peut contenir des commentaires.

Le fichier de configuration doit commencer (peut-être qu'il peut y avoir des commentaires qui le précède), par une déclaration avec notament le numéro de version.
Idée: cet entête peut contenir '#!' pour conner un interpréteur, le fiochier sera donc un script exécutable.

Un principe général: si les générateurs doivent demander des informations, le fait de demander est exnregistrer et donc cette informations peut servir pour le cache. Cela peut-être appliqué pour les tags.



# Définition

Orchestarteur
: Télécharge les dépendances, et lance les générateurs. Il devrait mettre en cache les opérations intermédiaires pour ne pas avoir à relancer les générateurs.

orchestre le générateurs. Nous considérons dans ce document qu'il tourne sur un unique ordinateur mais rien ne


Générateur
: Programme en web assembly avec l'interface définit ici qui va lire des fichiers, complêter une base de données de code, soit créer des fichiers.


# API

### Journeaux

- fichier annalysé // créer uniquement par l'orchestrateur.
- info
- warning
- error (qui entrâine l'arrêt de l'entièreté de la génération)

L'orchestrateur peut créer un diaggramme de séquence représentant toutes les orpérations et leur journeaux.

### Accès au système de fichier.

Open

Create

readdir

### Base de donnée

Base de donnée clé: string => valeur: []byte

### Lancement sub tâche

Lancement de sous process

### Teste / bench

Ajout d'un teste ou d'un benchmark

### Documentation

Ajout d'une entré dans la documentation.

### Version

Demande des informations concernant les compilateurs, et les version des dépendances utilisés.

### Étiquette

Alors demande si un étiquette est utilisé => permet à l'orchestrateur de mieux mettre en cache sans que les générateurs ai besoin de les déclarer.


# Fichier de configuration

Commentaire: `#` puis le reste de la ligne.

## Version et mot magique

Le fichier doit commencé par `CAGE-BUILD-0\r?\n`.
Peut-il y a avoir des truc commentaire avant.


Il y a trois types: les fichiers, les répertoire et les générateurs.
Le fichier de configuration peut-être lu dans l'ordre, et à chaque instant on connaît l'ensemble des variables utilisé et autre.
Je ne sais pas si on pourra faire des if.

## Opérateur

| Entré      | Sortie     | Opérateur |
| :--------- | :--------- | :-------: |
| répertoire | répertoire |   `>>`    |
| répertoire | fichier    |   `>|`    |
| fichier    | répertoire |   `|>`    |
| fichier    | fichier    |   `||`    |

On peut créer des répertoire par composition avec `{}`, on met le nom des fichiers puis `:` et un fichier ou un répertoire, ou bien par concaténation, avec `[]`, on concatène plusieurs répertoires séparé par des virgules. Il y a aussi les parenthèses pour transformer un flux en un générateur pouvant être utilisé par le programme.

Variable: truc standard, UTF-8 (on est au XXIème siècle), pas d'espace et par trop de caractère bizarre.
Ne peut être redéfini.
Défini `nomVar = valeur`


# Arcvhivage

Idée de créer un fichier avec tout, les dépendances ... pour pouvoir être recompilé à l'avenir. Comment demander au générateur toutes leur dépendances?
