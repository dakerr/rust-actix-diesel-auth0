// use crate::auth::validator;
use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use crate::state::new_state;
use actix_web::{middleware::Logger, App, HttpServer};
// use actix_web_httpauth::middleware::HttpAuthentication;


pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in handlers using data: AppState<'_, String>
    let data = new_state::<String>();

    let server = HttpServer::new(move || {
        //  let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(Logger::default())
            .configure(add_pool)
            .app_data(data.clone())
            .configure(routes)
    });

    server
        .bind(&CONFIG.server)?
        .run()
        .await
}