%--------------------------------------------------------------------------
% File     : ANA002-2 : TPTP v6.2.0. Bugfixed v1.0.1.
% Domain   : Analysis
% Problem  : Intermediate value theorem
% Version  : [WB87] axioms : Reduced & Augmented > Complete.
%            Theorem formulation : Ground form of goal.
% English  : If a function f is continuous in a real closed interval
%            [a,b], where f(a)<=0 and 0<=f(b), then there exists X such
%            that f(X) = 0.

% Refs     : [WB87]  Wang & Bledsoe (1987), Hierarchical Deduction
% Source   : [TPTP]
% Names    :

% Status   : Unsatisfiable
% Rating   : 0.14 v6.2.0, 0.22 v6.1.0, 0.29 v6.0.0, 0.43 v5.5.0, 0.50 v5.4.0, 0.70 v5.2.0, 0.60 v5.1.0, 0.55 v5.0.0, 0.50 v4.1.0, 0.38 v4.0.1, 0.40 v4.0.0, 0.43 v3.4.0, 0.25 v3.3.0, 0.33 v2.7.0, 0.25 v2.6.0, 0.33 v2.5.0, 0.40 v2.3.0, 0.33 v2.2.1, 0.25 v2.1.0, 0.75 v2.0.0
% Syntax   : Number of clauses     :   18 (   5 non-Horn;   5 unit;  14 RR)
%            Number of atoms       :   42 (   0 equality)
%            Maximal clause size   :    5 (   2 average)
%            Number of predicates  :    2 (   0 propositional; 2-3 arity)
%            Number of functors    :    9 (   4 constant; 0-2 arity)
%            Number of variables   :   23 (   0 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_UNS_RFO_NEQ_NHN

% Comments : The l comes from an instantiation of the least-up-bound axiom,
%            which says that there exists a point l such that f(l) = 0.
%          : In this version the clause in_interval is used.
% Bugfixes : v1.0.1 - Clause crossover3_and_g_function2, literal 2, fixed.
%--------------------------------------------------------------------------
%----Definition of in an interval
cnf(in_interval,axiom,
    ( in_interval(Lower,X,Upper)
    | ~ less_than_or_equal(Lower,X)
    | ~ less_than_or_equal(X,Upper) )).

%----Inequality axioms
cnf(reflexivity_of_less,axiom,
    ( less_than_or_equal(X,X) )).

cnf(totality_of_less,axiom,
    ( less_than_or_equal(X,Y)
    | less_than_or_equal(Y,X) )).

cnf(transitivity_of_less,axiom,
    ( less_than_or_equal(X,Z)
    | ~ less_than_or_equal(X,Y)
    | ~ less_than_or_equal(Y,Z) )).

%----Interpolation axioms
cnf(interpolation1,axiom,
    ( ~ less_than_or_equal(X,q(Y,X))
    | less_than_or_equal(X,Y) )).

cnf(interpolation2,axiom,
    ( ~ less_than_or_equal(q(X,Y),X)
    | less_than_or_equal(Y,X) )).

%----Continuity axioms
cnf(continuity1,axiom,
    ( less_than_or_equal(f(X),n0)
    | ~ less_than_or_equal(X,h(X))
    | ~ in_interval(lower_bound,X,upper_bound) )).

cnf(continuity2,axiom,
    ( less_than_or_equal(f(X),n0)
    | ~ less_than_or_equal(Y,X)
    | ~ less_than_or_equal(f(Y),n0)
    | less_than_or_equal(Y,h(X))
    | ~ in_interval(lower_bound,X,upper_bound) )).

cnf(continuity3,axiom,
    ( less_than_or_equal(n0,f(X))
    | ~ less_than_or_equal(k(X),X)
    | ~ in_interval(lower_bound,X,upper_bound) )).

cnf(continuity4,axiom,
    ( less_than_or_equal(n0,f(X))
    | ~ less_than_or_equal(X,Y)
    | ~ less_than_or_equal(n0,f(Y))
    | less_than_or_equal(k(X),Y)
    | ~ in_interval(lower_bound,X,upper_bound) )).

%----Least upper bound axioms
cnf(crossover1,axiom,
    ( less_than_or_equal(X,l)
    | ~ less_than_or_equal(X,upper_bound)
    | ~ less_than_or_equal(f(X),n0) )).

cnf(crossover2_and_g_function1,axiom,
    ( less_than_or_equal(g(X),upper_bound)
    | less_than_or_equal(l,X) )).

cnf(crossover3_and_g_function2,axiom,
    ( less_than_or_equal(f(g(X)),n0)
    | less_than_or_equal(l,X) )).

cnf(crossover4_and_g_function3,axiom,
    ( ~ less_than_or_equal(g(X),X)
    | less_than_or_equal(l,X) )).

%----Enpoints of the interval
cnf(the_interval,hypothesis,
    ( less_than_or_equal(lower_bound,upper_bound) )).

cnf(lower_mapping,hypothesis,
    ( less_than_or_equal(f(lower_bound),n0) )).

cnf(upper_mapping,hypothesis,
    ( less_than_or_equal(n0,f(upper_bound)) )).

cnf(prove_there_is_x_which_crosses,negated_conjecture,
    ( ~ in_interval(f(l),n0,f(l)) )).

%--------------------------------------------------------------------------
