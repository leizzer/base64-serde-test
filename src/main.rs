#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate base64_serde;

use base64::STANDARD;
base64_serde_type!(Base64Standard, STANDARD);

extern crate log;
extern crate env_logger;

use actix_web::{web, App, HttpResponse, HttpServer, FromRequest, Error};
use actix_web::middleware::Logger;

use std::io;

use futures::Future;

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateParams {
    pub name: Option<String>,
    #[serde(with = "Base64Standard")]
    #[serde(default)]
    pub file: Vec<u8>,
}

pub fn update(
    card_id: web::Path<i32>,
    card_update: web::Json<UpdateParams>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        update_card(card_id.into_inner(), card_update.into_inner())
    })
    .then(|res| match res {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(_) => Ok(HttpResponse::NotFound().into()),
    })
}

fn update_card(id: i32, card_update: UpdateParams) -> Result<UpdateParams, String>{
    println!(
        "{} : {:?}",
        id, card_update
        );


    Ok(card_update)
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
    let sys = actix_rt::System::new("base64-option");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::resource("/{card_id}").data(
                    web::Json::<UpdateParams>::configure(|cfg| {
                        cfg.limit(4194304) //4MB
                    })
                )
                .route(web::put().to_async(update))
            )
    })
    .bind("localhost:3000")?
    .start();

    sys.run()
}
