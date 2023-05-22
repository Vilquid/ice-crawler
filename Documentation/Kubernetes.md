# Kubernetes

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec **Kubernetes**

## Installation de K3S sur Ubuntu

### **Pas ce qu'on a fait**

```bash
curl -sfL https://get.k3s.io | sh -
mkdir -p $HOME/.kube
sudo cp /etc/rancher/k3s/k3s.* $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
```

### **Ce qu'on a fait**

#### Installer k3s sur la master node

```bash
curl -sfL https://get.k3s.io | sh -s - server --disable servicelb --cluster-init
```

> **--cluster-init** est utilisé pour la Haute-Disponibilité (HA) et est seulement utilisé sur la première node server(master) k3s

Puis on récupère:

1. IP de la master node
2. Token de la master node généré pendant l'installation (peut être lu sur la master node avec ```sudo cat /var/lib/rancher/k3s/server/node-token```)
3. le fichier kubeconfig situé à /etc/rancher/k3s/k3s.yaml sur la master node

#### Pour HA : Ajouter les autres master nodes

> Inspiré de ce [Tuto](https://docs.k3s.io/installation/ha-embedded)

> Dans le cas présent, il faut un nombre total impair de master(server) node.

> Il ne faut pas oublier de mettre les mêmes arguments utilisés pour le premier master, pour chaque master (à l'exception de **--cluster-init**)

Sur chaque master additionnel:

```bash
curl -sfL https://get.k3s.io | K3S_TOKEN=<Master_Node_Token> sh -s - server --disable servicelb --server https://<Master_Node_IP>:6443
```

#### Installer k3s sur les worker nodes

Sur chaque worker node:

```bash
curl -sfL https://get.k3s.io | K3S_URL=https://<Master_Node_IP>:6443 K3S_TOKEN=<Master_Node_Token> sh -
```

## Désinstalation de k3s

### Worker nodes

Sur la worker node à désinstaller :

```bash
/usr/local/bin/k3s-agent-uninstall.sh
```

Puis, sur la machine enliaison avec le cluster :

```bash
kubectl delete node <Node_Name>
```

### Master nodes

Sur la master node à désinstaller :

```bash
/usr/local/bin/k3s-uninstall.sh
```

## Installer Helm

[Lien](https://helm.sh/fr/docs/intro/install/)

## Supprimer un pod

```bash
kubectl delete pod <nom_du_pod>
```

## Kong Gateway

### Applying changes to Gateway endpoints

```bash
curl -F "config=@Kong-endpoints-icecrawler.yaml" http://10.10.20.122:8001/config
```
