person(paul).
person(claire).
person(bob).
person(sean).
person(julian).

hasBrother(sean, paul).
hasFather(claire, bob).

hasMother(paul, claire).
hasMother(S,Y) :-
	hasBrother(S, Z),
	hasMother(Z, Y).

hasSon(bob, julian).

hasUncle(sean, julian).
hasUncle(S, O) :-
	hasBrother(S, B),
	hasUncle(B, O).

hasUncle(S, O) :-
	hasMother(S, M),
	hasBrother(M, O).

hasUncle(S, O) :-
	hasMother(S, M),
	hasFather(M, F),
	hasSon(F, O),
	O \= M.
