use crate::dns::{dns, DNSRecord};
use crate::tls::*;
use serde::{Deserialize, Serialize, Serializer};
use crate::tls::tls;


#[derive(Debug, Deserialize)]
pub struct Input
{
	pub domain: String,
	pub ip: String,
}

#[derive(Debug, Serialize)]
pub struct DATAResult
{
	pub dns: DNSRecord,
	pub tls: Retour,
}

pub(crate) fn data(structure: Input) -> DATAResult
{
	let dns = dns(&structure.domain);

	let tls = tls(String::from(structure.ip), String::from(structure.domain.clone()));

	let data = DATAResult
	{
		dns,
		tls,
	};

	return data;
}
