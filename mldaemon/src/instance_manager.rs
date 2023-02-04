use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::thread::JoinHandle;
use chrono::{DateTime, Local};

struct InstanceWrapper<T>
    where T: Send
{
    instance: Arc<Mutex<T>>,
    latest_access_time: DateTime<Local>
}

impl <T> InstanceWrapper<T>
    where T: Send
{
    pub fn new(instance: T) -> InstanceWrapper<T> {
        return Self {
            instance: Arc::new(Mutex::new(instance)),
            latest_access_time: Local::now()
        }
    }

    pub fn get(&mut self) -> &mut Arc<Mutex<T>> {
        return &mut self.instance;
    }

    pub fn latest_access_time(&self) -> DateTime<Local> {
        self.latest_access_time.clone()
    }

    pub fn set_latest_access_time(&mut self) {
        self.latest_access_time = Local::now();
    }
}

pub trait InstanceIdInitializer<T>
    where T: Send
{
    fn get_id(&self) -> String;

    fn init(instance_id: &str) -> Option<T>;
}

pub struct InstanceManager<T>
    where T: InstanceIdInitializer<T> + Send + Sync
{
    instance_lookup: HashMap<String, InstanceWrapper<T>>,
    background_thread_running: bool,
    thread_handle: Option<JoinHandle<()>>
}

impl <T> InstanceManager<T>
    where T: InstanceIdInitializer<T> + Send + Sync + 'static
{
    pub fn new() -> InstanceManager<T> {
        return InstanceManager {
            instance_lookup: HashMap::new(),
            background_thread_running: true,
            thread_handle: None
        };
    }

    pub fn start(instance_manager: Arc<Mutex<InstanceManager<T>>>) {

        let im_ptr = instance_manager.clone();
        let im_ptr_clone = im_ptr.clone();
        im_ptr.lock().unwrap().set_thread_handle(thread::spawn(move ||
        {
            loop
            {
                thread::sleep(time::Duration::from_secs(10));

                let guard = im_ptr_clone.lock();
                if guard.is_err() {
                    println!("Failed to acquire lock");
                    continue;
                }

                let mut instance = guard.unwrap();

                if !instance.background_thread_running {
                    return;
                }

                let now = Local::now();

                let mut instance_clear_list: Vec<String> = Vec::new();

                for instance_key in instance.instance_lookup.keys() {
                    let instance = instance.instance_lookup.get(instance_key);
                    if instance.is_some() {
                        let delta = now - instance.unwrap().latest_access_time();
                        if delta.num_seconds() >= 30 {
                            instance_clear_list.push(instance.unwrap().instance.lock().unwrap().get_id());
                        }
                    }
                }

                for instance_id in instance_clear_list {
                    instance.instance_lookup.remove(instance_id.as_str());
                }
            }
        }));
    }

    pub fn get(&mut self, instance_id: &str) -> Option<Arc<Mutex<T>>> {
        let instance_option = self.instance_lookup.get_mut(instance_id);

        if instance_option.is_none() {
            let instance_option = T::init(instance_id);
            if instance_option.is_none() {
                return None; // Failed to init
            }
            let mut instance_wrapper = InstanceWrapper::new(instance_option.unwrap());
            instance_wrapper.set_latest_access_time();
            let instance_ptr = instance_wrapper.instance.clone();
            self.instance_lookup.insert(instance_id.to_string(),
                                        instance_wrapper);
            return Some(instance_ptr);
        }
        return Some(instance_option.unwrap().get().clone());
    }

    pub fn set_thread_handle(&mut self, thread_handle: JoinHandle<()>) {
        self.thread_handle = Some(thread_handle);
    }
}
