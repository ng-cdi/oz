#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use actix_web::web::Data;
use sophia::graph::inmem::FastGraph;
use sophia::parser::turtle;
use sophia::triple::stream::TripleSource;
use swipl::prelude::*;
use common::{AppState, turtle_to_xml, Triple, Config};
use log::info;
use clap::Parser;

use crate::api::sessions::SessionManager;
use crate::common::{Catalog, to_rdf, OzIntent};

mod prolog;
mod api;
mod stack;
mod common;
mod srims;
mod models;
mod monitor;
mod demo;
mod onos;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Setup full system
    #[arg(short, long)]
    setup: bool,

    /// Clean system
    #[arg(short, long)]
    clean: bool,

    /// Run whatever test
    #[arg(short, long)]
    test: bool,

    /// Add new slice to create test case
    #[arg(short, long)]
    next: bool,

    /// Test Intent
    #[arg(short, long)]
    intent: Option<String>,
    
    /// Add Subscriber
    #[arg(short, long)]
    addsub: Option<u8>,

}

//const GRAPHDB: &str = "http://localhost:7200";

fn srims_test() {
    let mut data = srims::read_srims();
    let devices = srims::srims_to_rdf(&mut data);
    println!("{devices:#?}");  
}

fn convert_test() {
    let _files: Vec<File> = Vec::new();   
    let mut icm = File::open("../ontologies/tmf/icm.ttl").unwrap();
    let mut network_model = File::open("../ontologies/network/network.ttl").unwrap();
    let mut intent_model = File::open("../ontologies/intent/intent.ttl").unwrap();
    let mut service = File::open("../ontologies/examples/tmf_example/service.ttl").unwrap();
    let mut cust_int = File::open("../ontologies/examples/tmf_example/customer_intent.ttl").unwrap();
    let mut live_net = File::open("../ontologies/examples/tmf_example/live_network.ttl").unwrap();


    turtle_to_xml(&mut icm, "../ontologies/xml/icm.xml");
    turtle_to_xml(&mut network_model, "../ontologies/xml/nm.xml");
    turtle_to_xml(&mut intent_model, "../ontologies/xml/im.xml");
    turtle_to_xml(&mut service, "../ontologies/xml/service.xml");
    turtle_to_xml(&mut live_net, "../ontologies/xml/live_network.xml");
    turtle_to_xml(&mut cust_int, "../ontologies/xml/customer_intent.xml");
}

/// Plan 
fn plan(data: Data<RwLock<AppState>>) -> bool {
    let data = data.read().unwrap();
    let cfg = &data.config;
    let design_fix = match cfg.oz.kind.as_str() {
        "live" => {
            info!("wow...");
            return true;
        },
        e => {
            info!("plan: {}", e);                        
            prolog::fix(&data)
        },
    };
    // Run another validation to see if all are fixed?
    // If yes, start translation into imperative steps
    // If no... idk...
    let mut all_clear = true;
    let results = prolog::validate_design(&data, design_fix.clone());
    for i in results.iter() {
        info!("{}: {}", i.0, i.1);
        if i.1 == "\"false\"".to_string() {
            all_clear = false;
        }
    }

    if all_clear == true {
        //figure out delta for OSM use-case
        //prolog::apply_changes(data, design_fix);
        
        return true
    }
    false
}

/// Monitor & Validate
fn validation_loop(data: Data<RwLock<AppState>>) {
    // store last and compare before pushing to websocket
    loop {
        let data_read = data.read().unwrap();
        let results = prolog::validate_intent(&data_read);
        drop(data_read);
        {
            let mut data = data.write().unwrap();
            let service_lock = &mut data.services;
            let service = service_lock.get_mut(0).unwrap();
            let intent = &mut service.intent;
            intent.status = results.clone();
        }
        let mut fix = false;
        for i in results.iter() {
            info!("{}: {}", i.0, i.1);
            if i.1 == "\"false\"".to_string() {
                fix = true;
            }
        }
        if fix == true {
            plan(data.clone());
        }
        thread::sleep(Duration::from_secs(10));
    }
}

