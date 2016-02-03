%--------------------------------------------------------------------------
% File     : CAT001-3 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Category Theory
% Problem  : XY monomorphism => Y monomorphism
% Version  : [Sco79] axioms : Reduced > Complete.
% English  : If xy is a monomorphism, then y is a monomorphism.

% Refs     : [Sco79] Scott (1979), Identity and Existence in Intuitionist L
% Source   : [ANL]
% Names    : p1.ver3.in [ANL]

% Status   : Unsatisfiable
% Rating   : 0.09 v6.2.0, 0.30 v6.1.0, 0.36 v6.0.0, 0.30 v5.5.0, 0.35 v5.4.0, 0.30 v5.3.0, 0.33 v5.2.0, 0.25 v5.1.0, 0.24 v5.0.0, 0.14 v4.1.0, 0.23 v4.0.1, 0.36 v4.0.0, 0.18 v3.7.0, 0.00 v3.3.0, 0.07 v3.2.0, 0.08 v3.1.0, 0.09 v2.7.0, 0.17 v2.6.0, 0.00 v2.5.0, 0.25 v2.4.0, 0.11 v2.2.1, 0.22 v2.2.0, 0.11 v2.1.0, 0.33 v2.0.0
% Syntax   : Number of clauses     :   22 (   2 non-Horn;   7 unit;  17 RR)
%            Number of atoms       :   44 (  20 equality)
%            Maximal clause size   :    4 (   2 average)
%            Number of predicates  :    3 (   0 propositional; 1-2 arity)
%            Number of functors    :    8 (   4 constant; 0-2 arity)
%            Number of variables   :   34 (   4 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_UNS_RFO_SEQ_NHN

% Comments : Axioms simplified by Art Quaife.
%--------------------------------------------------------------------------
%----Include Scott's axioms for category theory
include('examples/CAT003-0.ax').
%--------------------------------------------------------------------------
cnf(assume_ab_exists,hypothesis,
    ( there_exists(compose(a,b)) )).

cnf(monomorphism,hypothesis,
    ( compose(compose(a,b),X) != Y
    | compose(compose(a,b),Z) != Y
    | X = Z )).

cnf(assume_bh_exists,hypothesis,
    ( there_exists(compose(b,h)) )).

cnf(bh_equals_bg,hypothesis,
    ( compose(b,h) = compose(b,g) )).

cnf(prove_h_equals_g,negated_conjecture,
    (  h != g )).

%--------------------------------------------------------------------------
