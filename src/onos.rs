use reqwest::header::HeaderMap;
use std::collections::HashMap;
use serde_yaml::Value;

pub async fn get_hosts() {
    let client = reqwest::Client::new();

    let req = client.get("http://127.0.0.1:8181/onos/v1/hosts")
                    .basic_auth("onos", Some("rocks"))
                    .send()
                    .await.unwrap();
    let json: Value = req.json().await.unwrap();
    println!("{:?}", json);
}

pub async fn get_switch_ports(device_id: String) {
    let client = reqwest::Client::new();
    let req = client.get("http://127.0.0.1:8181/onos/v1/devices")
                    .basic_auth("onos", Some("rocks"))
                    .send()
                    .await.unwrap();
    let devices: Value = req.json().await.unwrap();
    println!("{:?}", devices);
}

pub async fn add_flow(device: String) {
    let client = reqwest::Client::new();
    let map = format!(r#"
        {{
          "flows": [
            {{
              "priority": 100,
              "timeout": 0,
              "isPermanent": true,
              "deviceId": "{device}",
              "treatment": {{
                "instructions": [
                  {{
                    "type": "OUTPUT",
                    "port": "CONTROLLER"
                  }}
                ]
              }},
              "selector": {{
                "criteria": [
                  {{
                    "type": "IPV4_DST",
                    "ip": "192.0.0.17/32"
                  }},
                  {{
                    "type": "IPV4_SRC",
                    "ip": "192.0.0.0/8"
                  }}
                ]
              }}
            }}
          ]
        }}"#);

    let m: Value = serde_json::from_str(&map).unwrap();

    let req = client.post("http://127.0.0.1:8181/onos/v1/flows")
                    .basic_auth("onos", Some("rocks"))
                    .json(&m)
                    .send()
                    .await.unwrap();

    println!("{:?}", req.text().await);
}

pub async fn get_switch_id(switch_name: &str) -> Option<String> {
    let client = reqwest::Client::new();
    let req = client.get("http://127.0.0.1:8181/onos/v1/devices")
                    .basic_auth("onos", Some("rocks"))
                    .send()
                    .await.unwrap();
    let devices: Value = req.json().await.unwrap();
    
    for i in devices["devices"].as_sequence().unwrap() {
        let id = i["id"].as_str().unwrap();
        match get_switch_name(id).await {
            Some(i) => {
                if i == switch_name.to_string() {
                    println!("{}", id);
                    return Some(id.to_string())
                }
            },
            None => return None,
        }
    }

    None
}

pub async fn get_switch_name(device_id: &str) -> Option<String> {
    let client = reqwest::Client::new();
    let req = client.get(format!("http://127.0.0.1:8181/onos/v1/devices/{device_id}/ports"))
                    .basic_auth("onos", Some("rocks"))
                    .send()
                    .await.unwrap();
    let devices: Value = req.json().await.unwrap();
    for i in devices["ports"].as_sequence().unwrap() {
        if let Value::String(p_type) = &i["port"] {
            if *p_type == "local".to_string() {
                return Some(i["annotations"]["portName"].as_str().unwrap().to_string());
            }
        }
    }
    return None
}

pub async fn get_host_id() {

}

pub async fn add_host_intent(host_one: &str, host_two: &str) {
    let client = reqwest::Client::new();
    let map = format!(r#"
    {{
        "type": "HostToHostIntent",
        "appId": "org.onosproject/ovsdb",
        "priority": 55,
        "one": "{host_one}",
        "two": "{host_two}",
    }}"#);

    let m: Value = serde_json::from_str(&map).unwrap();
    let req = client.post("http://127.0.0.1:8181/onos/v1/intents")
                    .basic_auth("onos", Some("rocks"))
                    .json(&m)
                    .send()
                    .await.unwrap();
}
