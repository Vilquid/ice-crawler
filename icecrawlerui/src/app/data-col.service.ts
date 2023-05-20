import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { CacheService } from './cache.service';
import { map, concat, Observable} from 'rxjs';

import { DNSRecord } from "./shared-interfaces/dns-record"
import { DATAResult } from './shared-interfaces/data-result';
import { DMARCRecord } from './shared-interfaces/dmarc-record';
import { SPFRecord } from './shared-interfaces/spf-record';
import { DANERecord } from './shared-interfaces/dane-record';
import { BIMIRecord } from './shared-interfaces/bimi-record';
import { MTARecord } from './shared-interfaces/mta-record';
import { TLSRecord } from './shared-interfaces/tls-rpt-record';
import { CertificateRecord } from './shared-interfaces/certificate-record';
import { IssuerDetails } from './shared-interfaces/issuer-details';
import { SendToDomain } from './shared-interfaces/send-to-domain';
import { SendToCIDR } from './shared-interfaces/send-to-cidr';




@Injectable({
  providedIn: 'root'
})
export class DataColService {

  constructor(private httpClient: HttpClient, private cache: CacheService) {  }

  launchScanByCIDR(CIDR: SendToCIDR): Observable<DATAResult[]>{            // Post
      const url = "http://10.10.20.122/database"
      return this.httpClient.post<DATAResult>(url,CIDR).pipe(
      map((data: any) => {
        console.log(data)
        let dataresult: DATAResult = {
          tls: data.tls,
        };

        let dnsR: DNSRecord = {
          domain: data.dns.domain,
        }

          let dmarcR: DMARCRecord = {
            v: data.dns.dmarc.v,
            p: data.dns.dmarc.p,
            sp: data.dns.dmarc.sp,
            pct: data.dns.dmarc.pct,
            ruf: data.dns.dmarc.ruf,
            rua: data.dns.dmarc.rua,
            ri: data.dns.dmarc.ri,
            rf: data.dns.dmarc.rf,
            aspf: data.dns.dmarc.aspf,
            adkim: data.dns.dmarc.adkim,
            fo: data.dns.dmarc.fo,
          }
          dnsR.dmarc = dmarcR

          let spfR: SPFRecord = {
            domain: data.dns.spf.domain,
            version: data.dns.spf.version,
            mechanisms: [],
            qualifier: data.dns.spf.qualifier,
            ip: [],
            include: [],
            all: data.dns.spf.all
          }
            const mechanismsL: string[]=[];
            for (let i in data.dns.spf.mechanisms){
              mechanismsL.push(i)
            }
            spfR.mechanisms = mechanismsL

            const ipL: string[]=[];
            for (let i in data.dns.spf.ip){
              ipL.push(i)
            }
            spfR.ip = ipL

            const includeL: string[]=[];
            for (let i in data.dns.spf.include){
              includeL.push(i)
            }
            spfR.include = includeL
          dnsR.spf = spfR

          let daneR: DANERecord = {
            forme_certificat: data.dns.dane.forme_certificat,
            signature_certificat: data.dns.dane.signature_certificat,
            signature_cle_publique: data.dns.dane.signature_cle_publique,
            presence_hash: data.dns.dane.presence_hash,
            hash: data.dns.dane.hash
          }
          dnsR.dane = daneR

          let bimiR: BIMIRecord = {
            version: data.dns.bimi.version,
            url_expediteur: data.dns.bimi.url_expediteur,
            url_politique: data.dns.bimi.url_politique,
            url_reputation: data.dns.bimi.url_reputation,
            hash: data.dns.bimi.hash,
            s: data.dns.bimi.s
          }
          dnsR.bimi = bimiR

          let mtaR: MTARecord = {
            version: data.dns.mta.version,
            sn: data.dns.mta.sn
          }
          dnsR.mta =  mtaR

          let tlsR: TLSRecord = {
            v: data.dns.tls.v,
            rua: data.dns.tls.rua
          }
          dnsR.tls = tlsR

          let certificateR: CertificateRecord = {
            domain: data.dns.certificate.domain,
            signature_algorithm_server: data.dns.certificate.signature_algorithm_server,
            issuer_server: {
              city: data.dns.certificate.issuer_server.city,
              state: data.dns.certificate.issuer_server.state,
              locality: data.dns.certificate.issuer_server.locality,
              organization: data.dns.certificate.issuer_server.organization,
              common_name: data.dns.certificate.issuer_server.common_name
            },
            validity_server: {
              not_before: data.dns.certificate.validity_server.not_before,
              not_after: data.dns.certificate.validity_server.not_after,
              is_valid: data.dns.certificate.validity_server.is_valid
            },
            subject_server: {
              city: data.dns.certificate.subject_server.city,
              state: data.dns.certificate.subject_server.state,
              locality: data.dns.certificate.subject_server.locality,
              organization: data.dns.certificate.subject_server.organization,
              common_name: data.dns.certificate.subject_server.common_name
            },
            extensions_server: {
              subject_alternative_names: data.dns.certificate.extensions_server.subject_alternative_names
            },
            signature_algorithm_intermediate: data.dns.certificate.signature_algorithm_intermediate,
            issuer_intermediate: {
              city: data.dns.certificate.issuer_intermediate.city,
              state: data.dns.certificate.issuer_intermediate.state,
              locality: data.dns.certificate.issuer_intermediate.locality,
              organization: data.dns.certificate.issuer_intermediate.organization,
              common_name: data.dns.certificate.issuer_intermediate.common_name
            },
            validity_intermediate: {
              not_before: data.dns.certificate.validity_intermediate.not_before,
              not_after: data.dns.certificate.validity_intermediate.not_after,
              is_valid: data.dns.certificate.validity_intermediate.is_valid,
            },
            subject_intermediate: {
              city: data.dns.certificate.subject_intermediate.city,
              state: data.dns.certificate.subject_intermediate.state,
              locality: data.dns.certificate.subject_intermediate.locality,
              organization: data.dns.certificate.subject_intermediate.organization,
              common_name: data.dns.certificate.subject_intermediate.common_name
            },
            extensions_intermediate: {
              subject_alternative_names: data.dns.certificate.extensions_server.subject_alternative_names
            }
          }
          dnsR.certificate = certificateR

        dataresult.dns = dnsR;

        return dataresult;
      })
    )
  }

