SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema ice_crawler_DB
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `ice_crawler_DB` ;
USE `ice_crawler_DB` ;

-- -----------------------------------------------------
-- Table `ice_crawler_DB`.`domains`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ice_crawler_DB`.`domains` (
  `id` INT NOT NULL AUTO_INCREMENT COMMENT 'id du domaine',
  `domain` TINYTEXT NOT NULL COMMENT 'Nom du domaine',
  `note` FLOAT NOT NULL COMMENT 'Note du domaine',
  `bimi.version` TINYTEXT NOT NULL COMMENT 'Version de BIMI',
  `bimi.url_expediteur` MEDIUMTEXT NOT NULL,
  `bimi.url_politique` MEDIUMTEXT NOT NULL,
  `bimi.url_reputation` MEDIUMTEXT NOT NULL,
  `bimi.hash` MEDIUMTEXT NOT NULL,
  `bimi.s` MEDIUMTEXT NOT NULL,
  `certificate.signature_algorithm_server` MEDIUMTEXT NOT NULL,
  `certificate.IssuerDetails.city` TINYTEXT NOT NULL,
  `certificate.IssuerDetails.state` TINYTEXT NOT NULL,
  `certificate.IssuerDetails.locality` TINYTEXT NOT NULL,
  `certificate.IssuerDetails.organization` TINYTEXT NOT NULL,
  `certificate.IssuerDetails.common_name` TINYTEXT NOT NULL,
  `certificate.ValidityDetails.not_before` TINYTEXT NOT NULL,
  `certificate.ValidityDetails.not_after` TINYTEXT NOT NULL,
  `certificate.ValidityDetails.is_valid` TINYTEXT NOT NULL,
  `certificate.SubjectDetails.city` TINYTEXT NOT NULL,
  `certificate.SubjectDetails.state` TINYTEXT NOT NULL,
  `certificate.SubjectDetails.locality` TINYTEXT NOT NULL,
  `certificate.SubjectDetails.organization` TINYTEXT NOT NULL,
  `certificate.SubjectDetails.common_name` TINYTEXT NOT NULL,
  `certificate.ExtensionsDetails.subject_alternative_names` MEDIUMTEXT NOT NULL COMMENT 'Noms alternatifs du sujet',
  `certificate.signature_algorithm_intermediate` TINYTEXT NOT NULL,
  `certificate.issuer_intermediate.city` TINYTEXT NOT NULL,
  `certificate.issuer_intermediate.state` TINYTEXT NOT NULL,
  `certificate.issuer_intermediate.locality` TINYTEXT NOT NULL,
  `certificate.issuer_intermediate.organization` TINYTEXT NOT NULL,
  `certificate.issuer_intermediate.common_name` TINYTEXT NOT NULL,
  `certificate.validity_intermediate.not_before` TINYTEXT NOT NULL,
  `certificate.validity_intermediate.not_after` TINYTEXT NOT NULL,
  `certificate.validity_intermediate.is_valid` TINYTEXT NOT NULL,
  `certificate.subject_intermediate.city` TINYTEXT NOT NULL,
  `certificate.subject_intermediate.state` TINYTEXT NOT NULL,
  `certificate.subject_intermediate.locality` TINYTEXT NOT NULL,
  `certificate.subject_intermediate.organization` TINYTEXT NOT NULL,
  `certificate.subject_intermediate.common_name` TINYTEXT NOT NULL,
  `certificate.extensions_intermediate.subject_alternative_names` MEDIUMTEXT NOT NULL,
  `dane.forme_certificat` TINYTEXT NOT NULL,
  `dane.signature_certificat` TINYTEXT NOT NULL,
  `dane.signature_cle_publique` TINYTEXT NOT NULL,
  `dane.presence_hash` TINYTEXT NOT NULL,
  `dane.hash` TINYTEXT NOT NULL,
  `dmarc.v` TINYTEXT NOT NULL,
  `dmarc.p` TINYTEXT NOT NULL,
  `dmarc.sp` TINYTEXT NOT NULL,
  `dmarc.pct` TINYTEXT NOT NULL,
  `dmarc.ruf` TINYTEXT NOT NULL,
  `dmarc.rua` TINYTEXT NOT NULL,
  `dmarc.ri` TINYTEXT NOT NULL,
  `dmarc.rf` TINYTEXT NOT NULL,
  `dmarc.aspf` TINYTEXT NOT NULL,
  `dmarc.adkim` TINYTEXT NOT NULL,
  `dmarc.fo` TINYTEXT NOT NULL,
  `mta_sts.version` TINYTEXT NOT NULL,
  `mta_sts.sn` TINYTEXT NOT NULL,
  `spf.version` TINYTEXT NOT NULL,
  `spf.mechanisms` MEDIUMTEXT NOT NULL,
  `spf.qualifier` MEDIUMTEXT NOT NULL,
  `spf.ip` TINYTEXT NOT NULL,
  `spf.include` MEDIUMTEXT NOT NULL,
  `spf.all` MEDIUMTEXT NOT NULL,
  `tls_rpt.v` TINYTEXT NOT NULL,
  `tls_rpt.rua` TINYTEXT NOT NULL,
  `bimi.note` FLOAT NOT NULL,
  `certificate.note` FLOAT NOT NULL,
  `dane.note` FLOAT NOT NULL,
  `dmarc.note` FLOAT NOT NULL,
  `mta_sts.note` FLOAT NOT NULL,
  `spf.note` FLOAT NOT NULL,
  `tls_rpt.note` FLOAT NOT NULL,
  `timestamp` DATETIME NOT NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `server`
    FOREIGN KEY (`id`)
    REFERENCES `ice_crawler_DB`.`servers` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
AUTO_INCREMENT = 1;

CREATE UNIQUE INDEX `id_UNIQUE` ON `ice_crawler_DB`.`domains` (`id` ASC);

-- -----------------------------------------------------
-- Table `ice_crawler_DB`.`servers`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ice_crawler_DB`.`servers` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `ip` VARCHAR(45) NOT NULL,
  `domaine` TINYTEXT NOT NULL,
  `tls.certificat` VARCHAR(45) NOT NULL,
  `tls.liste` VARCHAR(30000) NOT NULL,
  `tls.cyfaible` VARCHAR(45) NOT NULL,
  `tls.starttls` VARCHAR(45) NOT NULL,
  `tls.note` FLOAT NOT NULL,
  `timestamp` DATETIME NOT NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `domaine`
    FOREIGN KEY (`domaine`)
    REFERENCES `ice_crawler_DB`.`domains` (`domain`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
AUTO_INCREMENT = 1;

CREATE INDEX `id_UNIQUE` ON `ice_crawler_DB`.`servers` (`id` ASC);
CREATE INDEX `domaine_idx` ON `ice_crawler_DB`.`servers` (`domaine` ASC);
CREATE INDEX `ip_UNIQUE` ON `ice_crawler_DB`.`servers` (`ip` ASC);
CREATE INDEX `domaine_UNIQUE` ON `ice_crawler_DB`.`servers` (`domaine` ASC);
CREATE INDEX `tls.certificat_UNIQUE` ON `ice_crawler_DB`.`servers` (`tls.certificat` ASC);
CREATE INDEX `tls.cyfaible_UNIQUE` ON `ice_crawler_DB`.`servers` (`tls.cyfaible` ASC);
CREATE INDEX `tls.starttls_UNIQUE` ON `ice_crawler_DB`.`servers` (`tls.starttls` ASC);
CREATE INDEX `tls.note_UNIQUE` ON `ice_crawler_DB`.`servers` (`tls.note` ASC);

-- -----------------------------------------------------
-- Table `ice_crawler_DB`.`users`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ice_crawler_DB`.`users` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `mail` VARCHAR(60) NOT NULL COMMENT 'Adresse mail de l\'utilisateur',
  `hash` VARCHAR(60) NOT NULL COMMENT 'Hash de l\'utilisateur',
  `sel` VARCHAR(45) NOT NULL COMMENT 'Sel associé au mot de passe',
  PRIMARY KEY (`id`))
ENGINE = InnoDB
AUTO_INCREMENT = 1;

CREATE UNIQUE INDEX `id_UNIQUE` ON `ice_crawler_DB`.`users` (`id` ASC);

CREATE USER 'ice_crawler_user' IDENTIFIED BY 'fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG';

GRANT ALL ON `ice_crawler_DB`.* TO 'ice_crawler_user';
GRANT SELECT ON TABLE `ice_crawler_DB`.* TO 'ice_crawler_user';
GRANT SELECT, INSERT, TRIGGER ON TABLE `ice_crawler_DB`.* TO 'ice_crawler_user';
GRANT SELECT, INSERT, TRIGGER, UPDATE, DELETE ON TABLE `ice_crawler_DB`.* TO 'ice_crawler_user';

SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
