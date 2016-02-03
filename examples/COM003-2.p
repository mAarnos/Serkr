%--------------------------------------------------------------------------
% File     : COM003-2 : TPTP v6.2.0. Released v1.1.0.
% Domain   : Computing Theory
% Problem  : The halting problem is undecidable
% Version  : [Bru91] axioms.
% English  :

% Refs     : [Bur87b] Burkholder (1987), A 76th Automated Theorem Proving Pr
%          : [Bru91] Brushi (1991), The Halting Problem
% Source   : [Bru91]
% Names    : - [Bru91]

% Status   : Unsatisfiable
% Rating   : 0.00 v2.0.0
% Syntax   : Number of clauses     :   43 (   2 non-Horn;   1 unit;  35 RR)
%            Number of atoms       :  109 (   0 equality)
%            Maximal clause size   :    5 (   3 average)
%            Number of predicates  :   17 (   0 propositional; 1-4 arity)
%            Number of functors    :   10 (   6 constant; 0-1 arity)
%            Number of variables   :  103 (  24 singleton)
%            Maximal term depth    :    2 (   1 average)
% SPC      : CNF_UNS_RFO_NEQ_NHN

% Comments :
%--------------------------------------------------------------------------
%----Definitions of the new predicates.
% (all X (program_decides(X) <->
%         (all Y (program(Y) -> (all Z decides(X,Y,Z)))))).
cnf(program_decides1,axiom,
    ( ~ program_decides(X)
    | ~ program(Y)
    | decides(X,Y,Z) )).

cnf(program_decides2,axiom,
    ( program_decides(X)
    | program(f2(X)) )).

cnf(program_decides3,axiom,
    ( program_decides(X)
    | ~ decides(X,f2(X),f1(X)) )).

% (all X (program_program_decides(X) <->
%         (program(X) & program_decides(X)))).
cnf(program_program_decides1,axiom,
    ( ~ program_program_decides(X)
    | program(X) )).

cnf(program_program_decides2,axiom,
    ( ~ program_program_decides(X)
    | program_decides(X) )).

cnf(program_program_decides3,axiom,
    ( program_program_decides(X)
    | ~ program(X)
    | ~ program_decides(X) )).

% (all X (algorithm_program_decides(X) <->
%         (algorithm(X) & program_decides(X)))).
cnf(algorithm_program_decides1,axiom,
    ( ~ algorithm_program_decides(X)
    | algorithm(X) )).

cnf(algorithm_program_decides2,axiom,
    ( ~ algorithm_program_decides(X)
    | program_decides(X) )).

cnf(algorithm_program_decides3,axiom,
    ( algorithm_program_decides(X)
    | ~ algorithm(X)
    | ~ program_decides(X) )).

% (all X (all Y (program_halts2(X,Y) <->
%                (program(X) & halts2(X,Y))))).
cnf(program_halts1,axiom,
    ( ~ program_halts2(X,Y)
    | program(X) )).

cnf(program_halts3a,axiom,
    ( ~ program_halts2(X,Y)
    | halts2(X,Y) )).

cnf(program_halts3b,axiom,
    ( program_halts2(X,Y)
    | ~ program(X)
    | ~ halts2(X,Y) )).

% (all X (all Y (all Z (all W (halts3_outputs(X,Y,Z,W) <->
%                              (halts3(X,Y,Z) & outputs(X,W))))))).
cnf(halts3_outputs1,axiom,
    ( ~ halts3_outputs(X,Y,Z,W)
    | halts3(X,Y,Z) )).

cnf(halts3_outputs2,axiom,
    ( ~ halts3_outputs(X,Y,Z,W)
    | outputs(X,W) )).

cnf(halts3_outputs3,axiom,
    ( halts3_outputs(X,Y,Z,W)
    | ~ halts3(X,Y,Z)
    | ~ outputs(X,W) )).

