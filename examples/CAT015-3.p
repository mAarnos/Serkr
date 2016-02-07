%--------------------------------------------------------------------------
% File     : CAT015-3 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Category Theory
% Problem  : Prove something exists
% Version  : [Sco79] axioms : Reduced & Augmented > Complete.
% English  : Can anything be proven to exist, directly from the axioms?
%            Using all the kinds of resolution steps possible, no.

% Refs     : [Sco79] Scott (1979), Identity and Existence in Intuitionist L
% Source   : [ANL]
% Names    : p15.related.in [ANL]

% Status   : Satisfiable
% Rating   : 0.12 v6.2.0, 0.10 v6.1.0, 0.00 v5.4.0, 0.10 v5.3.0, 0.11 v5.2.0, 0.10 v5.0.0, 0.11 v4.1.0, 0.00 v2.5.0, 0.33 v2.2.1, 0.25 v2.2.0, 0.67 v2.1.0, 0.00 v2.0.0
% Syntax   : Number of clauses     :   19 (   3 non-Horn;   4 unit;  13 RR)
%            Number of atoms       :   41 (  16 equality)
%            Maximal clause size   :    4 (   2 average)
%            Number of predicates  :    3 (   0 propositional; 1-2 arity)
%            Number of functors    :    4 (   0 constant; 1-2 arity)
%            Number of variables   :   34 (   5 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_SAT_RFO_EQU_NUE

% Comments : Axioms simplified by Art Quaife.
%          : Using Quaife's axiom ((x=y) | E(x) | E(y).).
%--------------------------------------------------------------------------
%----Include Scott's axioms for category theory
include('examples/CAT003-0.ax').
%--------------------------------------------------------------------------
%----Quaife has this written: (this looks really weird, but results from
%----having = here stand for equivalence, and it is a strange fact from
%----Scott's conception that all non-existent things are equivalent.
cnf(equal_things_exist,axiom,
    ( X = Y
    | there_exists(X)
    | there_exists(Y) )).

%----Denial that anything at all exists
cnf(prove_something_exists,negated_conjecture,
    ( ~ there_exists(X) )).

%----The ANL group use these lemmas as demodulators
%input_clause(name,hypothesis,
%    [++equal(domain(domain(X)),domain(X))]).
%input_clause(name,hypothesis,
%    [++equal(codomain(domain(X)),domain(X))]).
%input_clause(name,hypothesis,
%    [++equal(domain(codomain(X)),codomain(X))]).
%input_clause(name,hypothesis,
%    [++equal(codomain(codomain(X)),codomain(X))]).
%input_clause(name,hypothesis,
%    [++equal(domain(compose(X,Y)),domain(Y))]).
%input_clause(name,hypothesis,
%    [++equal(codomain(compose(X,Y)),codomain(X))]).
%input_clause(name,hypothesis,
%    [++equal(compose(domain(X),domain(X)),domain(X))]).
%input_clause(name,hypothesis,
%    [++equal(compose(codomain(X),codomain(X)),codomain(X))]).
%--------------------------------------------------------------------------
