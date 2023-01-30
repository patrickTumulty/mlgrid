
mod mldaemon_model;
mod mldaemon_utils;
mod test_data;

mod instance_manager;

use std::fs;
use std::fs::{ReadDir};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;

use actix_web::{web, post, get, App, HttpServer, Responder, HttpResponse, http};
use chrono::Utc;
use graymat::activation_function::ActivationFunction;
use graymat::neural_network::NeuralNetwork;
use crate::mldaemon_model::{MlDaemonModel, MODEL_INFO_BIN, ModelInfo};
use crate::mldaemon_utils::{get_models_directory_path, make_dir_if_not_present};
use serde::{Deserialize};
use crate::instance_manager::InstanceManager;
use crate::test_data::TestData;


#[get("/ping")]
async fn ping() -> impl Responder {
    let now = Utc::now();
    let now_str = now.to_rfc3339();
    format!("I'm Alive: {}", now_str)
}

#[get("/get-models")]
async fn get_models() -> impl Responder {

    let models_dir = get_models_directory_path();

    make_dir_if_not_present(&models_dir);

    let models: ReadDir = models_dir.read_dir().unwrap();
    let mut models_vec: Vec<String> = Vec::new();
    for model in models {
        models_vec.push(model.unwrap().file_name().to_str().unwrap().to_owned());
    }

    return web::Json(models_vec);
}

#[get("/get-model-info/{model_name}")]
async fn get_model_info(model_name: web::Path<String>) -> impl Responder {
    let models_dir = get_models_directory_path();
    if !&models_dir.exists() {
        return HttpResponse::BadRequest().body("No models found");
    }
    let model_name_string = model_name.into_inner();
    let model_dir = models_dir.join(model_name_string.to_owned());
    if !&model_dir.exists() {
        return HttpResponse::BadRequest().body(format!("Model {} not found", model_name_string));
    }

    let model_info: ModelInfo = ModelInfo::from_file(&model_dir.join(MODEL_INFO_BIN));

    return HttpResponse::Ok().json(web::Json(model_info));
}

#[derive(Deserialize)]
struct NewModelInfo {
    model_name: String,
    layer_neurons: Vec<u32>,
    activation_function_id: u8,
    layer_output_labels: Vec<String>
}

#[post("/new-model")]
async fn new_model(info: web::Json<NewModelInfo>) -> impl Responder {

    let models_dir = get_models_directory_path();
    make_dir_if_not_present(&models_dir);

    let new_model_info = info.into_inner();

    let nn = parse_new_model_info_to_neural_network(&new_model_info);

    let model = MlDaemonModel::new(&new_model_info.model_name,
                                   nn,
                                   new_model_info.layer_output_labels);

    model.save(models_dir);

    return HttpResponse::Ok().finish();
}

fn parse_new_model_info_to_neural_network(info: &NewModelInfo) -> NeuralNetwork {

    let layer_neurons = &info.layer_neurons;
    let input_neurons = layer_neurons[0] as usize;
    let output_neurons = layer_neurons[layer_neurons.len() - 1] as usize;
    let mut hidden_neurons: Vec<usize> = Vec::new();
    for i in 1..(layer_neurons.len() - 1) {
        hidden_neurons.push(layer_neurons[i] as usize);
    }

    let af_option = ActivationFunction::from_u8(info.activation_function_id);
    let mut af = ActivationFunction::SIGMOID;
    if !af_option.is_none() {
        af = af_option.unwrap();
    }

    return NeuralNetwork::new(input_neurons,
                              output_neurons,
                              hidden_neurons,
                              af);
}

#[post("/add-test-data/{model_name}")]
async fn add_test_data(model_name: web::Path<String>,
                       test_data_json: web::Json<TestData>) -> impl Responder
{
    let model_name_path = model_name.into_inner();
    let target_model_dir = get_target_model_dir(&model_name_path);
    if target_model_dir.is_none() {
        return HttpResponse::NoContent().body(format!("Mode {} not found",
                                                      model_name_path));
    }

    let test_data = test_data_json.into_inner();
    let test_data_dir = target_model_dir.unwrap().join("test_data");

    make_dir_if_not_present(&test_data_dir);

    let test_data_file_path = test_data_dir.join(test_data.label.to_owned());
    if test_data_file_path.exists() {
        return HttpResponse::BadRequest().body(format!("Test data with label {} already present",
                                                       test_data.label.to_owned()));
    }

    test_data.to_file(test_data_file_path.to_str()
                                         .unwrap()
                                         .to_string());

    return HttpResponse::Ok().finish();
}

fn get_target_model_dir(model_name_path: &str) -> Option<PathBuf> {
    let models_dir = get_models_directory_path();
    let target_model_dir = models_dir.join(model_name_path);
    if !target_model_dir.exists() {
        return None;
    }
    Some(target_model_dir)
}

#[post("/delete-model/{model_name}")]
async fn delete_model(model_name_path: web::Path<String>) -> impl Responder {

    let model_name_path = model_name_path.into_inner();
    let target_model_dir = get_target_model_dir(&model_name_path);
    if target_model_dir.is_none() {
        return HttpResponse::NoContent().body(format!("Mode {} not found",
                                                      model_name_path));
    }

    let result = fs::remove_dir_all(target_model_dir.unwrap());
    if result.is_err() {
        return HttpResponse::NoContent().body("Error clearing model");
    }

    return HttpResponse::Ok().finish();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let instance_manager: Arc<Mutex<InstanceManager<MlDaemonModel>>> = Arc::new(Mutex::new(InstanceManager::new()));

    InstanceManager::start(instance_manager.clone());

    HttpServer::new(|| {

        // let api_service = web::scope("/api");

        let cors = Cors::default().allow_any_origin()
                                  .allowed_methods(vec!["GET", "POST"])
                                  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                                  .allowed_header(http::header::CONTENT_TYPE);

        App::new().wrap(cors)
                  .service(get_models)
                  .service(get_model_info)
                  .service(new_model)
                  .service(ping)
                  .service(add_test_data)
                  .service(delete_model)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}