  launchScanByDomain(domainName: SendToDomain): Observable<any>{  // Post
    const url = "http://10.10.20.122/database"
      return this.httpClient.post<DATAResult>(url,domainName).pipe(
      map((data: any) => {
        console.log(data)
        let dataresult: DATAResult = {
          tls: data.tls,
        };

        let dnsR: DNSRecord = {
          domain: data.dns.domain,
        }

          let dmarcR: DMARCRecord = {
            v: data.dns.dmarc.v,
            p: data.dns.dmarc.p,
            sp: data.dns.dmarc.sp,
            pct: data.dns.dmarc.pct,
            ruf: data.dns.dmarc.ruf,
            rua: data.dns.dmarc.rua,
            ri: data.dns.dmarc.ri,
            rf: data.dns.dmarc.rf,
            aspf: data.dns.dmarc.aspf,
            adkim: data.dns.dmarc.adkim,
            fo: data.dns.dmarc.fo,
          }
          dnsR.dmarc = dmarcR

          let spfR: SPFRecord = {
            domain: data.dns.spf.domain,
            version: data.dns.spf.version,
            mechanisms: [],
            qualifier: data.dns.spf.qualifier,
            ip: [],
            include: [],
            all: data.dns.spf.all
          }
            const mechanismsL: string[]=[];
            for (let i in data.dns.spf.mechanisms){
              mechanismsL.push(i)
            }
            spfR.mechanisms = mechanismsL

            const ipL: string[]=[];
            for (let i in data.dns.spf.ip){
              ipL.push(i)
            }
            spfR.ip = ipL

            const includeL: string[]=[];
            for (let i in data.dns.spf.include){
              includeL.push(i)
            }
            spfR.include = includeL
          dnsR.spf = spfR

          let daneR: DANERecord = {
            forme_certificat: data.dns.dane.forme_certificat,
            signature_certificat: data.dns.dane.signature_certificat,
            signature_cle_publique: data.dns.dane.signature_cle_publique,
            presence_hash: data.dns.dane.presence_hash,
            hash: data.dns.dane.hash
          }
          dnsR.dane = daneR

          let bimiR: BIMIRecord = {
            version: data.dns.bimi.version,
            url_expediteur: data.dns.bimi.url_expediteur,
            url_politique: data.dns.bimi.url_politique,
            url_reputation: data.dns.bimi.url_reputation,
            hash: data.dns.bimi.hash,
            s: data.dns.bimi.s
          }
          dnsR.bimi = bimiR

          let mtaR: MTARecord = {
            version: data.dns.mta.version,
            sn: data.dns.mta.sn
          }
          dnsR.mta =  mtaR

          let tlsR: TLSRecord = {
            v: data.dns.tls.v,
            rua: data.dns.tls.rua
          }
          dnsR.tls = tlsR

          let certificateR: CertificateRecord = {
            domain: data.dns.certificate.domain,
            signature_algorithm_server: data.dns.certificate.signature_algorithm_server,
            issuer_server: {
              city: data.dns.certificate.issuer_server.city,
              state: data.dns.certificate.issuer_server.state,
              locality: data.dns.certificate.issuer_server.locality,
              organization: data.dns.certificate.issuer_server.organization,
              common_name: data.dns.certificate.issuer_server.common_name
            },
            validity_server: {
              not_before: data.dns.certificate.validity_server.not_before,
              not_after: data.dns.certificate.validity_server.not_after,
              is_valid: data.dns.certificate.validity_server.is_valid
            },
            subject_server: {
              city: data.dns.certificate.subject_server.city,
              state: data.dns.certificate.subject_server.state,
              locality: data.dns.certificate.subject_server.locality,
              organization: data.dns.certificate.subject_server.organization,
              common_name: data.dns.certificate.subject_server.common_name
            },
            extensions_server: {
              subject_alternative_names: data.dns.certificate.extensions_server.subject_alternative_names
            },
            signature_algorithm_intermediate: data.dns.certificate.signature_algorithm_intermediate,
            issuer_intermediate: {
              city: data.dns.certificate.issuer_intermediate.city,
              state: data.dns.certificate.issuer_intermediate.state,
              locality: data.dns.certificate.issuer_intermediate.locality,
              organization: data.dns.certificate.issuer_intermediate.organization,
              common_name: data.dns.certificate.issuer_intermediate.common_name
            },
            validity_intermediate: {
              not_before: data.dns.certificate.validity_intermediate.not_before,
              not_after: data.dns.certificate.validity_intermediate.not_after,
              is_valid: data.dns.certificate.validity_intermediate.is_valid,
            },
            subject_intermediate: {
              city: data.dns.certificate.subject_intermediate.city,
              state: data.dns.certificate.subject_intermediate.state,
              locality: data.dns.certificate.subject_intermediate.locality,
              organization: data.dns.certificate.subject_intermediate.organization,
              common_name: data.dns.certificate.subject_intermediate.common_name
            },
            extensions_intermediate: {
              subject_alternative_names: data.dns.certificate.extensions_server.subject_alternative_names
            }
          }
          dnsR.certificate = certificateR

        dataresult.dns = dnsR;

        return dataresult;
      })
    )
  }

