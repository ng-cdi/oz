use sophia::dataset::MutableDataset;
use sophia::graph::inmem::FastGraph;
use sophia::dataset::inmem::FastDataset;
use sophia::parser::turtle;
use sophia::ns::Namespace;
use sophia::triple::stream::TripleSource;
use std::fs::File;
use std::io::BufReader;

pub fn load_default() {
    let mut icm_f = File::open("ontologies/tmf/icm.ttl").unwrap();
    let mut network_model_f = File::open("ontologies/network/network.ttl").unwrap();
    let mut intent_model_f = File::open("ontologies/intent/intent.ttl").unwrap();
    let mut service_f = File::open("ontologies/examples/tmf_example/service.ttl").unwrap();

    let ontology: FastDataset = FastDataset::default();

    let icm: FastGraph = turtle::parse_bufread(BufReader::new(&icm_f)).collect_triples().unwrap();
    let network: FastGraph = turtle::parse_bufread(BufReader::new(&network_model_f)).collect_triples().unwrap();
    let intent: FastGraph =  turtle::parse_bufread(BufReader::new(&intent_model_f)).collect_triples().unwrap();
    let service: FastGraph = turtle::parse_bufread(BufReader::new(&service_f)).collect_triples().unwrap();
}
