use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::{PgConnection, PgPool};

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("health_check", web::get().to(health_check))
            .route("subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
