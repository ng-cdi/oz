:- use_module(library(rdf)).
/**:- use_module(library(semweb/rdf_db)).*/
:- use_module(library(semweb/rdf11)).
:- use_module(library(semweb/rdf_prefixes),
              [ (rdf_meta)/1, op(_,_,rdf_meta)
              ]).
:- use_module(library(semweb/rdfs)).
/**:- rdf_load(library(semweb/rdfs)).*/
:- use_module(library(semweb/turtle)).
:- use_module(library(semweb/rdf_portray)).
/**:- use_module(library(thea2/owl2_io)).
:- use_module(library(thea2/owl2_model)).*/
/**:- use_module(library(http/http_ssl_plugin)).*/

/**
:- rdf_register_prefix(owl, 'http://www.w3.org/2002/07/owl#').
:- rdf_register_prefix(rdf, 'http://www.w3.org/1999/02/22-rdf-syntax-ns#').
:- rdf_register_prefix(xml, 'http://www.w3.org/XML/1998/namespace').
:- rdf_register_prefix(xsd, 'http://www.w3.org/2001/XMLSchema#').
:- rdf_register_prefix(rdfs, 'http://www.w3.org/2000/01/rdf-schema#').*/
:- rdf_register_prefix(me, 'http://www.semanticweb.org/paul/ontologies/2023/1/facts#').

:- rdf_meta
	person(o),
	hasMother(r, o),
	hasFather(r, o),
	hasBrother(r, o),
	hasSister(r, o),
	hasSibling(r, o),
	hasSon(r, o),
	hasDaughter(r, o),
	hasUncle(r, o),
	hasAunt(r, o),
	hasParent(r, o),
	hasChild(r, o),
	hasGrandFather(r, o),
	hasGrandMother(r, o),
	hasGrandParent(r, o),
	hasCousin(r, o).

go :-
	read_turtle,
	example_query,
	infer_family,
	infer_family_two,
	infer_family_three,
	save_turtle.

start :-
	read_turtle.

read_turtle :-
	rdf_load('./family.ttl', [format(turtle), graph('http://www.semanticweb.org/paul/ontologies/2023/1/facts#'), register_namespaces(true)]).

/** untested **/
save_turtle :-
	rdf_save_turtle('./newfamily.ttl',
					[only_known_prefixes(true),
					a(true)]).

/** untested (rdf_meta) **/
/** https://www.swi-prolog.org/howto/UseRdfMeta.html **/
/**:- rdf_meta
	person(r).

person(ex:bob). **/

/** untested **/
/**
person(X) :-
	rdfs_individual_of(X, me:'Person').*/
/**
example_query :-
	rdf(me:'Claire', me:'hasFather', F).

example_query_s :-
	rdf(me:'Claire', me:'hasFather', F),
	rdf(F, me:'hasSon', S),
	rdf(E, me:'hasFather', S).
*/

/** RULES **/
/**rdf_assert(S, me:'hasMother', O) :- 
	rdf(S, me:'hasBrother', B),
	rdf(B, me:'hasMother', O).*/

/**
:- (rdf(S, me:'hasBrother', B),
	rdf(B, me:'hasMother', O) ->
	rdf_assert(S, me:'hasMother', O)).

:-	(rdf(B, me:'hasUncle', O),
	rdf(S, me:'hasBrother', B) ->
	rdf_assert(S, me:'hasUncle', O)).

:-	(rdf(S, me:'hasMother', M),
	rdf(M, me:'hasBrother', O) ->
	rdf_assert(S, me:'hasUncle', O)).

:-	(rdf(S, me:'hasMother', M),
	rdf(M, me:'hasFather', F),
	rdf(F, me:'hasSon', O),
	O \= M ->
	rdf_assert(F, me:'hasSon', O)).
*/

person(S) :-
	rdfs_individual_of(S, me:'Person').

hasMother(S, O) :-
	rdf_has(S, me:hasMother, O).

hasFather(S, O) :-
	rdf_has(S, me:hasFather, O).

hasBrother(S, O) :-
	rdf_has(S, me:hasBrother, O);
	rdf_has(O, me:hasBrother, S);
	(hasParent(S, P),hasSon(P, O)).

hasSister(S, O) :-
	rdf_has(S, me:hasSister, O);
	rdf_has(O, me:hasSister, S);
	(hasParent(S, P),hasDaughter(P, O)).

hasSibling(S, O) :-
	hasBrother(S, O);
	hasSister(S, O).

hasSon(S, O) :-
	rdf_has(S, me:hasSon, O).

hasDaughter(S, O) :-
	rdf_has(S, me:hasDaughter, O).

hasUncle(S, O) :-
	rdf_has(S, me:hasUncle, O);
	(hasParent(S, P),hasBrother(P, O)).

hasAunt(S, O) :-
	rdf_has(S, me:hasAunt, O);
	(hasParent(S, P),hasSister(P, O)).

hasParent(S, O) :-
	hasFather(S, O);
	hasMother(S, O).

hasChild(S, O) :-
	hasSon(S, O);
	hasDaughter(S, O);
	hasParent(O, S).

hasGrandChild(S, O) :-
	hasGrandParent(O, S);
	(hasChild(S, P),
	hasChild(P, O)).

hasGrandFather(S, O) :-
	hasParent(S, P),
	hasFather(P, O).

hasGrandMother(S, O) :-
	hasParent(S, P),
	hasMother(P, O).

hasGrandParent(S, O) :-
	hasGrandChild(O, S);
	(hasGrandMother(S, O);
	hasGrandFather(S, O)).

hasCousin(S, O) :-
	hasParent(S, P),
	hasSibling(P, BS),
	hasChild(BS, O).

infer_family :-
	(rdf(S, me:hasBrother, B),
		rdf(B, me:hasMother, O) ->
	rdf_assert(S, me:hasMother, O)).

infer_family_two :-
	(rdf(B, me:hasUncle, O),
		rdf(S, me:hasBrother, B) ->
	rdf_assert(S, me:hasUncle, O)).

infer_family_three :-
	(rdf(S, me:hasMother, M),
		rdf(M, me:hasBrother, O) ->
	rdf_assert(S, me:hasUncle, O)).

infer_family_four :-
	(rdf(S, me:hasFather, F),
		rdf(F, me:hasSon, O),
		O \= S ->
	rdf_assert(S, me:hasBrother, O)).
