use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::time::Duration;
use serde::Serialize;
use crate::cidr::cidr_notation;
use crate::ipinfo::ip_info;


/// # Brief
/// Structure associant un nom de domaine à une adresse IP
/// # Fields
/// - domain *String*
/// - ip *String*
#[derive(Debug, Serialize)]
pub struct Output
{
	pub domain: String,
	pub ip: String,
}


/// # Brief
/// Fonction qui dit si le port est ouvert sur la machine demandée
/// # Arguments
/// - adresse *SocketAddr* : adresse IP et port à tester
/// # Return
/// Retourne un *bool*
/// # Exemple
/// ```
/// let mut ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 25);
/// println!("{}", scan_port_addr(ip));
/// ```
/// # Warning
/// Une tentative de connexion est faite sur le port 25 de l'adresse IP donnée en paramètre pendant 200 ms. C'est un temps suffisant pour que 99% des serveurs SMTP répondent. Un temps plus long améliorerait la fiabilité mais ralentirait le programme.
fn scan_port_addr(addr: SocketAddr) -> bool
{
	match TcpStream::connect_timeout(&addr, Duration::from_millis(200))
	{
		Ok(_) => true,
		Err(_) => false,
	}
}

/// # Brief
/// Fonction qui incrémente les octets d'une adresse IP d'un tableau de byte de type *u8*
/// # Arguments
/// - bytes *&mut [u8]* : ip sous forme de tableau de bytes à incrémenter
/// # Return
/// Rien (car c'est un tableau qui est modifié
/// # Exemple
/// ```
/// let mut bytes: [u8; 4] = ip.octets();
/// increment_ip_bytes(&mut bytes);
/// ```
fn increment_ip_bytes(bytes: &mut [u8])
{
	for byte in bytes.iter_mut().rev()
	{
		match byte.checked_add(1)
		{
			None => *byte = 0,
			Some(new_byte) => {
				*byte = new_byte;
				return;
			}
		}
	}
}

/// # Brief
/// Fonction qui renvoie l'adresse IP d'après par rapport à l'adresse IP donnée en paramètre
/// # Arguments
/// - ip *IpAddr* : adresse IP à incrémenter
/// # Return
/// Renvoie l'ip suivante *IpAddr*
/// # Exemple
/// ```
/// ip.set_ip(increment_ip(ip.ip()).expect("Échec de l'incrémentation de l'IP"));
/// ```
/// # Warning
/// Il existe une variante non-implémentée de cette fonction
/// ```
/// let mut local_v4 = Ipv4Addr::new(127, 0, 0, 1);
/// while local_v4 != Ipv4Addr::new(127, 0, 0, 255)
/// {
/// 	let local_u32: u32 = local_v4.into();
/// 	let local_plus1 = Ipv4Addr::from(local_u32 + 1);
/// 	println!("{} + 1 = {}", local_v4, local_plus1);
/// 	local_v4 = local_plus1;
/// }
/// ```
pub fn increment_ip(ip: IpAddr) -> Option<IpAddr>
{
	match ip
	{
		IpAddr::V4(ip) => {
			let mut bytes: [u8; 4] = ip.octets();
			increment_ip_bytes(&mut bytes);

			if bytes.iter().fold(true, |acc, byte| acc && *byte == 0)
			{
				return None;
			}

			Some(Ipv4Addr::from(bytes).into())
		}
		IpAddr::V6(ip) => {
			let mut bytes: [u8; 16] = ip.octets();
			increment_ip_bytes(&mut bytes);
			if bytes.iter().fold(true, |acc, byte| acc && *byte == 0)
			{
				return None;
			}
			Some(Ipv6Addr::from(bytes).into())
		}
	}
}

/// # Brief
/// Fonction qui liste les adresses IP des serveurs qui ont leur port 25 ouvert
/// # Arguments
/// - cidr *&str* : plage d'adresses IP à scanner
/// # Return
/// Liste les adresses IP des serveurs qui ont leur port 25 ouvert de type *Vec<Output>*
/// # Exemple
/// ```
/// let liste = lister_serveurs_port_25("142.250.147.0/26");
/// ```
/// # Warning
/// Cette fonction est très longue à s'exécuter :
/// - Temps pour scanner 142.250.147.0/20 : 849 s
/// - Temps pour scanner 142.250.147.0/22 : 207 s
/// - Temps pour scanner 142.250.147.0/24 : 52 s
/// - Temps pour scanner 142.250.147.0/26 : 13 s
/// - Temps pour scanner 142.250.147.0/28 : 3 s
pub fn lister_serveurs_port_25(cidr: &str) -> Vec<Output>
{
	let mut ip_range = cidr_notation(cidr);

	ip_range.debut.push_str(":25");
	ip_range.fin.push_str(":25");

	// Traduction des entrées utilisateur en SocketAddr pour pouvoir les traiter
	let mut ip_debut_add: SocketAddr = String::from(&ip_range.debut).parse().expect("Échec de la transformation de ip_debut (String) à ip_debut_add (SocketAddr)");
	let ip_fin_add: SocketAddr = String::from(&ip_range.fin).parse().expect("Échec de la transformation de ip_fin (String) à ip_fin_add (SocketAddr)");

	// Création d'une liste d'adresse IP dont le port 25 est ouvert
	let mut list_domain_ip: Vec<Output> = Vec::new();

	// Scan de toutes les adresses IP comprises entre ip_debut_add et ip_fin_add
	while ip_debut_add.ip() != ip_fin_add.ip()
	{
		// Si le port 25 est ouvert, on l'ajoute à la liste
		if scan_port_addr(ip_debut_add)
		{
			// Ajout de l'adresse IP à la liste
			list_domain_ip.push(Output {
				domain: ip_info(ip_debut_add.ip().to_string().as_str()).unwrap(),
				ip: ip_debut_add.ip().to_string(),
			});
		}

		// Incrémentation de l'adresse IP
		ip_debut_add.set_ip(increment_ip(ip_debut_add.ip()).expect("Échec de l'incrémentation de l'IP"));
	}

	return list_domain_ip;
}


/// # Brief
/// TU de *lister_serveurs_port_25.rs*
#[cfg(test)]
mod tests_lister_serveurs_port_25
{
	use std::assert_eq;
	use std::net::{IpAddr, Ipv4Addr};
	use super::*;

	/// # Brief
	/// TU de *scan_port_addr()*
	/// # Arguments
	/// Aucun
	#[test]
	fn test_scan_port_addr()
	{
		assert!(scan_port_addr("142.250.147.27:25".parse().unwrap()));
		assert_eq!(scan_port_addr("142.250.147.26:25".parse().unwrap()), false);
	}

	/// # Brief
	/// TU de *increment_ip_bytes()*
	/// # Arguments
	/// Aucun
	#[test]
	fn test_increment_ip_bytes()
	{
		let mut bytes: [u8; 4] = [0, 0, 0, 0];
		increment_ip_bytes(&mut bytes);
		assert_eq!(bytes, [0, 0, 0, 1]);
	}

	/// # Brief
	/// TU de *increment_ip()*
	/// # Arguments
	/// Aucun
	#[test]
	fn test_increment_ip()
	{
		assert_eq!(increment_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))), Some(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 1))));
		assert_eq!(increment_ip(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255))), None);
	}
}