  getData(): Observable<DATAResult[]> {                           // Get
    const url = 'http://10.10.20.121'
    return this.httpClient.get<DATAResult[]>(url).pipe(
      map((data: any[]) => {
        const AllData: DATAResult[] = []

        for (let k=0; k<data.length; k++){

        let dataresult: DATAResult = {
          tls: data[k].tls,
        };

        let dnsR: DNSRecord = {
          domain: data[k].dns.domain,
        }

          let dmarcR: DMARCRecord = {
            v: data[k].dns.dmarc.v,
            p: data[k].dns.dmarc.p,
            sp: data[k].dns.dmarc.sp,
            pct: data[k].dns.dmarc.pct,
            ruf: data[k].dns.dmarc.ruf,
            rua: data[k].dns.dmarc.rua,
            ri: data[k].dns.dmarc.ri,
            rf: data[k].dns.dmarc.rf,
            aspf: data[k].dns.dmarc.aspf,
            adkim: data[k].dns.dmarc.adkim,
            fo: data[k].dns.dmarc.fo,
          }
          dnsR.dmarc = dmarcR

          let spfR: SPFRecord = {
            domain: data[k].dns.spf.domain,
            version: data[k].dns.spf.version,
            mechanisms: [],
            qualifier: data[k].dns.spf.qualifier,
            ip: [],
            include: [],
            all: data[k].dns.spf.all
          }
            const mechanismsL: string[]=[];
            for (let i in data[k].dns.spf.mechanisms){
              mechanismsL.push(i)
            }
            spfR.mechanisms = mechanismsL

            const ipL: string[]=[];
            for (let i in data[k].dns.spf.ip){
              ipL.push(i)
            }
            spfR.ip = ipL

            const includeL: string[]=[];
            for (let i in data[k].dns.spf.include){
              includeL.push(i)
            }
            spfR.include = includeL
          dnsR.spf = spfR

          let daneR: DANERecord = {
            forme_certificat: data[k].dns.dane.forme_certificat,
            signature_certificat: data[k].dns.dane.signature_certificat,
            signature_cle_publique: data[k].dns.dane.signature_cle_publique,
            presence_hash: data[k].dns.dane.presence_hash,
            hash: data[k].dns.dane.hash
          }
          dnsR.dane = daneR

          let bimiR: BIMIRecord = {
            version: data[k].dns.bimi.version,
            url_expediteur: data[k].dns.bimi.url_expediteur,
            url_politique: data[k].dns.bimi.url_politique,
            url_reputation: data[k].dns.bimi.url_reputation,
            hash: data[k].dns.bimi.hash,
            s: data[k].dns.bimi.s
          }
          dnsR.bimi = bimiR

          let mtaR: MTARecord = {
            version: data[k].dns.mta.version,
            sn: data[k].dns.mta.sn
          }
          dnsR.mta =  mtaR

          let tlsR: TLSRecord = {
            v: data[k].dns.tls.v,
            rua: data[k].dns.tls.rua
          }
          dnsR.tls = tlsR

          let certificateR: CertificateRecord = {
            domain: data[k].dns.certificate.domain,
            signature_algorithm_server: data[k].dns.certificate.signature_algorithm_server,
            issuer_server: {
              city: data[k].dns.certificate.issuer_server.city,
              state: data[k].dns.certificate.issuer_server.state,
              locality: data[k].dns.certificate.issuer_server.locality,
              organization: data[k].dns.certificate.issuer_server.organization,
              common_name: data[k].dns.certificate.issuer_server.common_name
            },
            validity_server: {
              not_before: data[k].dns.certificate.validity_server.not_before,
              not_after: data[k].dns.certificate.validity_server.not_after,
              is_valid: data[k].dns.certificate.validity_server.is_valid
            },
            subject_server: {
              city: data[k].dns.certificate.subject_server.city,
              state: data[k].dns.certificate.subject_server.state,
              locality: data[k].dns.certificate.subject_server.locality,
              organization: data[k].dns.certificate.subject_server.organization,
              common_name: data[k].dns.certificate.subject_server.common_name
            },
            extensions_server: {
              subject_alternative_names: data[k].dns.certificate.extensions_server.subject_alternative_names
            },
            signature_algorithm_intermediate: data[k].dns.certificate.signature_algorithm_intermediate,
            issuer_intermediate: {
              city: data[k].dns.certificate.issuer_intermediate.city,
              state: data[k].dns.certificate.issuer_intermediate.state,
              locality: data[k].dns.certificate.issuer_intermediate.locality,
              organization: data[k].dns.certificate.issuer_intermediate.organization,
              common_name: data[k].dns.certificate.issuer_intermediate.common_name
            },
            validity_intermediate: {
              not_before: data[k].dns.certificate.validity_intermediate.not_before,
              not_after: data[k].dns.certificate.validity_intermediate.not_after,
              is_valid: data[k].dns.certificate.validity_intermediate.is_valid,
            },
            subject_intermediate: {
              city: data[k].dns.certificate.subject_intermediate.city,
              state: data[k].dns.certificate.subject_intermediate.state,
              locality: data[k].dns.certificate.subject_intermediate.locality,
              organization: data[k].dns.certificate.subject_intermediate.organization,
              common_name: data[k].dns.certificate.subject_intermediate.common_name
            },
            extensions_intermediate: {
              subject_alternative_names: data[k].dns.certificate.extensions_server.subject_alternative_names
            }
          }
          dnsR.certificate = certificateR

        dataresult.dns = dnsR;

        AllData.push(dataresult)
        }
        return AllData;
      })
    )
  }

