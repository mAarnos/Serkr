%--------------------------------------------------------------------------
% File     : ANA002-4 : TPTP v6.2.0. Released v1.1.0.
% Domain   : Analysis
% Problem  : Intermediate value theorem
% Version  : [WB87] axioms.
%            Theorem formulation : Ground form of goal.
% English  : If a function f is continuous in a real closed interval
%            [a,b], where f(a)<=0 and 0<=f(b), then there exists X such
%            that f(X) = 0.

% Refs     : [WB87]  Wang & Bledsoe (1987), Hierarchical Deduction
% Source   : [WB87]
% Names    : ivt.lop [SETHEO]

% Status   : Unsatisfiable
% Rating   : 0.14 v6.2.0, 0.22 v6.1.0, 0.29 v6.0.0, 0.43 v5.5.0, 0.38 v5.4.0, 0.60 v5.3.0, 0.70 v5.2.0, 0.50 v5.1.0, 0.45 v5.0.0, 0.50 v4.1.0, 0.38 v4.0.1, 0.40 v4.0.0, 0.57 v3.4.0, 0.50 v3.3.0, 0.33 v2.7.0, 0.12 v2.6.0, 0.33 v2.5.0, 0.40 v2.3.0, 0.33 v2.2.1, 0.50 v2.1.0, 1.00 v2.0.0
% Syntax   : Number of clauses     :   17 (   5 non-Horn;   4 unit;  13 RR)
%            Number of atoms       :   44 (   0 equality)
%            Maximal clause size   :    6 (   3 average)
%            Number of predicates  :    1 (   0 propositional; 2-2 arity)
%            Number of functors    :    9 (   4 constant; 0-2 arity)
%            Number of variables   :   20 (   0 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_UNS_RFO_NEQ_NHN

% Comments : The l comes from an instantiation of the least-up-bound axiom,
%            which says that there exists a point l such that f(l) = 0.
%--------------------------------------------------------------------------
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
    | ~ less_than_or_equal(lower_bound,X)
    | ~ less_than_or_equal(X,upper_bound) )).

cnf(continuity2,axiom,
    ( less_than_or_equal(f(X),n0)
    | ~ less_than_or_equal(Y,X)
    | ~ less_than_or_equal(f(Y),n0)
    | less_than_or_equal(Y,h(X))
    | ~ less_than_or_equal(lower_bound,X)
    | ~ less_than_or_equal(X,upper_bound) )).

cnf(continuity3,axiom,
    ( less_than_or_equal(n0,f(X))
    | ~ less_than_or_equal(k(X),X)
    | ~ less_than_or_equal(lower_bound,X)
    | ~ less_than_or_equal(X,upper_bound) )).

cnf(continuity4,axiom,
    ( less_than_or_equal(n0,f(X))
    | ~ less_than_or_equal(X,Y)
    | ~ less_than_or_equal(n0,f(Y))
    | less_than_or_equal(k(X),Y)
    | ~ less_than_or_equal(lower_bound,X)
    | ~ less_than_or_equal(X,upper_bound) )).

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

%----Endpoints of the interval
cnf(the_interval,hypothesis,
    ( less_than_or_equal(lower_bound,upper_bound) )).

cnf(lower_mapping,hypothesis,
    ( less_than_or_equal(f(lower_bound),n0) )).

cnf(upper_mapping,hypothesis,
    ( less_than_or_equal(n0,f(upper_bound)) )).

cnf(prove_there_is_x_which_crosses,negated_conjecture,
    ( ~ less_than_or_equal(f(l),n0)
    | ~ less_than_or_equal(n0,f(l)) )).

%--------------------------------------------------------------------------
