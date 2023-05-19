import { Injectable } from '@angular/core';
import { BIMIRecord } from './shared-interfaces/bimi-record'
import { DANERecord } from './shared-interfaces/dane-record';
import { DMARCRecord } from './shared-interfaces/dmarc-record';
import { MTARecord } from './shared-interfaces/mta-record';
import { SPFRecord } from './shared-interfaces/spf-record';
import { TLSRecord } from './shared-interfaces/tls-rpt-record';
import { CertificateRecord } from './shared-interfaces/certificate-record';
import { DATAResult } from './shared-interfaces/data-result';

@Injectable({
  providedIn: 'root'
})
export class NotationService {

  constructor() { }

}



enum BIMIFields {
  version = 4,
  url_expediteur = 1.2,
  url_politique = 1.2,
  url_reputation = 1.2,
  hash = 1.2,
  s = 1.2
}

enum DANEFields {
  forme_certificat = 2,
  signature_certificat = 2,
  signature_cle_publique = 2,
  presence_hash = 2,
  hash = 2
}

enum DMARCFields {
  v = 2.75,
  p = 2.75,
  sp = 0.5,
  pct = 0.5,
  ruf = 0.5,
  rua = 0.5,
  ri = 0.5,
  rf = 0.5,
  aspf = 0.5,
  adkim = 0.5,
  fo = 0.5
}

enum MTASTSFields {
  v = 5,
  id = 5
}


enum SPFFields {
  version = 1.5,
  mechanisms = 1.5,
  qualifier = 1.5,
  ip = 1.5,
  include = 1.5,
  all = 1.5
}

enum TLSRPTFields {
  v = 5,
  rua = 5
}

enum CertificateFields {
  signature_algorithm_server = 1,
  issuer_server_organization = 1,
  issuer_server_organizational_unit = 1,
  validity_server_is_valid = 1,
  subject_server_common_name = 1,
  extensions_server_subject_alternative_name = 1,
  signature_algorithm_intermediate = 1,
  issuer_intermediate = 0.25,
  validity_intermediate_is_valid = 1,
  subject_intermediate_common_name = 1
}

function getBimiScore(bimi: BIMIRecord): number {
  let score = 0;

    if (bimi.version != "vide") {
      score += BIMIFields.version;
    }

    if(bimi.url_expediteur != "vide") {
      score += BIMIFields.url_expediteur;
    }

    if(bimi.url_politique != "vide") {
      score += BIMIFields.url_politique;
    }

    if(bimi.url_reputation != "vide") {
      score += BIMIFields.url_reputation;
    }

    if(bimi.hash != "vide") {
      score += BIMIFields.hash;
    }

    if(bimi.s != "vide") {
      score += BIMIFields.s;
    }


  console.log(`Note BIMI : ${score}/10`);
  return score;
}

function getDaneScore(dane: DANERecord): number {
  let score = 0;

    if (dane.forme_certificat != "vide") {
      score += DANEFields.forme_certificat;
    }

    if(dane.signature_certificat == true) {
      score += DANEFields.signature_certificat;
    }

    if(dane.signature_cle_publique == true) {
      score += DANEFields.signature_cle_publique;
    }

    if(dane.presence_hash == true) {
      score += DANEFields.presence_hash;
    }

    if(dane.hash != "vide") {
      score += DANEFields.hash;
    }

  console.log(`Note DANE : ${score}/10`);
  return score;
}


function getDmarcScore(dmarc: DMARCRecord): number {
  let score = 0;

    if (dmarc.v != "vide") {
      score += DMARCFields.v;
    }

    if(dmarc.p != "vide") {
      score += DMARCFields.p;
    }

    if(dmarc.sp != "vide") {
      score += DMARCFields.sp;
    }

    if(dmarc.pct != "vide") {
      score += DMARCFields.pct;
    }

    if(dmarc.ruf != "vide") {
      score += DMARCFields.ruf;
    }

    if(dmarc.rua != "vide") {
      score += DMARCFields.rua;
    }

    if(dmarc.ri != "vide") {
      score += DMARCFields.ri;
    }

    if(dmarc.rf != "vide") {
      score += DMARCFields.rf;
    }

    if(dmarc.aspf != "vide") {
      score += DMARCFields.aspf;
    }

    if(dmarc.adkim != "vide") {
      score += DMARCFields.adkim;
    }

    if(dmarc.fo != "vide") {
      score += DMARCFields.fo;
    }

  console.log(`Note DMARC : ${score}/10`);
  return score;
}

