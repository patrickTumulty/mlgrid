
mod mldaemon_model;
mod mldaemon_utils;
mod test_data;

mod instance_manager;

use std::{fs};
use std::fs::{ReadDir};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;

use actix_web::{web, post, get, App, HttpServer, Responder, HttpResponse, http, HttpResponseBuilder};

use actix_web::http::StatusCode;
use chrono::{Local, Utc};
use ndarray::{Array2};
use graymat::activation_function::ActivationFunction;
use graymat::neural_network::NeuralNetwork;
use crate::mldaemon_model::{MlDaemonModel, MODEL_INFO_BIN, ModelInfo};
use crate::mldaemon_utils::{get_models_directory_path, hash_string, make_dir_if_not_present};
use serde::{Deserialize};
use graymat::column_vector::ColumnVector;
use crate::instance_manager::InstanceManager;
use crate::test_data::TestData;


#[derive(Deserialize)]
struct NetworkTrainingParameters {
    iterations: u32,
    batch_size: usize,
    learning_rate: f32
}

#[post("/train-network/{model_name}")]
async fn train_network(model_name: web::Path<String>,
                       network_training_params: web::Json<NetworkTrainingParameters>,
                       instance_manager_ptr: web::Data<Arc<Mutex<InstanceManager<MlDaemonModel>>>>) -> impl Responder
{
    let instance_manager = instance_manager_ptr.into_inner().clone();
    let instance = instance_manager.lock()
                                   .unwrap()
                                   .get(model_name.as_str());

    let iterations = network_training_params.iterations;
    let batch_size = network_training_params.batch_size;
    let learning_rate = network_training_params.learning_rate;

    let mut training_data: Vec<(ColumnVector, ColumnVector)> = Vec::new();

    let models_dir = get_models_directory_path();
    let selected_model_dir = models_dir.join(model_name.as_str());
    let test_data_dir = selected_model_dir.join(TEST_DATA_DIR_NAME);
    let test_data_files = test_data_dir.read_dir().unwrap();
    for test_data_file in test_data_files {
        if test_data_file.is_err() {
            continue;
        }
        let path = test_data_file.unwrap().path().to_str().unwrap().to_string();
        let test_data = TestData::from_file(path);
        let input_data = ColumnVector::from_vec(test_data.data);
        let target_result = ColumnVector::from_vec(test_data.target);
        training_data.push((input_data, target_result));
    }

    println!("Training starting");

    instance.unwrap().lock().unwrap().neural_network_mut()
                                     .train(training_data,
                                            iterations,
                                            batch_size,
                                            learning_rate);

    println!("Training done");

    return HttpResponse::Ok().finish();
}


#[derive(Deserialize)]
struct NetworkInputData {
    data: Vec<Vec<f32>>
}

#[post("/evaluate-network/{model_name}")]
async fn evaluate_network(model_name: web::Path<String>,
                          network_input_data: web::Json<NetworkInputData>,
                          instance_manager_ptr: web::Data<Arc<Mutex<InstanceManager<MlDaemonModel>>>>) -> impl Responder
{
    let lock = instance_manager_ptr.lock();
    if lock.is_err() {
        return HttpResponse::NoContent().finish();
    }

    let instance = lock.unwrap().get(model_name.as_str());
    if instance.is_none() {
        return HttpResponse::NoContent().finish();
    }

    let data: Vec<Vec<f32>> = network_input_data.into_inner().data;
    let data_copy = vec_matrix_to_array2(data);

    let cvec = ColumnVector::from(&data_copy);
    let result = instance.unwrap().lock().unwrap().neural_network().evaluate(cvec);

    // println!("{}", result);

    return HttpResponseBuilder::new(StatusCode::OK).json(result.get_data().to_owned().into_raw_vec());
}

fn vec_matrix_to_array2(data: Vec<Vec<f32>>) -> Array2<f32> {
    let rows = data.len();
    let cols = data[0].len();
    let mut data_copy: Array2<f32> = Array2::zeros((cols, rows));
    for i in 0..rows {
        for j in 0..cols {
            data_copy[[i, j]] = data[i][j];
        }
    }
    data_copy
}


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

    let models: ReadDir = models_dir.read_dir()
                                    .unwrap();
    let mut models_vec: Vec<String> = Vec::new();
    for model in models {
        if model.is_ok() {
            let dir_name = model.unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .to_owned();
            if dir_name.starts_with(".") {
                continue;
            }
            models_vec.push(dir_name);
        }
    }

    return web::Json(models_vec);
}

#[get("/get-model-info/{model_name}")]
async fn get_model_info(model_name: web::Path<String>,
                        instance_manager_ptr: web::Data<Arc<Mutex<InstanceManager<MlDaemonModel>>>>)
                        -> impl Responder
{
    let instance_manager = instance_manager_ptr.into_inner().clone();
    let instance = instance_manager.lock()
                                   .unwrap()
                                   .get(model_name.as_str());

    if instance.is_none() {
        return HttpResponse::NoContent().finish();
    }

    return HttpResponse::Ok().json(web::Json(instance.unwrap().lock().unwrap().model_info().clone()));
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
                                   new_model_info.layer_output_labels, 0);

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

const TEST_DATA_DIR_NAME: &'static str = "test_data";

#[post("/add-test-data/{model_name}")]
async fn add_test_data(model_name: web::Path<String>,
                       test_data_json: web::Json<TestData>,
                       instance_manager_ptr: web::Data<Arc<Mutex<InstanceManager<MlDaemonModel>>>>) -> impl Responder
{
    let instance_manager = instance_manager_ptr.into_inner().clone();
    let instance = instance_manager.lock()
                                                       .unwrap()
                                                       .get(model_name.as_str());
    if instance.is_none() {
        return HttpResponse::NoContent().finish();
    }
    instance.unwrap().lock().unwrap().increment_total_test_examples();

    let model_name_path = model_name.into_inner();
    let target_model_dir = get_target_model_dir(&model_name_path);
    if target_model_dir.is_none() {
        return HttpResponse::NoContent().body(format!("Mode {} not found",
                                                      model_name_path));
    }

    let test_data = test_data_json.into_inner();
    let test_data_dir = target_model_dir.unwrap().join(TEST_DATA_DIR_NAME);

    make_dir_if_not_present(&test_data_dir);

    let hash_title = hash_string(Local::now().to_string().as_str());

    let test_data_file_path = test_data_dir.join(hash_title);

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
    let instance_manager_ptr: Arc<Mutex<InstanceManager<MlDaemonModel>>> = Arc::new(Mutex::new(InstanceManager::new()));

    InstanceManager::start(instance_manager_ptr.clone());

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin()
                                  .allowed_methods(vec!["GET", "POST"])
                                  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                                  .allowed_header(http::header::CONTENT_TYPE);

        App::new().wrap(cors)
                  .app_data(web::Data::new(instance_manager_ptr.clone()))
                  .service(get_models)
                  .service(get_model_info)
                  .service(new_model)
                  .service(ping)
                  .service(add_test_data)
                  .service(delete_model)
                  .service(evaluate_network)
                  .service(train_network)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}






