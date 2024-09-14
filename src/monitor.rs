use std::str::FromStr;

use futures::join;
use log::info;
use reqwest::{Client, Proxy};
use serde::{Serialize,Deserialize};
use serde_json::Value;


const CPU_UTIL: &str = r#"100 - (avg by (instance,job) (rate(node_cpu_seconds_total{mode="idle"}[1m])) * 100)"#;
//const TOT_CPU_CORES: &str = r#"count without(cpu, mode) (node_cpu_seconds_total{mode="idle"})"#;
const FREE_MEM: &str = r#"avg by (instance, job) (avg_over_time(node_memory_MemFree_bytes[1m]))"#;
const TOT_MEM: &str = r#"avg by (instance, job) (node_memory_MemTotal_bytes)"#;
const FREE_DISK: &str = r#"floor(avg by (instance, job) (avg_over_time(node_filesystem_free_bytes[5m])))"#;
const TOT_DISK: &str = r#"avg by (instance, job) (node_filesystem_free_bytes)"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct PromConfig {
    pub url: String,
    pub port: u16,
    pub proxy: Option<String>,
}

impl PromConfig {
    pub async fn get_client(&self) -> Client {
        let mut c_builder = Client::builder();
        match &self.proxy {
            Some(proxy_val) => {
                let proxy = Proxy::http(proxy_val);
                c_builder = match proxy {
                    Ok(p) => c_builder.proxy(p),
                    Err(e) => panic!("{:?}", e),
                };
            }
            None => ()
        }
        c_builder = c_builder.danger_accept_invalid_certs(true);
        c_builder.build().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub hostname: String,
    pub address: String,
    pub cpu_util: f64,
    pub cpu_cores: String,
    pub mem_free: String,
    pub mem_total: String,
    pub disk_free: String,
    pub disk_total: String,
}

impl Machine {
    fn default() -> Self {
        Machine {
            hostname: "".to_string(),
            address: "".to_string(),
            cpu_util: 0.0,
            cpu_cores: "".to_string(),
            mem_free: "".to_string(),
            mem_total: "".to_string(),
            disk_free: "".to_string(),
            disk_total: "".to_string(),
        }        
    }
}

pub async fn get_all(prom: &PromConfig) -> Vec<Machine> {
    let mut results = Vec::new();

    let res = join!(
        query(prom, CPU_UTIL),
        query(prom, FREE_MEM),
        query(prom, TOT_MEM),
        query(prom, FREE_DISK),
        query(prom, TOT_DISK),
    );
    let (cpu_util, free_mem, tot_mem, free_disk, tot_disk) = (res.0, res.1, res.2, res.3, res.4);

    get_res(cpu_util, &mut results, "CPU_UTIL").await;
    get_res(free_mem, &mut results, "FREE_MEM").await;
    get_res(tot_mem, &mut results, "TOT_MEM").await;
    get_res(free_disk, &mut results, "FREE_DISK").await;
    get_res(tot_disk, &mut results, "TOT_DISK").await;
    

    info!("{:?}", results);

    results
}

pub async fn get_res(prom_res: Vec<(String, String, String)>, machines: &mut Vec<Machine>, field: &str) {
    
    for i in prom_res.iter() {
        let machine = match machines.iter_mut().find(|x| x.hostname == i.0) {
            Some(m) => m,
            None => {
                let newm = Machine {
                    hostname: i.0.clone(),
                    address: i.1.clone(),
                    ..Machine::default()
                };
                machines.push(newm);
                machines.iter_mut().find(|x| x.hostname == i.0).unwrap()
            },
        };
        match field {
            "CPU_UTIL" => machine.cpu_util = f64::from_str(&i.2).unwrap(),
            "FREE_MEM" => machine.mem_free = i.2.to_owned(),
            "TOT_MEM" => machine.mem_total = i.2.to_owned(),
            "FREE_DISK" => machine.disk_free = i.2.to_owned(),
            "TOT_DISK" => machine.disk_total = i.2.to_owned(),
            &_ => panic!("No field str..."),
        }
    }

}

pub async fn query(prom: &PromConfig, query: &str) -> Vec<(String, String, String)> {
    let mut results = Vec::new();
    let client = prom.get_client().await;
    let res = client.get(format!("{}:{}/api/v1/query?query={}", prom.url, prom.port, query)).send();
    let json: Value = res.await.unwrap().json().await.unwrap();
    let jobj = json.as_object().unwrap();
    let result = jobj.get("data").unwrap().get("result").unwrap().as_array().unwrap();
    for i in result.iter() {
        info!("{}", serde_json::to_string_pretty(&json).unwrap());
        let name = i.get("metric").unwrap().get("job").unwrap().as_str().unwrap().to_string();
        let addr = i.get("metric").unwrap().get("instance").unwrap().as_str().unwrap().to_string();
        let value = i.get("value").unwrap().as_array().unwrap()[1].as_str().unwrap().to_string();
        results.push(
            (name, addr, value)
        );        
    }

    results
}
