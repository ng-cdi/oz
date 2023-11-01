use std::{path::Path, io::BufReader, fs::File};
use sophia::{graph::inmem::FastGraph, parser::turtle, triple::stream::TripleSource};

use crate::common;

pub fn generate_ontology(path: &Path) {
    match common::parse_turtle(path) {
        Ok(ont) => {
               
        },
        Err(err) => panic!("{:?}", err),
    }
}
