%------------------------------------------------------------------------------
% File     : GRP001-2 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Group Theory
% Problem  : X^2 = identity => commutativity
% Version  : [MOW76] (equality) axioms : Augmented.
% English  : If the square of every element is the identity, the system
%            is commutative.

% Refs     : [MOW76] McCharen et al. (1976), Problems and Experiments for a
%          : [LO85]  Lusk & Overbeek (1985), Reasoning about Equality
%          : [LW92]  Lusk & Wos (1992), Benchmark Problems in Which Equalit
% Source   : [ANL]
% Names    : GP1 [MOW76]
%          : Problem 1 [LO85]
%          : GT1 [LW92]
%          : xsquared.ver2.in [ANL]

% Status   : Unsatisfiable
% Rating   : 0.12 v6.2.0, 0.14 v6.1.0, 0.06 v6.0.0, 0.14 v5.5.0, 0.11 v5.4.0, 0.00 v5.1.0, 0.07 v4.1.0, 0.09 v4.0.1, 0.07 v4.0.0, 0.08 v3.7.0, 0.00 v2.2.1, 0.22 v2.2.0, 0.29 v2.1.0, 0.25 v2.0.0
% Syntax   : Number of clauses     :    8 (   0 non-Horn;   8 unit;   2 RR)
%            Number of atoms       :    8 (   8 equality)
%            Maximal clause size   :    1 (   1 average)
%            Number of predicates  :    1 (   0 propositional; 2-2 arity)
%            Number of functors    :    6 (   4 constant; 0-2 arity)
%            Number of variables   :    8 (   0 singleton)
%            Maximal term depth    :    3 (   2 average)
% SPC      : CNF_UNS_RFO_PEQ_UEQ

% Comments :
%------------------------------------------------------------------------------
%----Include equality group theory axioms
include('examples/GRP004-0.ax').
%------------------------------------------------------------------------------
%----Redundant two axioms
cnf(right_identity,axiom,
    ( multiply(X,identity) = X )).

cnf(right_inverse,axiom,
    ( multiply(X,inverse(X)) = identity )).

cnf(squareness,hypothesis,
    ( multiply(X,X) = identity )).

cnf(a_times_b_is_c,hypothesis,
    ( multiply(a,b) = c )).

cnf(prove_b_times_a_is_c,negated_conjecture,
    (  multiply(b,a) != c )).

%------------------------------------------------------------------------------
