# icecrawlerui


![Livre](https://www.openapis.org/wp-content/uploads/sites/3/2021/12/readme-blue.png "Livre")
* * *

|        Equipe        |                    |
| ---------------------| -------------------|
| Développeur back-end | Auxence DELEUZIERE |
| Développeur front-end| Théo MIGNOTTE      |
| Développeur back-end | Corentin GIGOT     |

* * *


## Description du site
***

>L'objectif de notre projet est de proposer un site vitrine présentant les diverses caratéristiques de l'univers Star Wars tel que les **personnages**, les **planètes** et les **vaisseaux**.

### Au niveau du code

* Front-end
  * Utilisation de *Bootstrap*
  * Utilisation du node module AOS pour les animations
* Back-end
  * Extraction de l'API
  * Créations de deux services :
     ```html <Data>
    data-col.service.ts
    ```
    *pour la récupération de données depuis l'API*.
    ```html <Cache>
    cache.service.ts
    ```
     *pour éviter de depasser le nombre de requête de l'API*.

  * Synchronisations des caches en cascade.
  * Utilisation de la fonction
    ```html <Concat>
    concat();
    ```
    Pour afficher toutes les pages de notre API en fusionnant les sorties de plusieurs observables
  * Implementation d'une **barre de recherche** pour une fluidité de recherche
  * Mise en place d'un système de gestion d'erreurs suivant le modèle **Try/Catch**

  > NB : La vitesse d'affichage des données du site est proportionelle au nombre de requêtes formulées à l'API
* * *
## Comment l'utiliser ?
* * *

> En tant que site vitrine notre site permet exclusivement de lire des informations 

### Compostion :
### ***1. Une page d'accueil possédant :***
  * Un **fond dynamique**
  * Un **header** avec 
    * En haut a gauche un bouton de retour d'accueil
    * En haut à droite ***5 onglets*** accessible par simple clic
  * Un **footer** avec
    * En bas à gauche un avertissement sur les droits reservés par notre site
    * En bas à droite un bouton faisant office de lien de redirection vers le site de l'**ISEN**

  En déroulant cette page vous accederez à des statistiques générales sur le nombre de **personnages**, **vaisseaux**, **planètes** et **soldats** connus dans l'univers de **STAR WARS**.

  Enfin les sabres laser les plus emblématiques de la série vous sont illustrés juste en dessous  dans la section **SABRES LASERS**.

  ### ***2. Cinq onglets subdivisés de la manière suivante :***

  * **'Accueil'** permetant le retour à la home page
  * **'Personnages'** étant une bibliotèque des personnages phares de la série avec leurs principales informations :
    * Nom
    * Films apparus
    * Genre
    * Date de naissance
    * Lieu de vie *( rectangle bleu )*
    * Race *( rectangle jaune )*
    * Galerie photo *( rectangle vert )*

> Galerie est un lien de redirection vers la page image de Google contenant le 'nom' du personnage.

* **'Vaisseaux'** étant une bibliotèque des vaisseaux spaciaux icnoniques de la série avec leurs principales informations :
    * Nom
    * Films apparus
    * Nombre d'équipiers
    * Taille *( longueur )*
    * Vitesse atmosphérique
    * Prix *( rectangle bleu )*
    * Type *( rectangle jaune )*
    * Galerie photo *( rectangle vert )*
> * * *
* **'Planètes'** étant une bibliotèque recensant les planètes connues de la série avec leurs principales informations :
    * Nom
    * Films apparus
    * Diamètre
    * Gravité 
    * Type de terrain
    * Nombre d'habitants *( rectangle bleu )*
    * Galerie photo *( rectangle vert )*



* * *

> Chacune de ces trois précédentes catégories est agrémenté d'une **barre de recherche**  pour vous permettre de trouver votre personnage favoris.
> NB : La page d'accueil est la première visible sur le site ainsi que d'une **remarque exhaustive** concernant la catégorie.
* * *

* **'A propos'** est l'onglet informatif sur le déroulé de la **conception du site** ainsi que la **présentation de l'équipe**.

> Notre équipe vous remercie et espère que le site saura être utile au padawan qui sommeil en vous.

