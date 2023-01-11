mod model_manager;
mod mldaemon_model;
mod mldaemon_utils;

use std::env::current_dir;
use std::fs::{ReadDir};

use std::path::PathBuf;
use actix_web::{web, post, get, App, HttpRequest, HttpServer, Responder, HttpResponse};
use chrono::Utc;
use graymat::activation_function::ActivationFunction;
use graymat::neural_network::NeuralNetwork;
use crate::mldaemon_model::MlDaemonModel;
use crate::mldaemon_utils::make_dir_if_not_present;
use serde::Deserialize;


#[get("/ping")]
async fn ping() -> impl Responder {
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    format!("I'm Alive: {}", now_str)
}

#[get("/get-models")]
async fn get_models() -> impl Responder {

    let current_dir: PathBuf = current_dir().unwrap();
    let models_dir: PathBuf = current_dir.join("models");

    make_dir_if_not_present(&models_dir);

    let models: ReadDir = models_dir.read_dir().unwrap();
    let mut models_vec: Vec<String> = Vec::new();
    for model in models {
        models_vec.push(model.unwrap().file_name().to_str().unwrap().to_owned());
    }

    return web::Json(models_vec);
}

#[derive(Deserialize)]
struct NewModelInfo {
    model_name: String
}

#[post("/new-model")]
async fn new_model(info: web::Json<NewModelInfo>) -> impl Responder {

    let current_dir: PathBuf = current_dir().unwrap();
    let models_dir: PathBuf = current_dir.join("models");

    make_dir_if_not_present(&models_dir);

    let nn = NeuralNetwork::new(2, 2, vec![2], ActivationFunction::SIGMOID);

    let model = MlDaemonModel::new(&info.model_name, nn);

    model.save(models_dir);

    return HttpResponse::Ok();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_models)
                  .service(new_model)
                  .service(ping)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}






