use actix_web::{get, post, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, App, HttpServer, HttpRequest};
use serde::{Serialize, Deserialize};
use reqwest;
use actix_cors::Cors;
use serde_json::json;
use crate::lister_serveurs_port_25::lister_serveurs_port_25;



mod lister_serveurs_port_25;
mod ipinfo;
mod cidr;




//mod mx_records;
//mod ipinfo;




#[derive(Debug)]
pub struct Output
{
	domain: String,
	ip: String,
}





// pense l'adresse de l'api c'est 0.0.0.0 et le port c'est 9000
#[derive(Deserialize)]
pub struct Debut {
	cidr: String,
}






#[post("/")]
async fn pourpost(req: Json<Debut>) -> HttpResponse {
	println!("je suis vivant");
	let mut contenu=req.cidr.to_string();
	if contenu.eq("") {
		return HttpResponse::Ok().body("erreur j'ai rien reçu");
	}
	let list_domain = lister_serveurs_port_25(contenu.as_str());
	println!("paré à envoyer {:?}", list_domain);
	for i in list_domain.iter() {
		//envoi vers l'api des services d'extraction de données
		println!("envoie de {:?}",i);
		let mut data=String::new();
		data = "{\"domaine\": \"".to_owned() + i.domain.as_str() + "\"ip\": \"".to_owned().clone().as_str() + i.ip.as_str() + "\"}";
		println!("data= {:?}",data);
		let chaine = serde_json::to_string(&data).unwrap();
		println!("chaine= {:?}",chaine);
		let resp = reqwest::Client::new()
			.post("0.0.0.0:9002")
			.header("Content-Type", "application/json")
			.body(chaine)
			.send()
			.await;
		println!("réponse={:?}",resp);
	}




	return HttpResponse::Ok().body("ok");
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new( move || { let cors = Cors::default().allow_any_origin().send_wildcard();
		App::new()
			.wrap(cors)
			.service(pourpost)
	})
		.bind(("0.0.0.0", 9001)).expect("REASON")
		.run()
		.await

}







// use actix_web::{App, HttpServer, web};
// use crate::lister_serveurs_port_25::lister_serveurs_port_25;
//
// mod lister_serveurs_port_25;
// mod ipinfo;
// mod cidr;
//
//
// /// # Brief
// /// Fonction qui formate la liste des serveurs qui ont leur port 25 ouvert pour l'API
// /// # Arguments
// /// - data *web::Data<AppStateWithCounter.>*
// /// # Return
// /// Les données de type *String* à envoyer à l'API
// async fn index() -> String
// {
// 	let liste = lister_serveurs_port_25("142.250.147.0/27");
//
// 	format!("{:#?}", serde_json::to_string(&liste).unwrap())
// }
//
// /// # Brief
// /// Fonction API qui renvoie la liste des serveurs qui ont leur port 25 ouvert sur le port 8080 (127.0.0.1 - http://localhost:8080)
// /// # Arguments
// /// Pas d'argument
// /// # Return
// /// Etat d'exécution de la fonction de type *std::io::Result<()>*
// #[actix_web::main]
// async fn main() -> std::io::Result<()>
// {
// 	// Indication que la recherche est terminée
// 	println!("Va sur http://localhost:9001");
//
// 	// Création d'une nouvelle instance HttpServer pour envoyer les données
// 	HttpServer::new(move || {
// 		App::new()
// 			.route("/", web::get().to(index))
// 	})
// 		.bind(("0.0.0.0", 9001))?
// 		.run()
// 		.await
// }


// /// # Brief
// /// TU de *main.rs*
// #[cfg(test)]
// mod tests_main
// {
// 	use std::error::Error;
// 	use actix_web::{http, test};
// 	use actix_web::body::to_bytes;
// 	use super::*;
//
//
// 	/// # Brief
// 	/// TU de *main.rs* qui vérifie que la fonction *index()* renvoie bien la liste des serveurs qui ont leur port 25 ouvert
// 	#[actix_web::test]
// 	async fn test_index() -> Result<(), Error>
// 	{
// 		let app = App::new().route("/", web::get().to(index));
// 		let app = test::init_service(app).await;
//
// 		let req = test::TestRequest::get().uri("/").to_request();
// 		let resp = app.call(req).await?;
//
// 		assert_eq!(resp.status(), http::StatusCode::OK);
//
// 		let response_body = resp.into_body();
// 		assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);
//
// 		Ok(())
// 	}
// }

