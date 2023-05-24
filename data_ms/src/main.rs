use actix_cors::Cors;
use actix_web::{get, post, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, App, HttpServer, HttpRequest};
use serde::{Serialize, Deserialize};
use serde_json::{json, to_string};
use crate::data::{data, Input};
use reqwest::*;



mod dns;
mod dane;
mod dmarc;
mod ipinfo;
mod bimi;
mod spf;
mod tls_rpt;
mod mta_sts;
mod certificate;
mod data;
mod tls;




#[post("/")]
async fn pourpost(req: Json<Input>) -> HttpResponse {
	println!("1");
	let mut contenu= Input {domain: req.domain.to_string(), ip: req.ip.to_string() };
	if contenu.domain.eq("") {
		println!("rien reçu");
		return HttpResponse::Ok().body("rien... t'es sûr ?");
	}
	println!("2");
	let mut retour = data(contenu);
	println!("3");
	println!("serialisation en cours...");
	let chaine = serde_json::to_string(&retour).unwrap();
	println!("4");
	println!("c'est serialisé. {}", chaine);
	let url = format!("http://database.default");

	
	let resp = reqwest::Client::new()
		.post(&url)
		.header("Content-Type", "application/json")
		.body(chaine.clone())
		.send()
		.await;
	println!("{:?}",resp);
	println!("{:?}",resp);
	return HttpResponse::Ok().body("ok");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

	HttpServer::new( move || {
		let cors = Cors::default().allow_any_origin().send_wildcard();
		App::new()
			.wrap(cors)
			.service(pourpost)
	})
		.bind(("0.0.0.0", 9002)).expect("REASON")
		.run()
		.await

}
