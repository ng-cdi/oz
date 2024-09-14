:- use_module(library(chr)).
:- chr_constraint p/2, q/2.
:- chr_constraint validMachine/3.

%p1 @ p <=> q.
%p2 @ p ==> q.
%p3 @ p,q <=> true.
%p4 @ p \ q <=> true.

%p1 @ p(a) <=> true | true.
%p2 @ p(X) <=> X=a | true.
%p3 @ p(X) <=> true | X=a.
%p4 @ p(X) <=> true , X = a | true.
%p5 @ p(X) <=> X = a , X = b | true.

%p1 @ p(X,Y), q(Z,Y) <=> q(X,Y).
%p2 @ q(Z,Y), p(X,Y) <=> q(X,Y).
%p3 @ p(X,Y), q(Z,Y) ==> q(X,Y).
%p4 @ p(X,Z)\ q(Z,Y) <=> q(X,Y).

% Scenario: We need several constraints to be guaranteed
% and several to be optional. 

redundancy @ validMachine(X, Y, E) ==> availability | redundantDeployableMachine(X, Y, E).
security @ validMachine(X, Y, E) ==> security | secureDeployableMachine(X, Y, E).
latency @ validMachine(X, Y, E) ==> latency | latentDeployableMachine(X, Y, E).
throughput @ validMachine(X, Y, E) ==> throughput | throughputDeployableMachine(X, Y, E).
buffer @ validMachine(X, Y, E) <=> true.

/*
validMachine(X, Y, E) :-
	X = a,
	Y = b,
	E = c.
*/

soundMachine(X, Y) :-
	deployableMachine(X, Y).

deployableMachine(X, Y) :-
	X = deployable,
	Y = machine.

redundantDeployableMachine(X, Y, E) :-
	X = redundant,
	Y = deployable, 
	E = machine,
	true.

secureDeployableMachine(X, Y, E) :-
	X = secure,
	Y = deployable,
	E = machine,
	true.

latentDeployableMachine(X, Y, E) :-
	true.

throughputDeployableMachine(X, Y, E) :-
	true.

availability :-
	true.

security :-
	true.

latency :- false.
throughput :- false.
