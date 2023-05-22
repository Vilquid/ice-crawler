# NAS

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec le **NAS**, le **NFS** & la **BDD**

## Connexion au NAS (ssh)

```bash
ssh cybernas@10.10.2.1
```

## Utiliser NFS pout mettre en place BDD

>Pour faire une opération sur toutes les nodes utiliser le script

Pour toutes les nodes il faut installer les outils nfs

```bash
 echo <MDP_VM> | sudo -S apt install -y nfs-common
```

change owner to 999

```bash
 echo <MDP_VM> | sudo -S chown -R 999:999 /volume1/data &   
```

create the dir:

```bash
 echo <MDP_VM> | sudo -S mkdir -p /volume1/data/
```

dans /etc/fstab: (à la main sur toutes les VM)

```bash
 sudo vim /etc/fstab
```

```bash
 10.10.2.1:/volume1/data   /volume1/data   nfs      auto,_netdev,nofail 0 0
```

Then mount the volume:

```bash
 echo <MDP_VM> | sudo -S mount -t nfs 10.10.2.1:/volume1/data /volume1/data 
```

Then set permissions to 777 for /volume1/data

```bash
 echo <MDP_VM> | sudo -S chmod 777 /volume1/data/
```

tuto ici: [(Déprécié)](https://itnext.io/kubernetes-storage-part-1-nfs-complete-tutorial-75e6ac2a1f77)
