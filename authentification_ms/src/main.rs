#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use jsonwebtoken::{encode, Header, EncodingKey};
use actix_cors::Cors;
use actix_web::{HttpServer, App, web, HttpResponse, Responder, http::header, post};
use actix_web::web::Data;
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager};
use serde::{Deserialize, Serialize};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use dotenv::dotenv;

use models::{User, NewUser, LoginUser};
use crate::models::dechiffrement;


#[derive(Debug)]
enum ServerError {
    ArgonauticError,
    DieselError,
    EnvironmentError,
    R2D2Error,
    UserError(String),
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[post("/register")]
async fn process_signup(data: web::Json<NewUser>) -> impl Responder {
    use schema::users;

    let mut connection = establish_connection();

    let new_user = NewUser::new(data.username.clone(), data.email.clone(), data.password.clone());
    println!("{}", new_user.username);
    println!("{}", new_user.password);


    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut connection)
        .expect("Error registering used.");

    println!("{:?}", data);
    HttpResponse::Ok()//.json(format!("Successfully saved user: {}", data.username))
}

#[derive(Debug, Serialize, Deserialize)]
struct UsernameEmailCheck {
    username: String,
    email: String,
    username_available: bool,
    email_available: bool,
}

async fn check_username_email(data: web::Json<UsernameEmailCheck>, pool: Data<Pool>) -> Result<HttpResponse, ServerError> {
    use schema::users::dsl::*;

    let username_to_check = &data.username;
    let email_to_check = &data.email;

    let mut connection = pool.get().map_err(|_| ServerError::DieselError)?;

    let user = users
        .filter(username.eq(username_to_check))
        .first::<User>(&mut connection)
        .optional()
        .map_err(|_| ServerError::DieselError)?;

    let is_username_available = user.is_none();

    let user = users
        .filter(email.eq(email_to_check))
        .first::<User>(&mut connection)
        .optional()
        .map_err(|_| ServerError::DieselError)?;

    let is_email_available = user.is_none();

    let response = UsernameEmailCheck {
        username: username_to_check.clone(),
        email: email_to_check.clone(),
        username_available: is_username_available,
        email_available: is_email_available,
    };

    Ok(HttpResponse::Ok().json(response))
}

// la structure Claims contient les informations qu'on veut stocker dans le token JWT
// on l'ID de l'utilisateur et la date d'expiration du token
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// La fonction create_jwt_token prend l'ID de l'utilisateur en argument et crée un token JWT avec ces informations
fn create_jwt_token(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let secret = "your-secret-key";
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}


#[post("/login")]
async fn process_login(
    data: web::Json<LoginUser>,
    id: Identity,
    pool: Data<Pool>,
) -> Result<HttpResponse, ServerError> {
    use schema::users::dsl::{username, users};

    let mut connection = pool.get()?;
    let user = users.filter(username.eq(&data.username)).first::<User>(&mut connection)?;

    let valid = dechiffrement(data.password.clone(), user.password);

    if valid {
        let other_username = user.username.clone();
        let session_token = String::from(other_username.as_str());

        id.remember(session_token);

        // On crées le token JWT en appelant la fonction create_jwt_token
        let jwt_token = create_jwt_token(&other_username);

        let token = jwt_token.clone();

        Ok(HttpResponse::Ok()
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .insert_header(("Access-Control-Expose-Headers", "Authorization"))
            .finish())
    } else {
        Ok(HttpResponse::BadRequest().json("Password is incorrect."))
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test")
    }
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::ArgonauticError => HttpResponse::InternalServerError().json("Argonautica Error."),
            ServerError::DieselError => HttpResponse::InternalServerError().json("Diesel Error."),
            ServerError::EnvironmentError => HttpResponse::InternalServerError().json("Environment Error."),
            ServerError::UserError(data) => HttpResponse::InternalServerError().json(data),
            _ => panic!(),
        }
    }
}

impl From<std::env::VarError> for ServerError {
    fn from(_: std::env::VarError) -> ServerError {
        ServerError::EnvironmentError
    }
}

impl From<r2d2::Error> for ServerError {
    fn from(_: r2d2::Error) -> ServerError {
        ServerError::R2D2Error
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            diesel::result::Error::NotFound => {
                log::error!("{:?}", err);
                ServerError::UserError("Username not found.".to_string())
            }
            _ => ServerError::DieselError
        }
    }
}


impl From<argonautica::Error> for ServerError {
    fn from(_: argonautica::Error) -> ServerError {
        ServerError::ArgonauticError
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)
        .expect("Failed to create postgres pool.");


    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)
            )
            )
            .app_data(Data::new(pool.clone()))
            .service(process_login)
            .service(process_signup)
            .service(web::resource("/check_username_email").route(web::post().to(check_username_email)))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
