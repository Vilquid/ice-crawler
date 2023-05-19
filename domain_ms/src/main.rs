use crate::mx_records::mx_records;
use actix_web::{get, post, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, App, HttpServer, HttpRequest};
use serde::{Serialize, Deserialize};
use reqwest;
use serde_json::json;


mod mx_records;
mod ipinfo;


#[derive(Serialize)]
pub struct Output
{
    domain: String,
    ip: String,
}

#[derive(Deserialize)]
pub struct Debut {
	domaine: String,
}



#[post("/")]
async fn service(req: Json<Debut>) -> HttpResponse {
	let demande = req.domaine.to_string();
	if demande.eq("") {
		return HttpResponse::Ok().body("je n'ai rien reçu!");
	}
	let mut reponse = mx_records(&demande);
	for i in reponse {
		let mut payload = serde_json::to_string(&i).unwrap();
		let resp = reqwest::Client::new()
		    .post("0.0.0.0:9002")
		    .header("Content-Type", "application/json")
		    .body(payload.clone())
		    .send()
		    .await;
	}
	return HttpResponse::Ok().body("OK");
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {

	HttpServer::new( move || {
		//let cors = Cors::default().allow_any_origin().send_wildcard();
		App::new()
			//.wrap(cors)
			.service(service)
	})
		.bind(("0.0.0.0", 9000)).expect("REASON")
		.run()
		.await

}