% (all X (all Y (program_not_halts2(X,Y) <->
%                (program(X) & -halts2(X,Y))))).
cnf(program_not_halts1,axiom,
    ( ~ program_not_halts2(X,Y)
    | program(X) )).

cnf(program_not_halts3a,axiom,
    ( ~ program_not_halts2(X,Y)
    | ~ halts2(X,Y) )).

cnf(program_not_halts3b,axiom,
    ( program_not_halts2(X,Y)
    | ~ program(X)
    | halts2(X,Y) )).

% (all X (all Y (all W (halts2_outputs(X,Y,W) <->
%                       (halts2(X,Y) & outputs(X,W)))))).
cnf(halts2_outputs1,axiom,
    ( ~ halts2_outputs(X,Y,W)
    | halts2(X,Y) )).

cnf(halts2_outputs2,axiom,
    ( ~ halts2_outputs(X,Y,W)
    | outputs(X,W) )).

cnf(halts2_outputs3,axiom,
    ( halts2_outputs(X,Y,W)
    | ~ halts2(X,Y)
    | ~ outputs(X,W) )).

% (all X (all Y (all Z (all W (program_halts2_halts3_outputs(X,Y,Z,W) <->
%                              (program_halts2(Y,Z) &
%                               halts3_outputs(X,Y,Z,W))))))).
cnf(program_halts2_halts3_outputs1,axiom,
    ( ~ program_halts2_halts3_outputs(X,Y,Z,W)
    | program_halts2(Y,Z) )).

cnf(program_halts2_halts3_outputs2,axiom,
    ( ~ program_halts2_halts3_outputs(X,Y,Z,W)
    | halts3_outputs(X,Y,Z,W) )).

cnf(program_halts2_halts3_outputs3,axiom,
    ( program_halts2_halts3_outputs(X,Y,Z,W)
    | ~ program_halts2(Y,Z)
    | ~ halts3_outputs(X,Y,Z,W) )).

% (all X (all Y (all Z (all W (program_not_halts2_halts3_outputs(X,Y,Z,W) <->
%                              (program_not_halts2(Y,Z) &
%                               halts3_outputs(X,Y,Z,W))))))).
cnf(program_not_halts2_halts3_outputs1,axiom,
    ( ~ program_not_halts2_halts3_outputs(X,Y,Z,W)
    | program_not_halts2(Y,Z) )).

cnf(program_not_halts2_halts3_outputs2,axiom,
    ( ~ program_not_halts2_halts3_outputs(X,Y,Z,W)
    | halts3_outputs(X,Y,Z,W) )).

cnf(program_not_halts2_halts3_outputs3,axiom,
    ( program_not_halts2_halts3_outputs(X,Y,Z,W)
    | ~ program_not_halts2(Y,Z)
    | ~ halts3_outputs(X,Y,Z,W) )).

% (all X (all Y (all W (program_halts2_halts2_outputs(X,Y,W) <->
%                       (program_halts2(Y,Y) & halts2_outputs(X,Y,W)))))).
cnf(program_halts2_halts2_outputs1,axiom,
    ( ~ program_halts2_halts2_outputs(X,Y,W)
    | program_halts2(Y,Y) )).

cnf(program_halts2_halts2_outputs2,axiom,
    ( ~ program_halts2_halts2_outputs(X,Y,W)
    | halts2_outputs(X,Y,W) )).

cnf(program_halts2_halts2_outputs3,axiom,
    ( program_halts2_halts2_outputs(X,Y,W)
    | ~ program_halts2(Y,Y)
    | ~ halts2_outputs(X,Y,W) )).

% (all X (all Y (all W (program_not_halts2_halts2_outputs(X,Y,W) <->
%               (program_not_halts2(Y,Y) & halts2_outputs(X,Y,W)))))).
cnf(program_not_halts2_halts2_outputs1,axiom,
    ( ~ program_not_halts2_halts2_outputs(X,Y,W)
    | program_not_halts2(Y,Y) )).