  getBackupData(domainName: SendToDomain): Observable<DATAResult> {          // Backup prog, bound to disappear
    const url = 'http://10.10.20.122/data'
    return this.httpClient.post<DATAResult>(url,domainName).pipe(
      map((data: any) => {
        console.log(data)
        let dataresult: DATAResult = {
          tls: data.tls,
        };

        let dnsR: DNSRecord = {
          domain: data.dns.domain,
        }

          let dmarcR: DMARCRecord = {
            v: data.dns.dmarc.v,
            p: data.dns.dmarc.p,
            sp: data.dns.dmarc.sp,
            pct: data.dns.dmarc.pct,
            ruf: data.dns.dmarc.ruf,
            rua: data.dns.dmarc.rua,
            ri: data.dns.dmarc.ri,
            rf: data.dns.dmarc.rf,
            aspf: data.dns.dmarc.aspf,
            adkim: data.dns.dmarc.adkim,
            fo: data.dns.dmarc.fo,
          }
          dnsR.dmarc = dmarcR

          let spfR: SPFRecord = {
            domain: data.dns.spf.domain,
            version: data.dns.spf.version,
            mechanisms: [],
            qualifier: data.dns.spf.qualifier,
            ip: [],
            include: [],
            all: data.dns.spf.all
          }
            const mechanismsL: string[]=[];
            for (let i in data.dns.spf.mechanisms){
              mechanismsL.push(i)
            }
            spfR.mechanisms = mechanismsL

            const ipL: string[]=[];
            for (let i in data.dns.spf.ip){
              ipL.push(i)
            }
            spfR.ip = ipL

            const includeL: string[]=[];
            for (let i in data.dns.spf.include){
              includeL.push(i)
            }
            spfR.include = includeL
          dnsR.spf = spfR

          let daneR: DANERecord = {
            forme_certificat: data.dns.dane.forme_certificat,
            signature_certificat: data.dns.dane.signature_certificat,
            signature_cle_publique: data.dns.dane.signature_cle_publique,
            presence_hash: data.dns.dane.presence_hash,
            hash: data.dns.dane.hash
          }
          dnsR.dane = daneR

          let bimiR: BIMIRecord = {
            version: data.dns.bimi.version,
            url_expediteur: data.dns.bimi.url_expediteur,
            url_politique: data.dns.bimi.url_politique,
            url_reputation: data.dns.bimi.url_reputation,
            hash: data.dns.bimi.hash,
            s: data.dns.bimi.s
          }
          dnsR.bimi = bimiR

          let mtaR: MTARecord = {
            version: data.dns.mta.version,
            sn: data.dns.mta.sn
          }
          dnsR.mta =  mtaR

          let tlsR: TLSRecord = {
            v: data.dns.tls.v,
            rua: data.dns.tls.rua
          }
          dnsR.tls = tlsR

          let certificateR: CertificateRecord = {
            domain: data.dns.certificate.domain,
            signature_algorithm_server: data.dns.certificate.signature_algorithm_server,
            issuer_server: {
              city: data.dns.certificate.issuer_server.city,
              state: data.dns.certificate.issuer_server.state,
              locality: data.dns.certificate.issuer_server.locality,
              organization: data.dns.certificate.issuer_server.organization,
              common_name: data.dns.certificate.issuer_server.common_name
            },
            validity_server: {
              not_before: data.dns.certificate.validity_server.not_before,
              not_after: data.dns.certificate.validity_server.not_after,
              is_valid: data.dns.certificate.validity_server.is_valid
            },
            subject_server: {
              city: data.dns.certificate.subject_server.city,
              state: data.dns.certificate.subject_server.state,
              locality: data.dns.certificate.subject_server.locality,
              organization: data.dns.certificate.subject_server.organization,
              common_name: data.dns.certificate.subject_server.common_name
            },
            extensions_server: {
              subject_alternative_names: data.dns.certificate.extensions_server.subject_alternative_names
            },
            signature_algorithm_intermediate: data.dns.certificate.signature_algorithm_intermediate,
            issuer_intermediate: {
              city: data.dns.certificate.issuer_intermediate.city,
              state: data.dns.certificate.issuer_intermediate.state,
              locality: data.dns.certificate.issuer_intermediate.locality,
              organization: data.dns.certificate.issuer_intermediate.organization,
              common_name: data.dns.certificate.issuer_intermediate.common_name
            },
            validity_intermediate: {
              not_before: data.dns.certificate.validity_intermediate.not_before,
              not_after: data.dns.certificate.validity_intermediate.not_after,
              is_valid: data.dns.certificate.validity_intermediate.is_valid,
            },
            subject_intermediate: {
              city: data.dns.certificate.subject_intermediate.city,
              state: data.dns.certificate.subject_intermediate.state,
              locality: data.dns.certificate.subject_intermediate.locality,
              organization: data.dns.certificate.subject_intermediate.organization,
              common_name: data.dns.certificate.subject_intermediate.common_name
            },
            extensions_intermediate: {
              subject_alternative_names: data.dns.certificate.extensions_server.subject_alternative_names
            }
          }
          dnsR.certificate = certificateR

        dataresult.dns = dnsR;

        return dataresult;
      })
    )
  }
}

