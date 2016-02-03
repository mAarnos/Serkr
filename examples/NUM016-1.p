%--------------------------------------------------------------------------
% File     : NUM016-1 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Number Theory
% Problem  : There exist infinitely many primes
% Version  : [LS74] axioms.
% English  :

% Refs     : [Luc68] Luckham (1968), Some Tree-paring Strategies for Theore
%          : [Cha70] Chang (1970), The Unit Proof and the Input Proof in Th
%          : [LS74]  Lawrence & Starkey (1974), Experimental Tests of Resol
%          : [WM76]  Wilson & Minker (1976), Resolution, Refinements, and S
% Source   : [SPRFN]
% Names    : Example 8b [Luc68]
%          : ls17 [LS74]
%          : Problem 17 [LS74]
%          : ls17 [WM76]

% Status   : Unsatisfiable
% Rating   : 0.14 v6.2.0, 0.00 v5.4.0, 0.10 v5.3.0, 0.00 v2.0.0
% Syntax   : Number of clauses     :   12 (   3 non-Horn;   4 unit;   7 RR)
%            Number of atoms       :   22 (   0 equality)
%            Maximal clause size   :    3 (   2 average)
%            Number of predicates  :    3 (   0 propositional; 1-2 arity)
%            Number of functors    :    3 (   1 constant; 0-1 arity)
%            Number of variables   :   16 (   0 singleton)
%            Maximal term depth    :    2 (   1 average)
% SPC      : CNF_UNS_RFO_NEQ_NHN

% Comments : These axioms are the same as in [Luc68]
%--------------------------------------------------------------------------
cnf(nothing_is_less_than_itself,axiom,
    ( ~ less(X,X) )).

cnf(numbers_are_different,axiom,
    ( ~ less(X,Y)
    | ~ less(Y,X) )).

cnf(everything_divides_itself,axiom,
    ( divides(X,X) )).

cnf(transitivity_of_divides,axiom,
    ( ~ divides(X,Y)
    | ~ divides(Y,Z)
    | divides(X,Z) )).

cnf(small_divides_large,axiom,
    ( ~ divides(X,Y)
    | ~ less(Y,X) )).

cnf(a_prime_is_less_than_the_next_one,axiom,
    ( less(X,factorial_plus_one(X)) )).

cnf(divisor_is_smaller,axiom,
    ( ~ divides(X,factorial_plus_one(Y))
    | less(Y,X) )).

cnf(division_by_prime_divisor,axiom,
    ( prime(X)
    | divides(prime_divisor(X),X) )).

cnf(prime_divsiors,axiom,
    ( prime(X)
    | prime(prime_divisor(X)) )).

cnf(smaller_prime_divisors,axiom,
    ( prime(X)
    | less(prime_divisor(X),X) )).

cnf(a_is_prime,hypothesis,
    ( prime(a) )).

cnf(prove_there_is_another_prime,negated_conjecture,
    ( ~ prime(X)
    | ~ less(a,X)
    | less(factorial_plus_one(a),X) )).

%--------------------------------------------------------------------------
