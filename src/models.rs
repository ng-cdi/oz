use sophia::graph::inmem::FastGraph;
use sophia::dataset::inmem::FastDataset;
use sophia::parser::turtle;
use sophia::triple::stream::TripleSource;
use std::fs::File;
use std::io::BufReader;

pub fn load_default() {
    let icm_f = File::open("ontologies/tmf/icm.ttl").unwrap();
    let network_model_f = File::open("ontologies/network/network.ttl").unwrap();
    let intent_model_f = File::open("ontologies/intent/intent.ttl").unwrap();
    let service_f = File::open("ontologies/examples/tmf_example/service.ttl").unwrap();

    let _ontology: FastDataset = FastDataset::default();

    let _icm: FastGraph = turtle::parse_bufread(BufReader::new(&icm_f)).collect_triples().unwrap();
    let _network: FastGraph = turtle::parse_bufread(BufReader::new(&network_model_f)).collect_triples().unwrap();
    let _intent: FastGraph =  turtle::parse_bufread(BufReader::new(&intent_model_f)).collect_triples().unwrap();
    let _service: FastGraph = turtle::parse_bufread(BufReader::new(&service_f)).collect_triples().unwrap();
}
