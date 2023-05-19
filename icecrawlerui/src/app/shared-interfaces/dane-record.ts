export interface DANERecord
{
	forme_certificat: string,
	signature_certificat: boolean,
	signature_cle_publique: boolean,
	presence_hash: boolean,
	hash: string
}
