%------------------------------------------------------------------------------
% File     : GRP707-1 : TPTP v6.2.0. Released v4.0.0.
% Domain   : Group Theory (Quasigroups)
% Problem  : A C-loop of exponent four with central squares is flexible
% Version  : Especial.
% English  :

% Refs     : [KPV07] Kinyon et al. (2007), C-loops: Extensions and Construc
%          : [PS08]  Phillips & Stanovsky (2008), Automated Theorem Proving
%          : [Sta08] Stanovsky (2008), Email to G. Sutcliffe
% Source   : [Sta08]
% Names    : KPV07 [PS08]

% Status   : Unsatisfiable
% Rating   : 0.29 v6.1.0, 0.19 v6.0.0, 0.38 v5.5.0, 0.37 v5.4.0, 0.20 v5.3.0, 0.17 v5.2.0, 0.21 v5.1.0, 0.27 v5.0.0, 0.21 v4.1.0, 0.18 v4.0.1, 0.57 v4.0.0
% Syntax   : Number of clauses     :   10 (   0 non-Horn;  10 unit;   1 RR)
%            Number of atoms       :   10 (  10 equality)
%            Maximal clause size   :    1 (   1 average)
%            Number of predicates  :    1 (   0 propositional; 2-2 arity)
%            Number of functors    :    6 (   3 constant; 0-2 arity)
%            Number of variables   :   16 (   0 singleton)
%            Maximal term depth    :    4 (   2 average)
% SPC      : CNF_UNS_RFO_PEQ_UEQ

% Comments :
%------------------------------------------------------------------------------
cnf(c01,axiom,
    ( mult(A,ld(A,B)) = B )).

cnf(c02,axiom,
    ( ld(A,mult(A,B)) = B )).

cnf(c03,axiom,
    ( mult(rd(A,B),B) = A )).

cnf(c04,axiom,
    ( rd(mult(A,B),B) = A )).

cnf(c05,axiom,
    ( mult(A,unit) = A )).

cnf(c06,axiom,
    ( mult(unit,A) = A )).

cnf(c07,axiom,
    ( mult(A,mult(B,mult(B,C))) = mult(mult(mult(A,B),B),C) )).

cnf(c08,axiom,
    ( mult(A,mult(A,mult(A,A))) = unit )).

cnf(c09,axiom,
    ( mult(mult(A,A),B) = mult(B,mult(A,A)) )).

cnf(goals,negated_conjecture,
    ( mult(mult(a,b),a) != mult(a,mult(b,a)) )).

%------------------------------------------------------------------------------
