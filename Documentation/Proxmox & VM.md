# Proxmox

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec **Proxmox** et les **VM**

## Changer le nom du serveur

```bash
nano /etc/hosts
nano /etc/hostname
reboot
```

## Changer la gateway du serveur

```bash
ip route change default via 10.10.20.254 proto kernel
reboot
```

## Activer les mises à jour

```bash
nano /etc/apt/sources.list.d/pve-enterprise.list
# Commenter la ligne : deb https://enterprise.proxmox.com/debian/pve bullseye pve-enterprise
# Enregistrer et quitter le fichier
apt update && apt full-upgrade -y && apt autoremove -y
reboot
```

Scripts aidant à l'installation de Proxmox : [Scripts](https://tteck.github.io/Proxmox/)

# Configuration des VM

## Exécuter une commande sur toutes les VM (workers)

```bash
#!/bin/bash

# Vérification de l'installation de sshpass
dpkg -s sshpass &> /dev/null

if [ $? -ne 0 ]
then
 # Installation de sshpass
 echo "sshpass n'est pas installé."
 read -p "Entrer le mot de passe de la machine hôte : " password_host
 echo "Installation de sshpass ..."
 echo $password_host | sudo -S apt install sshpass
 echo "sshpass installé."
fi

read -p "Entrer le mot de passe des VM : " password_vm
read -p "Entrer la commande à exécuter sur les VM : " commande

# Première VM : 10.10.20.102
start=102
# Dernière VM : 10.10.20.116
end=116

# Exécution de la commande sur toutes les VM
for i in $(seq $start $end)
do
 echo ""
 echo "##### Exécution sur 10.10.20.${i} #####"
 echo ""
 sshpass -p ${password_vm} ssh -o StrictHostKeyChecking=no vm@10.10.20.$i "${commande}"
done
```

## Set ip to a static-ip

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

* Appliquer les changements:

```bash
sudo netplan apply
```

## Change Hostname

* A 2 endroits :

```bash
sudo hostnamectl set-hostname <new_hostname>
```

* ET

```bash
sudo vim /etc/hosts
```

* ça devrait ressembler à ça, ne changer que ce qui est nécéssaire

```vim
127.0.0.1 localhost
127.0.1.1 <new_hostname> 

# The following lines are desirable for IPv6 capable hosts
::1     ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters
```
