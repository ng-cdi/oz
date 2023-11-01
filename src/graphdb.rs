
//DON'T FORGET. CITE Champin in Paper!!
fn test_servers() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    let cfg = common::read_config();
    let ostack = rt.block_on(cfg.openstack.get_client());
    let hypervisors = stack::list_hypervisors(ostack);
    let body: Value = rt.block_on(hypervisors.json()).unwrap();

    println!("{:?}", body);

    // Create OSM Client (Home)
    let mut client = cfg.osm.get_client();
    client = osm::auth::get_token(client);

    query("SELECT * WHERE { ?s ?p ?o . } limit 100".to_string());

    match graphdb_login() {
        Err(e) => println!("Error: {:?}", e),
        Ok(o) => add_openstack(o, body),
    }
}


fn graphdb_login() -> Result<String, Error> {
    let req = Client::new();
    let res = req.post("http://localhost:7200/rest/login/admin")
                .header("X-GraphDB-Password", "password123")
                .send();

    match res {
        Ok(o) => {
            match o.headers().get("authorization") {
                Some(oo) => return Ok(String::from(oo.to_str().unwrap())),
                None => Err(Error::new("Could not get graphdb credentials")),
            }
        }
        _ => Err(Error::new("graphdb login invalid")),
    }
}

fn to_graphdb(token: String) {

    let mut f = File::open("res/intent/network_model.ttl").unwrap();  
    let mut ontology = String::new();
    
    // use read_err to check error
    let _read_err = f.read_to_string(&mut ontology).unwrap().to_string();
    let req = Client::new();    
    let res = req.post("http://localhost:7200/rest/repositories/network/import/server")
        .header("Content-Type", "application-json")
        .header("authorization", token.as_str())
        .json(&json!({
            "importSettings": {
                "data": ontology, 
            }, 
        }))
        .send();

    match res {
        Err(e) => println!("Error: {:?}", e),
        Ok(o) => {
            println!("Headers: {:?}", o.headers());
            println!("Body: {:?}", o.text());
        },
    }
}

fn query(sparql: String) {
    let sparql = percent_encoding::utf8_percent_encode(&sparql, percent_encoding::NON_ALPHANUMERIC);
    let req = Client::new();
    let res = req.get(format!("{GRAPHDB}/repositories/network?query={sparql}"))
        .header("Accept", "application/sparql-results+json")
        .send();

    match res {
        Err(e) => println!("Error: {:?}", e),
        Ok(o) => {
            println!("Headers: {:?}", o.headers());
            println!("Body: {:?}", o.text());
        },
    }
}

fn statement(sparql: String) {
    let sparql = percent_encoding::utf8_percent_encode(&sparql, percent_encoding::NON_ALPHANUMERIC);
    let req = Client::new();
    let res = req.post(format!("{GRAPHDB}/repositories/network/statements?update={sparql}"))
        .header("Accept", "application/sparql-results+json")
        .send();

    match res {
        Err(e) => println!("Error: {:?}", e),
        Ok(o) => {
            println!("Headers: {:?}", o.headers());
            println!("Body: {:?}", o.text());
        },
    }
}

fn add_openstack(token: String, json: Value) {
    let mut sparql = String::new();
    if json.is_object() {
        let hyps = json.as_object().unwrap().get("hypervisors").unwrap();
        for i in hyps.as_array().unwrap() {
            let h = i.as_object().unwrap();
            sparql.push_str(format!("
                PREFIX ex: <http://www.semanticweb.org/paul/ontologies/2023/0/network_model#>
                PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                PREFIX : <http://www.semanticweb.org/paul/ontologies/2023/0/converged_network#>
                BASE <http://www.semanticweb.org/paul/ontologies/2023/0/converged_network#>
                INSERT DATA {{
                    GRAPH <http://www.semanticweb.org/paul/ontologies/2023/0/converged_network> {{
                        :{} rdf:type ex:Hypervisor .
                    }}
                }}", h["hypervisor_hostname"]
            ).as_str());
        }
    }
    statement(sparql);
}

