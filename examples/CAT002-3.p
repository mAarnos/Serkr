%--------------------------------------------------------------------------
% File     : CAT002-3 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Category Theory
% Problem  : X and Y monomorphisms, XY well-defined => XY monomorphism
% Version  : [Sco79] axioms : Reduced > Complete.
% English  : If x and y are monomorphisms and xy is well-defined then
%            xy is a monomorphism.

% Refs     : [Sco79] Scott (1979), Identity and Existence in Intuitionist L
% Source   : [ANL]
% Names    : p2.ver3.in [ANL]

% Status   : Unsatisfiable
% Rating   : 0.00 v6.2.0, 0.10 v6.1.0, 0.14 v6.0.0, 0.10 v5.5.0, 0.35 v5.3.0, 0.28 v5.2.0, 0.25 v5.1.0, 0.18 v5.0.0, 0.14 v4.1.0, 0.15 v4.0.1, 0.27 v4.0.0, 0.18 v3.7.0, 0.00 v3.3.0, 0.14 v3.2.0, 0.15 v3.1.0, 0.18 v2.7.0, 0.08 v2.6.0, 0.00 v2.5.0, 0.25 v2.4.0, 0.33 v2.3.0, 0.11 v2.2.1, 0.44 v2.2.0, 0.22 v2.1.0, 0.44 v2.0.0
% Syntax   : Number of clauses     :   23 (   2 non-Horn;   7 unit;  18 RR)
%            Number of atoms       :   47 (  23 equality)
%            Maximal clause size   :    4 (   2 average)
%            Number of predicates  :    3 (   0 propositional; 1-2 arity)
%            Number of functors    :    8 (   4 constant; 0-2 arity)
%            Number of variables   :   37 (   4 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_UNS_RFO_SEQ_NHN

% Comments : Axioms simplified by Art Quaife.
%--------------------------------------------------------------------------
%----Include Scott's axioms for category theory
include('examples/CAT003-0.ax').
%--------------------------------------------------------------------------
cnf(assume_ab_exists,hypothesis,
    ( there_exists(compose(a,b)) )).

cnf(cancellation_for_compose1,hypothesis,
    ( compose(a,X) != Y
    | compose(a,Z) != Y
    | X = Z )).

cnf(cancellation_for_compose2,hypothesis,
    ( compose(b,X) != Y
    | compose(b,Z) != Y
    | X = Z )).

cnf(assume_h_exists,hypothesis,
    ( there_exists(h) )).

cnf(ab_h_equals_ab_g,hypothesis,
    ( compose(compose(a,b),h) = compose(compose(a,b),g) )).

cnf(prove_g_equals_h,negated_conjecture,
    (  g != h )).

%--------------------------------------------------------------------------
