%--------------------------------------------------------------------------
% File     : GRP012-4 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Group Theory
% Problem  : Inverse of products = Product of inverses
% Version  : [MOW76] (equality) axioms : Augmented.
% English  : The inverse of products equals the product of the inverse,
%            in opposite order

% Refs     : [MOW76] McCharen et al. (1976), Problems and Experiments for a
% Source   : [ANL]
% Names    : - [ANL]

% Status   : Unsatisfiable
% Rating   : 0.12 v6.2.0, 0.14 v6.1.0, 0.06 v6.0.0, 0.19 v5.5.0, 0.16 v5.4.0, 0.00 v5.1.0, 0.07 v5.0.0, 0.00 v2.2.1, 0.22 v2.2.0, 0.29 v2.1.0, 0.25 v2.0.0
% Syntax   : Number of clauses     :    6 (   0 non-Horn;   6 unit;   1 RR)
%            Number of atoms       :    6 (   6 equality)
%            Maximal clause size   :    1 (   1 average)
%            Number of predicates  :    1 (   0 propositional; 2-2 arity)
%            Number of functors    :    5 (   3 constant; 0-2 arity)
%            Number of variables   :    7 (   0 singleton)
%            Maximal term depth    :    3 (   2 average)
% SPC      : CNF_UNS_RFO_PEQ_UEQ

% Comments : In Lemmas.eq.clauses of [ANL]
%--------------------------------------------------------------------------
%----Include equality group theory axioms
include('examples/GRP004-0.ax').
%--------------------------------------------------------------------------
%----Redundant two axioms
cnf(right_identity,axiom,
    ( multiply(X,identity) = X )).

cnf(right_inverse,axiom,
    ( multiply(X,inverse(X)) = identity )).

cnf(prove_inverse_of_product_is_product_of_inverses,negated_conjecture,
    (  inverse(multiply(a,b)) != multiply(inverse(b),inverse(a)) )).

%--------------------------------------------------------------------------
