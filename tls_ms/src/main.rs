use crate::tls::tls;

mod tls;

fn main()
{

	let tls = tls(String::from("142.250.147.27"));

	println!("TLS : {:#?}", tls);
}
