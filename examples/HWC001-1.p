%--------------------------------------------------------------------------
% File     : HWC001-1 : TPTP v6.2.0. Bugfixed v1.1.1.
% Domain   : Hardware Creation
% Problem  : Design an OR gate using NAND gates
% Version  : [WO+92] axioms.
% English  :

% Refs     : [WO+92] Wos et al. (1992), Automated Reasoning: Introduction a
% Source   : [ANL]
% Names    : - [WO+92]
%          : design_or.ver1.clauses [ANL]

% Status   : Unsatisfiable
% Rating   : 0.00 v6.0.0, 0.11 v5.5.0, 0.38 v5.4.0, 0.40 v5.3.0, 0.42 v5.2.0, 0.12 v5.1.0, 0.00 v5.0.0, 0.14 v4.1.0, 0.00 v3.1.0, 0.11 v2.7.0, 0.00 v2.6.0, 0.29 v2.5.0, 0.00 v2.2.1, 0.22 v2.2.0, 0.29 v2.1.0, 0.60 v2.0.0
% Syntax   : Number of clauses     :    9 (   0 non-Horn;   8 unit;   6 RR)
%            Number of atoms       :   11 (   5 equality)
%            Maximal clause size   :    3 (   1 average)
%            Number of predicates  :    2 (   0 propositional; 2-4 arity)
%            Number of functors    :    5 (   2 constant; 0-2 arity)
%            Number of variables   :   12 (   1 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_UNS_RFO_SEQ_HRN

% Comments : This input file came with no explanation of the representation.
% Bugfixes : v1.1.1 - Fixed clause in not_substitution1.
%--------------------------------------------------------------------------
%----Problem axioms
cnf(nand_definition,axiom,
    ( nand(X,Y) = not(and(X,Y)) )).

cnf(and_definition1,axiom,
    ( and(X,n0) = n0 )).

cnf(and_definition2,axiom,
    ( and(X,n1) = X )).

cnf(not_definition1,axiom,
    ( not(n0) = n1 )).

cnf(not_definition2,axiom,
    ( not(n1) = n0 )).

cnf(nand_table_definition,negated_conjecture,
    ( ~ output(X1,X2,X3,X4)
    | ~ output(Y1,Y2,Y3,Y4)
    | output(nand(X1,Y1),nand(X2,Y2),nand(X3,Y3),nand(X4,Y4)) )).

cnf(input1,negated_conjecture,
    ( output(n0,n0,n1,n1) )).

cnf(input2,negated_conjecture,
    ( output(n0,n1,n0,n1) )).

cnf(prove_cannot_make_or,negated_conjecture,
    ( ~ output(n0,n1,n1,n1) )).

%--------------------------------------------------------------------------
