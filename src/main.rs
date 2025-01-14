use std::{sync::{Arc, Mutex, RwLock}, time::Instant};
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use rand::distributions::{Alphanumeric, DistString};
use sqlx::{prelude::FromRow, sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Deserialize)]
struct CredentialBody {
    username: String,
    password: String
}

#[derive(Serialize, FromRow)]
struct User {
    username: String,
    cracked: u16,
    rank: Option<u32>
}

#[derive(Serialize)]
struct WebResponse {
    message: String,
}

#[derive(Deserialize)]
struct WebRequest {
    code: String,
    username: String,
    password: String
}

#[derive(Serialize)]
struct Webhook {
    username: String,
    avatar_url: Option<String>,
    content: String
}

struct AppState {
    currentcode: RwLock<String>,
    code_length: RwLock<u8>,
    client: Arc<Mutex<Pool<Sqlite>>>
}

#[derive(Serialize)]
struct Statistics {
    code_length: u8,
    server_uptime: u64,
}

#[get("/code-game/user/{username}")]
async fn get_user(username: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let client = data.client.lock().unwrap();

    let user: User = match sqlx::query_as(
        "
        SELECT username, cracked, rank FROM users
        WHERE
            username = $1

        LIMIT 1
        "
    )
    .bind(username.as_str())
    .fetch_one(&*client).await {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::NotFound().json(&WebResponse {
                message: "User not found".to_string()
            })
        }
    };

    HttpResponse::Ok().json(&user)
}

#[post("/code-game/user/register")]
async fn register_user(cred: web::Json<CredentialBody>, data: web::Data<AppState>) -> impl Responder {
    let client = data.client.lock().unwrap();
    match sqlx::query(
        "
        INSERT INTO users (username, password, cracked)
        VALUES ($1, $2, 0)
        "
    )
    .bind(cred.username.clone())
    .bind(cred.password.clone())
    .execute(&*client).await {
        Ok(_) => {
            return HttpResponse::Ok().json(&WebResponse {
                message: "Account made.".to_string()
            })
        },
        Err(_) => {
            return HttpResponse::ImATeapot().json(&WebResponse {
                message: "I'm a teapot.".to_string()
            })
        }
    }
}

#[delete("/code-game/user/{username}")]
async fn delete_user(cred: web::Json<CredentialBody>, data: web::Data<AppState>) -> impl Responder {
    let client = data.client.lock().unwrap();
    match sqlx::query(
        "
        DELETE FROM users
        WHERE EXISTS (
            username = $1
            AND
            password = $2
        )
        "
    )
    .bind(cred.username.clone())
    .bind(cred.password.clone())
    .execute(&*client).await {
        Ok(_) => {
            return HttpResponse::Ok().json(&WebResponse {
                message: "Account has been deleted".to_string()
            })
        },
        Err(_) => {
            return HttpResponse::Forbidden().json(&WebResponse {
                message: "Cannot find user.".to_string()
            })
        }
    }
}

#[get("/code-game/statistics")]
async fn get_statistics(data: web::Data<AppState>) -> HttpResponse {
    let now = Instant::now();

    HttpResponse::Ok().json(&Statistics {
        code_length: *data.code_length.read().unwrap(),
        server_uptime: now.elapsed().as_secs()
    })
}

#[post("/code-game/code")]
async fn try_code(req: web::Json<WebRequest>, data: web::Data<AppState>) -> HttpResponse {
    let mut code = data.currentcode.write().unwrap();
    let code_length = data.code_length.write().unwrap();
    let client = reqwest::Client::new();

    if req.code == *code {
        let mut rng = rand::thread_rng();
        *code = Alphanumeric.sample_string(&mut rng, (*code_length).into()).to_lowercase();

        let username = &*req.username;
        let password = &*req.password;

        let _ = client.post(format!("{}", dotenvy::var("WEBHOOK").unwrap()))
            .body(serde_json::to_string(&Webhook {
                username: format!("Code Cracker"),
                avatar_url: None,
                content: format!("> `[+] A code has been cracked by {}!`", username)
            }).unwrap())
            .header("Content-Type", "application/json")
            .send()
            .await;
        
        let client  = data.client.lock().unwrap();
        
        let user: User = sqlx::query_as(
            "
            SELECT * FROM users 
            WHERE 
                username = $1 
                AND
                password = $2
                
            LIMIT 1;
            "
        )
        .bind(username)
        .bind(password)
        .fetch_one(&*client).await.unwrap();

        let _ = sqlx::query(
            "
            UPDATE users
            SET cracked = cracked + 1
            WHERE
                username = $1
                AND
                password = $2
            "
        )
        .bind(username)
        .bind(password)
        .execute(&*client).await.unwrap();

        println!("[-] The code has been cracked by {}!", user.username);
        println!("[+] Current Code to crack: {}", code);

        return HttpResponse::Ok().json(&WebResponse { message: ("Code Cracked!").to_string()})
    }

    HttpResponse::Unauthorized().json(&WebResponse { message: ("Incorrect Code.").to_string()})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().unwrap();
    let client = SqlitePoolOptions::new().connect("./db/db.sqlite?mode=rwc").await.unwrap();

    let _ = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            cracked INTEGER NOT NULL,
            rank INTEGER
        )
        "
    )
    .execute(&client).await;
    
    let mut rng = rand::thread_rng();
    let code = Alphanumeric.sample_string(&mut rng, 4).to_lowercase();

    let currentcode = web::Data::new(AppState {
        currentcode: RwLock::new(code.to_string()),
        code_length: RwLock::new(4),
        client: Arc::new(Mutex::new(client))
    });

    println!("[+] Current Code to crack: {}", code);

    HttpServer::new(move || {
        App::new()
        .service(try_code)
        .service(get_statistics)
        .service(register_user)
        .service(get_user)
        .service(delete_user)
        .app_data(currentcode.clone())
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}