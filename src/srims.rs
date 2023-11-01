use std::sync::Arc;
use std::thread;
use std::{error::Error, io, process};
use std::{fs::File};
use csv::{DeserializeRecordsIter, Reader};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use futures::{executor, future};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use super::common::{KGraph, Triple};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "DeviceId")]
    device_id: String,
    #[serde(rename = "HarwareVersion")]
    hardware_version: Option<String>,
    #[serde(rename = "SoftwareVersion")]
    software_version: Option<String>,
    device_spec_name: String,
    device_spec_type: String,
    usage: String,
    device_role: Option<String>,
    inventory_status: Option<String>,
    operational_status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(rename = "DeviceId")]
    device_id: String,
    #[serde(rename = "MTOSIName")]
    mtosi_name: String,
    #[serde(rename = "Manufacturer")]
    manufacturer: String,
    #[serde(rename = "SpecType")]
    spec_type: String,
    #[serde(rename = "SpecName")]
    spec_name: String,
    spec_version: String,
    serial_number: Option<String>,
    #[serde(rename = "administrativeState")]
    admin_state: String,
    inventory_status: String,
    operational_status: String,
    #[serde(rename = "parentMTOSIName")]
    parent_mtosi_name: String,
    //#[serde(skip_serializing_if = "Option::is_none")]
    parent_position: Option<String>,
    parent_equipment_class: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    #[serde(rename = "DeviceId")]
    device_id: String,
    port_name: String,
    #[serde(rename = "MTOSIName")]
    mtosi_name: String,
    #[serde(rename = "Type")]
    port_type: String,
    port_signal_type: String,
    window_type: String, 
    #[serde(rename = "administrativeState")]
    administrative_state: String,
    inventory_status: String,
    operational_status: String,
    #[serde(rename = "parentCardMTOSIName")]
    parent_card_mtosi_name: String,
    #[serde(rename = "LagId")]
    lag_id: String,
    #[serde(rename = "Laggroupname")]
    lag_group_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    service_name: String,
    service_status: String,
    #[serde(rename = "SpecName")]
    spec_name: String, 
    ckt_name: String,
    #[serde(rename = "AEnd_Device")]
    aend_device: String,
    #[serde(rename = "AEnd_PhysicalPort")]
    aend_physical_port: String,
    #[serde(rename = "AEnd_PortMTOSIName")]
    aend_port_mtosi_name: String,
    #[serde(rename = "ZEnd_Device")]
    zend_device: String,
    #[serde(rename = "ZEnd_Interface")]
    zend_interface: String, 
    #[serde(rename = "ZEnd_PhysicalPort")]
    zend_physical_port: String,
    #[serde(rename = "ZEnd_PortMTOSIName")]
    zend_port_mtosi_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chassis {
    #[serde(rename = "DeviceId")]
    device_id: String,
    #[serde(rename = "MTOSIName")]
    mtosi_name: String,
    position: Option<String>,
    #[serde(rename = "Manufacturer")]
    manufacturer: String,
    #[serde(rename = "Type")]
    chassis_type: String,
    spec_version: String,
    serial_number: Option<String>,
    #[serde(rename = "administrativeState")]
    admin_state: String,
    inventory_status: String,
    operational_status: String,
    #[serde(rename = "Holder")]
    holder: String,
    #[serde(rename = "Rack")]
    rack: String,
    #[serde(rename = "Suite")]
    suite: String,
    #[serde(rename = "Section")]
    section: String,
    #[serde(rename = "Floor")]
    floor: String,
} 

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BearerConnection {
    #[serde(rename = "DeviceId_A")]
    device_id: String,
    #[serde(rename = "MTOSIName_A")]
    mtosi_name: String,
    lag_id: String,
    lag_group_name: Option<String>,
    bearer_connection: String,
    #[serde(rename = "inventoryStatusBearer")]
    inventory_status_bearer: String,
    #[serde(rename = "operationalStatusBearer")]
    operational_status_bearer: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SRIMS {
    devices: Vec<Device>,
    cards_one: Vec<Card>,
    cards_two: Vec<Card>,
    cards_three: Vec<Card>,
    ports_one: Vec<Port>,
    ports_two: Vec<Port>,
    ports_three: Vec<Port>,
    chassis: Vec<Chassis>,
    services: Vec<Service>,
    bearconns: Vec<BearerConnection>,
}


#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SRIMSDevice {
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "HarwareVersion")]
    pub hardware_version: Option<String>,
    #[serde(rename = "SoftwareVersion")]
    pub software_version: Option<String>,
    pub device_spec_name: String,
    pub device_spec_type: String,
    pub usage: String,
    pub device_role: Option<String>,
    pub inventory_status: Option<String>,
    pub operational_status: Option<String>,
    pub chassis: Chassis,
    pub card_one: Card,
    pub card_two: Card,
    pub card_three: Card,
    pub port_one: Port,
    pub port_two: Port,
    pub port_three: Port,
}

impl SRIMS {
    fn new() -> Self {
        SRIMS{
            devices: Vec::new(),
            cards_one: Vec::new(),
            cards_two: Vec::new(),
            cards_three: Vec::new(),
            ports_one: Vec::new(),
            ports_two: Vec::new(),
            ports_three: Vec::new(),
            chassis: Vec::new(),
            services: Vec::new(),
            bearconns: Vec::new(),
        }
    }
}


