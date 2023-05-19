import { DMARCRecord } from "./dmarc-record"
import { SPFRecord } from "./spf-record"
import { DANERecord } from "./dane-record"
import { BIMIRecord } from "./bimi-record"
import { MTARecord } from "./mta-record"
import { TLSRecord } from "./tls-rpt-record"
import { CertificateRecord } from "./certificate-record"


export interface DNSRecord {
	domain: string,
	dmarc?: DMARCRecord,
	spf?: SPFRecord,
	dane?: DANERecord,
	bimi?: BIMIRecord,
	mta?: MTARecord,
	tls?: TLSRecord,
	certificate?: CertificateRecord
}