#[allow(unused_assignments)]
async fn first_setup(cfg: Config) -> AppState {

    let mut live_network: Vec<Triple> = Vec::new();
    let mut demo_intent: Vec<Triple> = Vec::new();
    let mut catalog: Catalog = Catalog::new();

    info!("Oz: Starting Prolog Engine...");
    let engine = Engine::new();
    let ctx = prolog::load_prolog(&engine);
    //let ctx = prolog::load_prolog_oz(&engine);

    // First time setup
    info!("Oz: Starting first time setup...");

    monitor::get_all(&cfg.prometheus).await;

    // Update local data from OSM
    match cfg.oz.kind.as_str() {
        "live" => {
            live_network = common::update_data(&cfg).await;
        },
        e => {
            //live_network = common::dummy_data(format!("../ontologies/examples/oz/{}.ttl", e).as_str());
            live_network = common::dummy_data(format!("../ontologies/examples/infrastructure/{}.ttl", e).as_str());
        },
    }

    // Get customer intent from file
    // TODO: Only once uploaded
    let cust_int = File::open("../ontologies/examples/tmf_example/customer_intent.ttl").unwrap();
    let graph: FastGraph = turtle::parse_bufread(BufReader::new(cust_int)).collect_triples().unwrap();
    //let graph = ontology_from_file("ontologies/examples/tmf_example/customer_intent.ttl").unwrap();
    //demo_intent.append(&mut common::new_to_rdf(graph).unwrap());
    demo_intent.append(&mut common::to_rdf(graph).unwrap());

    // Add Catalog from local service descriptions
    let oz_service = File::open("../ontologies/examples/tmf_example/service.ttl").unwrap();
    let service_graph: FastGraph = turtle::parse_bufread(BufReader::new(oz_service)).collect_triples().unwrap();
    let service_triples = to_rdf(service_graph).unwrap();
    // Oxigraph implementation
    //let service_triples = common::new_to_rdf(ontology_from_file("ontologies/examples/tmf_example/customer_intent.ttl").unwrap()).unwrap();
    
    let service_name = "http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/service.ttl".to_string();
    catalog.insert(service_name, service_triples);
    /*
    let cleanup = ctx.call_once(pred!(PL_cleanup/1), [&status]);
    match cleanup {
        Ok(_) => {
            info!("Oz: Prolog Engine Cleaned!");
        },
        Err(e) => {
            error!("Oz: Prolog Engine failed to clean: {}", e);
            println!("{:?}", prolog::term_to_json(&ctx, &status));
        },
    }
    */
    drop(ctx);
    
    let appstate = AppState{
        services: vec![common::Service {
            id: "DEMO".to_string(),
            name: "DEMO".to_string(),
            intent_clean: demo_intent,
            intent: OzIntent {
                status: Vec::new(),
                intent: Vec::new(),
            },
            service_definitions: Vec::new(),
            osm_id: "21dd7ea7-6b28-4061-aa3f-ee9722bd4822".to_string(),
        }],        
        catalog,
        network: live_network,
        config: cfg,
        session_manager: Arc::new(SessionManager{ sessions: RwLock::new(HashMap::new())}),
    };

    appstate
} 

fn main() {

    //holmes::ontology::generate_prolog("../ontologies/network/network.ttl", "./prolog/ont_network.pro");
    //holmes::ontology::generate_prolog("../ontologies/intent/intent.ttl", "./prolog/ont_intent.pro");
    
    let args = Args::parse();

    env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Get clients
    let _cfg = common::read_config();


    if args.clean {
        rt.block_on(demo::cleanup());
    } else if args.setup {
        rt.block_on(demo::start_demo());
    } else if args.test {
        rt.block_on(demo::test());
    } else if args.next {
        rt.block_on(demo::situation());
    } else if let Some(device) = args.intent {
        rt.block_on(onos::add_flow(device))
    } else if let Some(sub) = args.addsub {
        rt.block_on(demo::add_subscriber(sub));
    } 

    return;



    // Files manually edited...

    // First Startup
    //let aps = rt.block_on(first_setup(cfg));
    //let appstate = Data::new(RwLock::new(aps));
    
    /*
    let de = prolog::DomainExpert::new();
    de.load_prolog_all();
    let ctx = de.get_context();
    let dump_core_switches = pred!{coreSwitch/1};
    //let dump_core_switches = pred!{dump_core_switchesb/3};
    //let dump_core_switches = pred!{arnold/3};
    let stuff = de.query_extract(ctx, dump_core_switches);
    println!("{:?}", stuff);
    for i in stuff {
        println!("{:?}", i);
    }
    */

    // Start validation loop
  
    /*
    let asc = appstate.clone();
    thread::spawn(|| {
        validation_loop(asc);
    });
    */

    // Start api server
    /*
    info!("Oz: Server started!");
    rt.block_on(
        async {
            api::start_server(appstate).await.unwrap();
        }
    );
    */
}
