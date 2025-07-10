#[macro_use] extern crate rocket;

// Importa o módulo JSON do Rocket para trabalhar com dados JSON
use rocket::serde::json::{Value, json}; 

// Importa o módulo de resposta do Rocket para trabalhar com respostas HTTP
use rocket::response::status; 
use rocket::http::{Status, Header};
use rocket::request::{FromRequest, Outcome};
use rocket::request::Request;

// Estrutura para armazenar as credenciais
#[derive(Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

// Constantes para as credenciais
const VALID_USERNAME: &str = "meuapp";
const VALID_PASSWORD: &str = "123456";

// Implementação do FromRequest para BasicAuth
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid_credentials(username: &str, password: &str) -> bool {
            username == VALID_USERNAME && password == VALID_PASSWORD
        }
        // Verifica se o header Authorization está presente
        match request.headers().get_one("Authorization") {
            None => Outcome::Error((Status::Unauthorized, ())),
            Some(header) => {
                let split = header.split_whitespace().collect::<Vec<_>>();
                if split.len() != 2 || split[0] != "Basic" {
                    return Outcome::Error((Status::Unauthorized, ()));
                }
                // Decodifica o token Basic
                match base64::decode(split[1]) {
                    Ok(credentials) => {
                        let decoded = String::from_utf8(credentials).unwrap_or_default();
                        let split: Vec<&str> = decoded.split(':').collect();
                        
                        if split.len() != 2 {
                            return Outcome::Error((Status::Unauthorized, ()));
                        }
                        // Separa o username e a senha
                        let (username, password) = (split[0], split[1]);
                        // Verifica se as credenciais são válidas
                        if is_valid_credentials(username, password) {
                            Outcome::Success(BasicAuth {
                                username: username.to_string(),
                                password: password.to_string(),
                            })
                        } else {
                            Outcome::Error((Status::Unauthorized, ()))
                        }
                    }
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
        }
    }
}

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value { // Define uma rota GET para o caminho "/rustaceans"
    json!([{"id": 1, "name": "John Doe" }, {"id": 2, "name": "Jane Doe again"}])
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe","email": "John@doe.com" })
}
#[post("/rustaceans", format = "json")] // Define uma rota POST para o caminho "/rustaceans"
fn crate_rustacean(_auth: BasicAuth) -> Value { // Cria um novo rustacean
    json!({"id": 3, "name": "John Doe", "email": "John@doe.com"})
} 
#[put("/rustaceans/<id>", format = "json")] // Rota para atualizar um rustacean existente
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value { // Atualiza um rustacean existente
    json!({"id": id, "name": "John Doe", "email": "John@doe.com"})
}
#[delete("/rustaceans/<_id>")] // Underline no id para indicar que não será usado agora
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}
#[catch(404)] // Define um manipulador de erro para o código de status 404
fn not_found() -> Value {
    json!("Status: Not Found" ) // Retorna uma mensagem JSON para o erro 404
}


// #[rocket::main] // Define a função principal como assíncrona e inicializa o Rocket
#[rocket::main] 
async fn main() { 
    let _ = rocket::build()// Cria uma nova instância do Rocket
        .mount("/", routes![
            get_rustaceans, // Monta a rota para obter todos os rustaceans
            view_rustacean, // Monta a rota para visualizar um rustacean específico
            crate_rustacean, // Monta a rota para criar um novo rustacean
            update_rustacean, // Monta a rota para atualizar um rustacean existente
            delete_rustacean, // Monta a rota para excluir um rustacean
        ]) // Monta a rota definida acima
        .register("/", catchers![
            not_found]) // Registra o manipulador de erro para o código de status 404
        .launch()   // Lança o servidor Rocket
        .await;     // Aguarda o término do lançamento do servidor
}
/*
Class 5
#[get("/")] // #[get"/"] // Define uma rota GET para o caminho "/"

fn hello() -> Value {
   json!("Hello, world!")
} */