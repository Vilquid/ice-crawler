# Docker

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec **Docker**

## Installer Docker

```bash
sudo apt update
sudo apt install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
apt-cache policy docker-ce
sudo apt install docker-ce
sudo systemctl status docker
sudo usermod -aG docker ${USER}
su - ${USER}
groups
```

## Créer Images

```bash
docker build -t <nom_MS> .
```

## Lancer Conteneur

```bash
docker run --rm -p <port_MS>:<port_MS> <nom_MS>
```

* Pour le Micro Service SMTP (temporairement)

```bash
docker run --rm -p 9007:9007 -it smtp bash
```