pub fn read_srims() -> SRIMS {
    let mut srims = SRIMS::new();
    let mut devices = csv::Reader::from_path("./res/SRIMS/Device.csv").unwrap();
    let mut cards_one = csv::Reader::from_path("./res/SRIMS/Card_1.csv").unwrap();
    let mut cards_two = csv::Reader::from_path("./res/SRIMS/Card_2.csv").unwrap();
    let mut cards_three = csv::Reader::from_path("./res/SRIMS/Card_3.csv").unwrap();
    let mut ports_one = csv::Reader::from_path("./res/SRIMS/Port_1.csv").unwrap();
    let mut ports_two = csv::Reader::from_path("./res/SRIMS/Port_2.csv").unwrap();
    let mut ports_three = csv::Reader::from_path("./res/SRIMS/Port_3.csv").unwrap();
    let mut chassis = csv::Reader::from_path("./res/SRIMS/Chassis.csv").unwrap();
    let mut services = csv::Reader::from_path("./res/SRIMS/Service_1.csv").unwrap();
    let mut bearconns = csv::Reader::from_path("./res/SRIMS/BearerConnection.csv").unwrap();

    get_thing(&mut devices, &mut srims.devices);
    get_thing(&mut chassis, &mut srims.chassis);
    get_thing(&mut bearconns, &mut srims.bearconns);
    get_thing(&mut services, &mut srims.services);
    get_thing(&mut cards_one, &mut srims.cards_one);
    get_thing(&mut cards_two, &mut srims.cards_two);
    get_thing(&mut cards_three, &mut srims.cards_three);
    get_thing(&mut ports_one, &mut srims.ports_one);
    get_thing(&mut ports_two, &mut srims.ports_two);
    get_thing(&mut ports_three, &mut srims.ports_three);

    srims
}

fn get_thing<'a, D: DeserializeOwned>(file: &mut Reader<File>, vec: &mut Vec<D>) {
    file.deserialize::<D>().into_iter().for_each(|result| {
        match result {
            Ok(o) => vec.push(o),
            Err(err) => println!("Error with {:?}", err),
        }
    });
}

pub fn get_all_chassis(srims: &mut SRIMS) -> Vec<Chassis> {
    let mut chassis: Vec<Chassis> = Vec::new();

    for i in 0..=srims.chassis.len() {
        let cha = srims.chassis.pop();                
        match cha {
            Some(i) => {
                let index = chassis.iter().position(|e| {
                    e.holder == i.holder && 
                    e.rack == i.rack &&
                    e.suite == i.suite &&
                    e.section == i.section &&
                    e.floor == i.floor
                }); 
                match index {
                    None => {
                        chassis.push(i);
                    },
                    _ => ()
                }
            },
            None => break,
        }
    }

    chassis
} 

pub fn srims_to_rdf(srims: &mut SRIMS) -> Vec<SRIMSDevice> {
    let chassis = get_all_chassis(srims);

    let mut all: Vec<SRIMSDevice> = Vec::new();
    for dev in srims.devices.iter() {
        let mut col = SRIMSDevice::default();
        col.device_id = dev.device_id.clone();

        let c_one_i = srims.cards_one.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match c_one_i {
            Some(i) => {
                col.card_one = srims.cards_one.swap_remove(i)
            },
            _ => (), 
        }

        let c_two_i = srims.cards_two.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match c_two_i {
            Some(i) => col.card_two = srims.cards_two.swap_remove(i),
            _ => (),
        }

        let c_three_i = srims.cards_three.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match c_three_i {
            Some(i) => col.card_three = srims.cards_three.swap_remove(i),
            _ => (),
        }

        let p_one_i = srims.ports_one.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match p_one_i {
            Some(i) => col.port_one = srims.ports_one.swap_remove(i),
            _ => (),
        }

        let p_two_i = srims.ports_two.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match p_two_i {
            Some(i) => col.port_two = srims.ports_two.swap_remove(i),
            _ => (),
        }
        let p_three_i = srims.ports_three.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match p_three_i {
            Some(i) => col.port_three = srims.ports_three.swap_remove(i),
            _ => (),
        }

        let chassis_i = srims.chassis.iter().position(|x| {
            dev.device_id == x.device_id
        });
        match chassis_i {
            Some(i) => col.chassis = srims.chassis.swap_remove(i),
            _ => (),
        }
        all.push(col)
    } 
    println!("{:?}", all);
    all    
}

pub fn get_graph() -> KGraph {
    let mut files = read_srims();
    let srims = srims_to_rdf(&mut files);
    println!("Here?");

    let mut nodes: Vec<String> = Vec::new();
    let mut links: Vec<Triple> = Vec::new();

    println!("Here?");

    for (i, device) in srims.into_iter().enumerate() {
        if i == 200 {
            break;
        }
        println!("Device numero: {}", i);
        let n_pos = nodes.len();
        let mut next_pos = n_pos + 1;
        nodes.push(device.device_id);

        if device.card_one.spec_name != "" {
            nodes.push(device.card_one.spec_name);
            links.push(Triple {
                s: n_pos.to_string(),
                p: "hasCard".to_string(),
                o: next_pos.to_string(),
            }); 
            next_pos += 1;
        }
        if device.port_one.port_name != "" {
            nodes.push(device.port_one.port_name);
            links.push(Triple {
                s: n_pos.to_string(),
                p: "hasPort".to_string(),
                o: next_pos.to_string(),
            }); 
            next_pos += 1;
        }
        
        if device.chassis.rack != "" {
            nodes.push(device.chassis.rack);
            links.push(Triple {
                s: n_pos.to_string(),
                p: "hasChassis".to_string(),
                o: next_pos.to_string(),
            }); 
            next_pos += 1;
        }
    }
    println!("{:?}", nodes);

    KGraph {
        nodes,
        links,
    }
}
