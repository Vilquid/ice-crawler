# Micro Services

> Le but de ce document est d'y inscrire toutes les commandes & infos en rapport avec les **Micro-Services** et/ou leur **Fonctionnement**

## Distribution des Ports

| **Micro Service**  | **Port** |
|--------------------|----------|
| _DOMAIN_           | 9000     |
| _IP-RANGE_         | 9001     |
| _Data_             | 9002     |
| _WEBUI_            | 9007     |
| _Authentification_ | 9008     |
| _database_         | 9009     |
| _BDD_              | 9010     |

# Notation

## Domaine

Notation totale : ((dane + dmarc + mta_sts + tls_rpt) / 3 + (bimi + certificate + spf) / 3)) / 6 * 10

### **BIMI (bimi)**

Note sur 10 :

* Présence du champ "version" : +4
* Présence du champ "url_expediteur" : +1,2
* Présence du champ "url_politique" : +1,2
* Présence du champ "url_reputation" : +1,2
* Présence du champ "hash" : +1,2
* Présence du champ "s" : +1,2

### **Certificate (certificate)**

Note sur 10 :

* Présence du champ "signature_algorithm_server" : +1
* Présence du champ "issuer_server.organization" : +1
* Présence du champ "issuer_server.common_name" : +1
* Présence du champ "validity_server.is_valid" : +1
* Présence du champ "subject_server.common_name" : +1
* Présence du champ "extensions_server.subject_alternative_name" : +1
* Présence du champ "signature_algorithm_intermediate" : +1
* Pour chaque présence d'un champ de "issuer_intermediate" : +0,25
* Si "validity_intermediate.is_valid" = true : +1
* Présence du champ "subject_intermediate.common_name" : +1

### **DANE (dane)**

Note sur 10 :

* Présence du champ "forme_certificat" : +2
* Présence du champ "signature_certificat" : +2
* Présence du champ "signature_cle_publique" : +2
* Si "presence_hash" = true : +2
* Présence du champ "hash" : +2

### **DMARC (dmarc)**

Note sur 10 :

* Présence du champ "v" : +2,75
* Présence du champ "p" : +2,75
* Présence du champ "sp" : +0,5
* Présence du champ "pct" : +0,5
* Présence du champ "ruf" : +0,5
* Présence du champ "rua" : +0,5
* Présence du champ "ri" : +0,5
* Présence du champ "rf" : +0,5
* Présence du champ "aspf" : +0,5
* Présence du champ "adkim" : +0,5
* Présence du champ "fo" : +0,5

### **MTA-STS (mta_sts)**

Note sur 10 :

* Présence du champ "v" : +5
* Présence du champ "id" : +5

### **SPF (spf)**

Note sur 10 :

* Présence du champ "version" : +1,5
* Présence du champ "mechanisms" : +1,5
* Présence du champ "qualifier" : +1,5
* Présence du champ "ip" : +1,5
* Présence du champ "include" : +1,5
* Présence du champ "all" : +1,5

### **TLS-RPT (tls_rpt)**

Note sur 10 :

* Présence du champ "v" : +5
* Présence du champ "rua" : +5

## IP (serveur)

Note : (cipher + tls) / 2

### **Cipher suite (cipher)**

Si la cipher suite appartient au 35 meilleures cipher suites, alors la note est de 10/10.
Si la cipher suite appartient au 70 meilleures cipher suites, alors la note est de 9/10.
Si la cipher suite appartient au 105 meilleures cipher suites, alors la note est de 8/10.
etc ...

### **TLS (tls)**

Si la version 1.3 de TLS est supportée, alors la note est de 10/10.
Si la version 1.2 de TLS est supportée, alors la note est de 7/10.
Si la version 1.1 de TLS est supportée, alors la note est de 4/10.
Si la version 1.0 de TLS est supportée, alors la note est de 1/10.
Sinon, la note est de 0/10.
