/*
:- use_module(library(rdf)).
:- use_module(library(semweb/rdf11)).
:- use_module(library(semweb/rdf_prefixes), [ (rdf_meta)/1, op(_,_,rdf_meta) ]).
:- use_module(library(semweb/rdfs)) .
:- use_module(library(semweb/turtle)) .
:- use_module(library(semweb/rdf_portray)) .
:- use_module(library(uuid)).
:- use_module(library(increval)).

:- rdf_register_prefix(kpi, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/intent/kpi#').
:- rdf_register_prefix(cat, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/service#').
:- rdf_register_prefix(im, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/intent/intent#').
:- rdf_register_prefix(icm, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/intent/icm#').
:- rdf_register_prefix(in, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/customer_intent#').
:- rdf_register_prefix(sv, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/service#').
:- rdf_register_prefix(nm, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/network/network#').
:- rdf_register_prefix(ln, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/live_network#').
:- rdf_register_prefix(example, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/simple#').
*/

/*
:- dynamic link/2.

link(a, b).
link(b, c).

connected(X, Y) :-
	link(X, Y).

try(X, Y) :-
	connected(X, Y);	
	asserta(link(c, d)),
	false.
*/
:- table animal/1 as monotonic.
:- dynamic amphibian/1 as monotonic.
:- dynamic mammal/1 as monotonic.
:- dynamic reptile/1 as monotonic.
:- dynamic dog/1 as monotonic.
:- dynamic cat/1 as monotonic.

animal(X) :- mammal(X).
animal(X) :- amphibian(X).
animal(X) :- reptile(X).
 
mammal(X) :- dog(X).
mammal(X) :- cat(X).
amphibian(X) :- frog(X).
reptile(X) :- serpent(X).

dog(lassie).
dog(page).

cat(koko).
cat(skye).

frog(kermit).

serpent(monty).