function getMTAScore(mta: MTARecord): number {
  let score = 0;

    if (mta.version != "vide") {
      score += MTASTSFields.v;
    }

    if(mta.sn != "vide") {
      score += MTASTSFields.id;
    }

    console.log(`Note MTA-STS : ${score}/10`);
    return score;
  }

function getSPFScore(spf: SPFRecord): number {
  let score = 0;

    if (spf.version != "vide") {
      score += SPFFields.version;
    }

    if(spf.mechanisms.length > 0) {
      score += SPFFields.mechanisms;
    }

    if(spf.qualifier != "vide") {
      score += SPFFields.qualifier;
    }

    if(spf.ip.length > 0) {
      score += SPFFields.ip;
    }

    if(spf.include.length > 0) {
      score += SPFFields.include;
    }

    if(spf.all != "vide") {
      score += SPFFields.all;
    }


  console.log(`Note SPF : ${score}/10`);
  return score;
}


function getTLSScore(tls: TLSRecord): number {
  let score = 0;

    if (tls.v != "vide") {
      score += TLSRPTFields.v;
    }

    if(tls.rua != "vide") {
      score += TLSRPTFields.rua;
    }

    console.log(`Note TLS-RPT : ${score}/10`);
    return score;
}


function getCertificatScore(certificat: CertificateRecord): number {
  let score = 0;


    if (certificat.signature_algorithm_server != "vide") {
      score += CertificateFields.signature_algorithm_server;
    }

    if(certificat.issuer_server?.organization != "vide") {
      score += CertificateFields.issuer_server_organization;
    }


    if(certificat.issuer_server?.organization != "vide") {
      score += CertificateFields.issuer_server_organizational_unit;
    }

    if(certificat.validity_server?.is_valid == true) {
      score += CertificateFields.validity_server_is_valid;
    }

    if(certificat.subject_server?.common_name != "vide") {
      score += CertificateFields.subject_server_common_name;
    }

    if(certificat.extensions_server?.subject_alternative_names) {
      score += CertificateFields.extensions_server_subject_alternative_name;
    }

    if(certificat.signature_algorithm_intermediate != "vide") {
      score += CertificateFields.signature_algorithm_intermediate;
    }

    if(certificat.issuer_intermediate?.organization != "vide") {
      score += CertificateFields.issuer_intermediate;
    }

    if(certificat.validity_intermediate?.is_valid == true) {
      score += CertificateFields.validity_intermediate_is_valid;
    }

    if(certificat.subject_intermediate?.common_name != "vide") {
      score += CertificateFields.subject_intermediate_common_name;
    }

  console.log(`Note Certificat : ${score}/10`);
  return score;
}



interface Results {
  bimiScore?: number,
  daneScore?: number,
  dmarcScore?: number,
  mtaScore?: number,
  spfScore?: number,
  tlsScore?: number,
  certificateScore?: number
}

function getEverything(data: DATAResult): Results {
  return {
    bimiScore: data.dns && data.dns.bimi ? getBimiScore(data.dns.bimi) : undefined,
    daneScore: data.dns && data.dns.dane ? getDaneScore(data.dns.dane) : undefined,
    dmarcScore: data.dns && data.dns.dmarc ? getDmarcScore(data.dns.dmarc) : undefined,
    mtaScore: data.dns && data.dns.mta ? getMTAScore(data.dns.mta) : undefined,
    spfScore: data.dns && data.dns.spf ? getSPFScore(data.dns.spf) : undefined,
    tlsScore: data.dns && data.dns.tls ? getTLSScore(data.dns.tls) : undefined,
    certificateScore: data.dns && data.dns.certificate ? getCertificatScore(data.dns.certificate) : undefined
  };
}

//test
/*function getEverything(data: DATAResult): Results {

  return {

    bimiScore: getBimiScore(data.dns?.bimi),
    daneScore: getDaneScore(data.dns?.dane),
    dmarcScore: getDmarcScore(data.dns?.dmarc),
    mtaScore: getMTAScore(data.dns?.mta),
    spfScore: getSPFScore(data.dns?.spf)
  };

}*/


/*

function getTLSVersionScore(version: string): number {
  switch (version) {
    case "1.3":
      return 10;
    case "1.2":
      return 7;
    case "1.1":
      return 4;
    case "1.0":
      return 1;
    default:
      return 0;
  }

  //console.log(`La note est de ${score}/10`);
} */
