use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use crate::ipinfo::ip_info;
use regex::Regex;
use crate::Output;


/// # Brief
/// Fonction qui recherche les adresses IP des serveurs mail d'un domaine.
/// # Arguments
/// - domain_name *&str* : le nom de domaine à analyser
/// # Return
/// *Vec<String>* : un tableau contenant les adresses IP des serveurs mail
/// # Example
/// ```
/// use dns_ms::mx_records::mx_records;
/// mod mx_records;
///
/// fn main()
/// {
/// 	mx_records("google.com.");
/// }
/// ```
pub(crate) async fn mx_records(ip_domain: &str) -> Vec<Output>
{
	#[allow(unused_assignments)]
		let mut domain = String::new();

	// Si l'input match avec l'expression régulière, c'est une adresse IP
	if Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap().is_match(ip_domain) == true
	{
		domain = ip_info(ip_domain).await;
	}

	// Sinon, c'est un nom de domaine
	else
	{
		domain = ip_domain.to_string();
	}

	// list_domain_ip : tableau de structure Output
	let mut list_domain_ip: Vec<Output> = Vec::new();

	let mut ip: Vec<String> = Vec::new();

	// Création d'un résolveur qui effectuera le travail de recherche DNS
	let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

	// Appel de mx_lookup() et stockage du résultat dans mx_response
	// Le point final force cette recherche à être un FQDN.
	let mx_response = resolver.mx_lookup(domain);

	// Traitement des cas si la recherche DNS a réussi ou non
	match mx_response
	{
		Err(_) => (),

		Ok(mx_response) => {
			let records = mx_response.iter();

			for record in records
			{
				let lookup_response = resolver.lookup_ip(record.exchange().to_string().as_str());

				match lookup_response
				{
					// Ne rien faire si c'est vide
					Err(_) => (),

					// Si la réponse n'est pas vide, on récupère les adresses IP liées au domaine
					Ok(lookup_response) => {
						let ip_addrs = lookup_response.iter();

						for ip_addr in ip_addrs
						{
							if ip_addr.is_ipv4()
							{
								// ip.push(ip_addr.to_string());
								list_domain_ip.push(Output {
									domain: ip_info(ip_addr.to_string().as_str()).await,
									ip: ip_addr.to_string(),
								});
							}
						}
					}
				}
			}
		}
	}

	list_domain_ip
}