cnf(program_not_halts2_halts2_outputs2,axiom,
    ( ~ program_not_halts2_halts2_outputs(X,Y,W)
    | halts2_outputs(X,Y,W) )).

cnf(program_not_halts2_halts2_outputs3,axiom,
    ( program_not_halts2_halts2_outputs(X,Y,W)
    | ~ program_not_halts2(Y,Y)
    | ~ halts2_outputs(X,Y,W) )).

% New verions of the original axioms
% ((exists X algorithm_program_decides(X)) ->
%  (exists W program_program_decides(W))).
cnf(axiom1_1,hypothesis,
    ( ~ algorithm_program_decides(X)
    | program_program_decides(c1) )).

% (all W (program_program_decides(W) ->
%        (all Y (all Z (program_halts2_halts3_outputs(W,Y,Z,good) &
%                       program_not_halts2_halts3_outputs(W,Y,Z,bad)))))).
cnf(axiom2_1,hypothesis,
    ( ~ program_program_decides(W)
    | program_halts2_halts3_outputs(W,Y,Z,good) )).

cnf(axiom2_2,hypothesis,
    ( ~ program_program_decides(W)
    | program_not_halts2_halts3_outputs(W,Y,Z,bad) )).

% ((exists W (program(W) &
%             (all Y (program_halts2_halts3_outputs(W,Y,Y,good) &
%                     program_not_halts2_halts3_outputs(W,Y,Y,bad))))) ->
%  (exists V (program(V) &
%             (all Y (program_halts2_halts2_outputs(V,Y,good) &
%                     program_not_halts2_halts2_outputs(V,Y,bad)))))).
cnf(axiom3_1,hypothesis,
    ( ~ program(W)
    | ~ program_halts2_halts3_outputs(W,f3(W),f3(W),good)
    | ~ program_not_halts2_halts3_outputs(W,f3(W),f3(W),bad)
    | program(c2) )).

cnf(axiom3_2,hypothesis,
    ( ~ program(W)
    | ~ program_halts2_halts3_outputs(W,f3(W),f3(W),good)
    | ~ program_not_halts2_halts3_outputs(W,f3(W),f3(W),bad)
    | program_halts2_halts2_outputs(c2,Y,good) )).

cnf(axiom3_3,hypothesis,
    ( ~ program(W)
    | ~ program_halts2_halts3_outputs(W,f3(W),f3(W),good)
    | ~ program_not_halts2_halts3_outputs(W,f3(W),f3(W),bad)
    | program_not_halts2_halts2_outputs(c2,Y,bad) )).

% ((exists V (program(V) &
%             (all Y (program_halts2_halts2_outputs(V,Y,good) &
%                     program_not_halts2_halts2_outputs(V,Y,bad))))) ->
%  (exists U (program(U) &
%             (all Y ((program_halts2(Y,Y) -> halts2(U,Y)) &
%                     program_not_halts2_halts2_outputs(U,Y,bad)))))).
cnf(axiom4_1,hypothesis,
    ( ~ program(V)
    | ~ program_halts2_halts2_outputs(V,f4(V),good)
    | ~ program_not_halts2_halts2_outputs(V,f4(V),bad)
    | program(c3) )).

cnf(axiom4_2,hypothesis,
    ( ~ program(V)
    | ~ program_halts2_halts2_outputs(V,f4(V),good)
    | ~ program_not_halts2_halts2_outputs(V,f4(V),bad)
    | ~ program_halts2(Y,Y)
    | halts2(c3,Y) )).

cnf(axiom4_3,hypothesis,
    ( ~ program(V)
    | ~ program_halts2_halts2_outputs(V,f4(V),good)
    | ~ program_not_halts2_halts2_outputs(V,f4(V),bad)
    | program_not_halts2_halts2_outputs(c3,Y,bad) )).

%--- Prove -(exists X algorithm_program_decides(X)).
cnf(prove_algorithm_does_not_exist,negated_conjecture,
    ( algorithm_program_decides(c4) )).

%--------------------------------------------------------------------------
