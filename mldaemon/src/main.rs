
mod mldaemon_model;
mod mldaemon_utils;
mod test_data;

use std::fs::{ReadDir};
use actix_cors::Cors;

use actix_web::{web, post, get, App, HttpServer, Responder, HttpResponse};
use chrono::Utc;
use graymat::activation_function::ActivationFunction;
use graymat::neural_network::NeuralNetwork;
use crate::mldaemon_model::MlDaemonModel;
use crate::mldaemon_utils::{get_models_directory_path, make_dir_if_not_present};
use serde::{Deserialize};
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

#[derive(Deserialize)]
struct NewModelInfo {
    model_name: String,
    layer_neurons: Vec<u8>,
    activation_function_id: u8
}

#[post("/new-model")]
async fn new_model(info: web::Json<NewModelInfo>) -> impl Responder {

    let models_dir = get_models_directory_path();
    make_dir_if_not_present(&models_dir);

    let nn = parse_new_model_info_to_neural_network(&info);

    let model = MlDaemonModel::new(&info.model_name, nn);

    model.save(models_dir);

    return HttpResponse::Ok().finish();
}

fn parse_new_model_info_to_neural_network(info: &web::Json<NewModelInfo>) -> NeuralNetwork {

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

#[post("/add-test-data/{model_id}")]
async fn add_test_data(model_id_path: web::Path<String>,
                       test_data_json: web::Json<TestData>) -> impl Responder
{
    let models_dir = get_models_directory_path();
    let model_id_string: String = model_id_path.into_inner();
    let target_model_dir = models_dir.join(&model_id_string);
    if !target_model_dir.exists() {
        return HttpResponse::NoContent().body(format!("Mode {} not found",
                                                      model_id_string));
    }

    let test_data = test_data_json.into_inner();

    let test_data_file_path = target_model_dir.join(test_data.label.to_owned());
    if test_data_file_path.exists() {
        return HttpResponse::BadRequest().body(format!("Test data with label {} already present",
                                                       test_data.label.to_owned()));
    }

    test_data.to_file(test_data_file_path.to_str()
                                         .unwrap()
                                         .to_string());

    return HttpResponse::Ok().finish();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {

        // let api_service = web::scope("/api");

        let cors = Cors::default().allow_any_origin();

        App::new().wrap(cors)
                  .service(get_models)
                  .service(new_model)
                  .service(ping)
                  .service(add_test_data)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}






