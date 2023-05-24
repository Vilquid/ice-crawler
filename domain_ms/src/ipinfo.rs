use ipinfo::{IpInfo, IpInfoConfig};


/// # Brief
/// Fonction qui récupère le nom de domaine associé à l'adresse IP passée en paramètre.
/// # Arguments
/// - ip *&str* : adresse IP à analyser
/// # Return
/// Retourne le nom de domaine de type *String* associé à l'adresse IP.
pub(crate) fn ip_info(ip: &str) -> String
{
 	let config = IpInfoConfig { token: Some("aa3b89b7853f6c".to_string()), ..Default::default() };

	IpInfo::new(config).expect("should construct").lookup(&[ip]).unwrap().get(ip).unwrap().hostname.clone().unwrap().to_string()
}
