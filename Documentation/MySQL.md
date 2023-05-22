# Kubernetes

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec **MySQL**

## Afficher une base de données

Ouvrir le terminal du pod mysql

```bash
mysql
SHOW DATABASES;
USE <nom_de_la_base_de_données>;
SHOW TABLES;
SELECT * FROM <nom_de_la_table>;
``` 

## Supprimer une foreign key

```bash
ALTER TABLE servers DROP FOREIGN KEY domaine;
SHOW CREATE TABLE servers;
```

## Ajouter une table

```bash
ALTER TABLE domains ADD timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
```

## Supprimer une clé (contrainte d'unicité)

```bash
ALTER TABLE `ice_crawler_DB`.`servers` DROP INDEX `tls.starttls_UNIQUE`;
```
