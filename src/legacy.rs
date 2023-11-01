

pub fn insert_data<'a>(ctx: &Context<'a, ActivatedEngine<'a>>, triples: Vec<Triple>) {
    for mut i in triples {
        squash_iri(&mut i.o);
        match i.p.as_str() {
            "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>" => {
                if i.o.as_str() != "NamedIndividual" {
                    println!("\nIndividual");
                    println!("Object: {}", i.o);

                    // No touchy...
                    let winner = Atom::new("Winner");
                    let functor = Functor::new("Hello", 1);
                    let test_winner = ctx.new_term_ref();
                    //let test_winner = term! {ctx: Hello(#&winner)}.unwrap();

                    //println!("{:?}", test_winner.term_type());

                    println!("{:?}", test_winner.put(&functor));
                    println!("{:?}", test_winner.put(&winner));


                    //let hello = pred!(Hello/1);
                    
                    //let hello = ctx.new_term_ref();
                    //let winner = ctx.new_term_ref();
                    //hello.unify(Functor::new(i.o.as_str(), 1));
                    //let winner = ctx.new_term_ref();
                    //winner.unify("Winner".into_atom());
                    /*
                    let f = Functor::new("Hello", 1);
                    let a = Atom::new("Winner");
                    println!("{:?}", hello.put(&f));
                    println!("{:?}", hello.put(&a));
                    println!("{:?}", hello.unify(&winner));

                    
                    let test_winner = ctx.new_term_ref();
                    println!("{:?}", test_winner.unify(&hello));
                    */

                        

                    /*
                    unsafe {
                        let fli_functor_ptr = Functor::new(Atom::new("Hello"), 1);
                        //let fli_functor_name = CString::new("Hello").unwrap().into_raw();
                        //let fli_functor_name = Atom::new("Hello");
                        //println!("FLI_FUNCTOR_NAME: {:?}", fli_functor_name.atom_ptr());
                        //let fli_winner_ptr = CString::new("Winner").unwrap().into_raw();
                        //println!("FLI_WINNER_PTR: {:?}", fli_winner_ptr);
                        //let fli_functor_ptr = PL_new_functor(fli_functor_name.atom_ptr(), 1 as i32); 
                        //let f = Functor::wrap(fli_functor_ptr);
                        //println!("F: {:?}", f.name().to_string());
                        println!("FLI_FUNCTOR_PTR: {:?}", fli_functor_ptr);

                        //ATOM
                        //let fli_winner = PL_new_atom(fli_winner_ptr);
                        //println!("FLI_WINNER: {:?}", fli_winner);
                        let fli_winner = ctx.new_term_ref();
                        let fli_winner_atom = Atom::new("Winner");
                        fli_winner.put(&fli_winner_atom).unwrap();


                        //let fli_func_out = PL_new_functor(PL_new_atom(fli_functor_name), 1 as i32); 
                        //println!("FLI_FUNC_OUT {:?}", fli_func_out);
                        //let fli_out = PL_new_term_ref();
                        let fli_out = ctx.new_term_ref();                        

                        let fli_compound = PL_cons_functor(fli_out.term_ptr(), fli_functor_ptr.functor_ptr(), fli_winner.term_ptr());
                        println!("FLI_COMPOUND: {:?}", fli_compound);
                        println!("Moment of truth: {:?}", fli_out.term_type());

                        let out: Vec<Term> = ctx.compound_terms_vec(&fli_out).unwrap();
                        for i in out {
                            let s = i.get::<String>(); 
                            match s {
                                Ok(o) => println!("{}", o),
                                Err(err) => println!("{}", err.to_string()),
                            }
                        }

                        //let comp_ptr: *mut Term = fli_compound as *mut Term;
                        //let term = comp_ptr.read();
                        //let new_term = Box::from_raw(fli_compound as *mut Term);
                    }
                    */

                    //test_winner.put(&hello).unwrap();
                    //test_winner.put(&winner).unwrap();
                     
                    //println!("Test Winner: {:?}", test_winner.term_type());
                    let assertz = pred!(assertz/1);
                    let set_fact = ctx.call_once(assertz, [&test_winner]);
                    match set_fact {
                        Ok(..) => println!("OKAY"),
                        Err(err) => println!("NOT OKAY\nErr: {}", err.to_string()),
                    }

                    // Can't detect new fact for some reason, until out of scope...?
                    //
                    let test_hello = term! {ctx: Q}.unwrap();
                    let test_fact = ctx.open(pred!{Hello/1}, [&test_hello]);
                    while test_fact.next_solution() == Ok(true) {
                        println!("{:?}", translate_output(&test_hello, ctx));
                    }
                    test_fact.cut();
                }
            },

            _ => {
                println!("\nRelation");
                println!("Relation: {}", i.p);
                println!("Object: {}", i.o);
            },
        }
    }    
}

// In case the ab
fn class_as_term(v: String) {
    
}

fn relation_as_term(v: String) {
        
}
