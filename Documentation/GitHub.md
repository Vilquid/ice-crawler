# Github

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec **GitHub**

## Installer Github Desktop sur Ubuntu

```bash
sudo wget https://github.com/shiftkey/desktop/releases/download/release-2.8.1-linux2/GitHubDesktop-linux-2.8.1-linux2.deb
sudo dpkg -i GitHubDesktop-linux-2.8.1-linux2.deb
```

## Erreurs
si erreur :
```bash
 git push --set-upstream origin
 ```
executer la commande :
```bash
git config --global push.default current
```