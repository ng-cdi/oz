:- use_module(library(rdf)).
:- use_module(library(semweb/rdf11)).
:- use_module(library(semweb/rdf_prefixes), [ (rdf_meta)/1, op(_,_,rdf_meta) ]).
:- use_module(library(semweb/rdfs)) .
:- use_module(library(semweb/turtle)) .
:- use_module(library(semweb/rdf_portray)) .
:- use_module(library(uuid)).
:- use_module(library(increval)).
:- use_module(library(clpb)).
:- use_module(library(clpfd)).

:- rdf_register_prefix(kpi, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/intent/kpi#').
:- rdf_register_prefix(cat, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/service#').
:- rdf_register_prefix(im, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/intent/intent#').
:- rdf_register_prefix(icm, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/intent/icm#').
:- rdf_register_prefix(in, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/customer_intent#').
:- rdf_register_prefix(sv, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/service#').
:- rdf_register_prefix(nm, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/network/network#').
:- rdf_register_prefix(ln, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/live_network#').
:- rdf_register_prefix(example, 'http://raw.githubusercontent.com/Guilvareux/ontologies/main/examples/tmf_example/simple#').

/*
:- dynamic rewriteable/1.
rewriteable(X) :-
	writeln(X),
	writeln("Hello").

append_to_clause(Clause, A) :-
	nth_clause(Clause, 1, Ref), 
	clause(Head, Body, Ref), 
	retractall(Head), 
	asserta((Head :- Body, A)).

rewrite(Clause) :- 
	nth_clause(Clause, 1, Ref), 
	clause(Head, Body, Ref), 
	retractall(Head), 
	asserta((Head :- writeln("NO WAAAAY"), Body)).
pigeon(I, J, Rows, Cs) :-
        length(Rows, I), length(Row, J),
        maplist(same_length(Row), Rows),
        transpose(Rows, TRows),
        phrase((all_cards(Rows,[1]),all_cards(TRows,[0,1])), Cs).

all_cards([], _) --> [].
all_cards([Ls|Lss], Cs) --> [card(Cs,Ls)], all_cards(Lss, Cs).
pigeon(I, J, Rows, Cs) :-
        length(Rows, I), length(Row, J),
        maplist(same_length(Row), Rows),
        transpose(Rows, TRows),
        phrase((all_cards(Rows,[1]),all_cards(TRows,[0,1])), Cs).

all_cards([], _) --> [].
all_cards([Ls|Lss], Cs) --> [card(Cs,Ls)], all_cards(Lss, Cs).
*/

 
difference_append(OpenList1-Hole1,OpenList2-Hole2,OpenList1-Hole2):-
	Hole1=OpenList2.

:- X=[a,b,c|Ho]-Ho, difference_append(X,[d,e,f|Hole2]-Hole2,Ans-[]).
