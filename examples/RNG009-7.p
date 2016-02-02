%--------------------------------------------------------------------------
% File     : RNG009-7 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Ring Theory
% Problem  : If X*X*X = X then the ring is commutative
% Version  : [LW91] (equality) axioms.
% English  : Given a ring in which for all x, x * x * x = x, prove that
%            for all x and y, x * y = y * x.

% Refs     : [LO85]  Lusk & Overbeek (1985), Reasoning about Equality
%          : [LW91]  Lusk & Wos (1991), Benchmark Problems in Which Equalit
% Source   : [LW91]
% Names    : Problem 6 [LO85]
%          : RT2 [LW91]

% Status   : Unsatisfiable
% Rating   : 0.59 v6.2.0, 0.64 v6.1.0, 0.56 v6.0.0, 0.71 v5.5.0, 0.68 v5.4.0, 0.67 v5.2.0, 0.64 v5.1.0, 0.67 v5.0.0, 0.64 v4.0.0, 0.62 v3.7.0, 0.56 v3.4.0, 0.62 v3.3.0, 0.50 v3.1.0, 0.33 v2.7.0, 0.36 v2.6.0, 0.17 v2.5.0, 0.25 v2.4.0, 0.00 v2.2.1, 0.56 v2.2.0, 0.71 v2.1.0, 1.00 v2.0.0
% Syntax   : Number of clauses     :   12 (   0 non-Horn;  12 unit;   2 RR)
%            Number of atoms       :   12 (  12 equality)
%            Maximal clause size   :    1 (   1 average)
%            Number of predicates  :    1 (   0 propositional; 2-2 arity)
%            Number of functors    :    7 (   4 constant; 0-2 arity)
%            Number of variables   :   19 (   0 singleton)
%            Maximal term depth    :    3 (   2 average)
% SPC      : CNF_UNS_RFO_PEQ_UEQ

% Comments :
%--------------------------------------------------------------------------
%----Include ring theory axioms
%----mAarnos: Modified to deal with the new location of the file.
include('examples/RNG005-0.ax').
%--------------------------------------------------------------------------
cnf(x_cubed_is_x,hypothesis,
    ( multiply(X,multiply(X,X)) = X )).

cnf(a_times_b_is_c,negated_conjecture,
    ( multiply(a,b) = c )).

cnf(prove_commutativity,negated_conjecture,
    (  multiply(b,a) != c )).

%--------------------------------------------------------------------------
