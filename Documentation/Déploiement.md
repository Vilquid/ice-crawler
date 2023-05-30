# Fichier de déploiement

## Utilité du document

> Ce fichier contient toutes les informations utiles au déploiement du projet.

## Prérequis

- 4 serveurs
- 1 NAS (Synology)

Si le repository est cloné sur un serveur, il est d'usage de modifier les faux mots de passe insérés dans les scripts :
- database.sql
- mysql-secret.yaml
- postgres-configmap.yaml
- ipinfo.rs (token pour cette [API](https://ipinfo.io/))
- database_ms/main.rs (l. 303/377/499/603)

## Déploiement

### Proxmox

#### Installation

Télécharger l'ISO de Proxmox VE sur le site officiel : https://www.proxmox.com/en/downloads/category/iso-images-pve

Graver l'ISO sur une clé USB avec Rufus : https://rufus.ie/

Installer Proxmox VE sur le serveur : https://youtu.be/PptYrSgthQc

#### Configuration

##### Changer le nom du serveur

```bash
nano /etc/hosts
nano /etc/hostname
reboot
```

##### Changer la gateway du serveur

```bash
ip route change default via 10.10.20.254 proto kernel
reboot
```

À faire sur chaque VM qui sera sur le cluster :

##### Passer sur une IP fixe

```bash
sudo vim /etc/netplan/00-installer-config.yaml
```

Mettre le contenu suivant dedans en changeant la valeur de l'addresse :

```yaml
network:
  ethernets:
    ens18:
      dhcp4: false
      addresses: [10.10.20.<id_vm>/24]
      nameservers:
        addresses: [8.8.8.8, 8.8.4.4]
      routes:
      - to: default
        via: 10.10.20.254
  version: 2
```

Appliquer les changements :

```bash
sudo netplan apply
```

À faire sur chaque VM qui sera sur le cluster :

##### Changer le nom d'hôte

A 2 endroits :

```bash
sudo hostnamectl set-hostname <new_hostname>
sudo vim /etc/hosts
```

Ca devrait ressembler à ça, ne changer que ce qui est nécessaire.

```text
127.0.0.1 localhost
127.0.1.1 <new_hostname> 

# The following lines are desirable for IPv6 capable hosts
::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters
```

### Cluster Kubernetes

#### Installation de K3S sur Ubuntu

##### Installer k3s sur la master node

```bash
curl -sfL https://get.k3s.io | sh -s - server --disable servicelb --cluster-init
```

> `--cluster-init` est utilisé pour la haute disponibilité (HA) et est seulement utilisé sur la première node server(master) k3s

Puis on récupère :

1. IP de la master node
2. Token de la master node généré pendant l'installation (peut être lu sur la master node avec ```sudo cat /var/lib/rancher/k3s/server/node-token```)
3. Le fichier kubeconfig situé à /etc/rancher/k3s/k3s.yaml sur la master node

##### Pour HA : Ajouter les autres master nodes

> Inspiré de ce [Tuto](https://docs.k3s.io/installation/ha-embedded)

> Dans le cas présent, il faut un nombre total impair de master (server) node.

> Il ne faut pas oublier de mettre les mêmes arguments utilisés pour le premier master, pour chaque master (à l'exception de `--cluster-init`)

Sur chaque master additionnel :

```bash
curl -sfL https://get.k3s.io | K3S_TOKEN=<Master_Node_Token> sh -s - server --disable servicelb --server https://<Master_Node_IP>:6443
```

##### Installer k3s sur les worker nodes

Sur chaque worker node:

```bash
curl -sfL https://get.k3s.io | K3S_URL=https://<Master_Node_IP>:6443 K3S_TOKEN=<Master_Node_Token> sh -
```

##### Installer Helm

[Lien](https://helm.sh/fr/docs/intro/install/)

### Services

#### kubectl

kubectl doit être et installé sur la machine qui va déployer les services et configuré pour utiliser le fichier kubeconfig de la master node.

Pour chacun des fichiers contenus dans le dossier `kubernetes` (à l'exception des fichiers metalLB metal-L2Advertisement.yaml, metal-IPAddressPool.yaml et kong-endpoints-icecrawler), il faut faire la commande suivante :

```bash
kubectl apply -f <file_name>
```

Une fois cela fait, installer metalLB avec Helm :

```bash
helm repo add metallb https://metallb.github.io/metallb
helm install metallb metallb/metallb
```

Une fois cela fait, applique les fichiers de configuration de metalLB (metal-IPAddressPool.yaml et metal-L2Advertisement.yaml) :

```bash
kubectl apply -f <file_name>
``` 

Changer la configuration de Kong :

```bash
curl -F "config=@Kong-endpoints-icecrawler.yaml" http://10.10.20.122:8001/config
```

#### NFS

##### Connexion au NAS (ssh)

```bash
ssh <utilisateur>@<ip_nas>
```

##### Utiliser NFS pout mettre en place BDD

Pour toutes les nodes il faut installer les outils nfs :

```bash
 echo <MDP_VM> | sudo -S apt install -y nfs-common
```

Changer le propriétaire à 999 :

```bash
 echo <MDP_VM> | sudo -S chown -R 999:999 /volume1/data &   
```

Créer le dossier data :

```bash
 echo <MDP_VM> | sudo -S mkdir -p /volume1/data/
```

Dans /etc/fstab (sur toutes les VM) :

```bash
sudo vim /etc/fstab
```

```text
<ip_nas>:/volume1/data   /volume1/data   nfs      auto,_netdev,nofail 0 0
```

Monter le volume :

```bash
echo <MDP_VM> | sudo -S mount -t nfs <ip_nas>:/volume1/data /volume1/data 
```

Changer les permissions à 777 pour /volume1/data :

```bash
echo <MDP_VM> | sudo -S chmod 777 /volume1/data/
```

### Workflows

Enlever la partie "push" sur docker ou changer la destination du push. Pour cela, il faut changer les valeurs contenues dans les secrets de GitHub (`DOCKER_USERNAME` et `DOCKER_ACCESS_TOKEN`). De plus, il faut changer dans chaque fichier de déploiement K8S le nom du repository d'où pull l'image.


## Utilisation du produit

Lancement de scan sur un ensemble de domaines :

```bash
curl -X POST -H "Content-Type: application/json" -d '{"domaine": ["laposte.net","gmail.com"]}' http://<ip_gateway>/domain
```

Lancement de scan sur une CIDR:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"cidr": "142.251.9.14/27"}' http://<ip_gateway>/smtp
```

Pour accéder à la web UI, il faut taper l'adresse de la gateway dans un navigateur web. Ensuite, il faut aller dans "Recherche" et y taper les informations que l'on souhaite.
