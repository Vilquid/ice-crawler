import { DNSRecord } from "./dns-record";
import { Retour } from "./retour";

export interface DATAResult
{
	dns?: DNSRecord,
	tls?: Retour,
}
