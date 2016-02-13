%--------------------------------------------------------------------------
% File     : TOP001-2 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Topology
% Problem  : Topology generated by a basis forms a topological space, part 1
% Version  : [WM89] axioms : Incomplete > Reduced & Augmented > Incomplete.
% English  :

% Refs     : [WM89]  Wick & McCune (1989), Automated Reasoning about Elemen
% Source   : [WM89]
% Names    : Lemma 1a [WM89]

% Status   : Unsatisfiable
% Rating   : 0.14 v6.2.0, 0.00 v5.4.0, 0.10 v5.1.0, 0.00 v5.0.0, 0.07 v4.1.0, 0.00 v2.7.0, 0.12 v2.6.0, 0.00 v2.4.0, 0.00 v2.1.0, 0.00 v2.0.0
% Syntax   : Number of clauses     :   13 (   1 non-Horn;   3 unit;  11 RR)
%            Number of atoms       :   27 (   0 equality)
%            Maximal clause size   :    3 (   2 average)
%            Number of predicates  :    5 (   0 propositional; 2-2 arity)
%            Number of functors    :    7 (   2 constant; 0-3 arity)
%            Number of variables   :   25 (   0 singleton)
%            Maximal term depth    :    3 (   1 average)
% SPC      : CNF_UNS_RFO_NEQ_NHN

% Comments : The axioms in this version are known to be incomplete. To
%            make them complete it is be necessary to add appropriate set
%            theory axioms.
%--------------------------------------------------------------------------
%----Include Point-set topology axioms
% include('Axioms/TOP001-0.ax').
%--------------------------------------------------------------------------
%----Sigma (union of members).
cnf(union_of_members_1,axiom,
    ( ~ element_of_set(U,union_of_members(Vf))
    | element_of_set(U,f1(Vf,U)) )).

cnf(union_of_members_2,axiom,
    ( ~ element_of_set(U,union_of_members(Vf))
    | element_of_collection(f1(Vf,U),Vf) )).

cnf(union_of_members_3,axiom,
    ( element_of_set(U,union_of_members(Vf))
    | ~ element_of_set(U,Uu1)
    | ~ element_of_collection(Uu1,Vf) )).

%----Basis for a topology
cnf(basis_for_topology_28,axiom,
    ( ~ basis(X,Vf)
    | equal_sets(union_of_members(Vf),X) )).

%----Topology generated by a basis
cnf(topology_generated_37,axiom,
    ( ~ element_of_collection(U,top_of_basis(Vf))
    | ~ element_of_set(X,U)
    | element_of_set(X,f10(Vf,U,X)) )).

cnf(topology_generated_38,axiom,
    ( ~ element_of_collection(U,top_of_basis(Vf))
    | ~ element_of_set(X,U)
    | element_of_collection(f10(Vf,U,X),Vf) )).

cnf(set_theory_1,axiom,
    ( subset_sets(X,X) )).

cnf(set_theory_2,axiom,
    ( ~ subset_sets(X,Y)
    | ~ element_of_set(U,X)
    | element_of_set(U,Y) )).

cnf(set_theory_3,axiom,
    ( ~ equal_sets(X,Y)
    | subset_sets(X,Y) )).

cnf(set_theory_4,axiom,
    ( subset_sets(X,Y)
    | element_of_set(in_1st_set(X,Y),X) )).

cnf(set_theory_5,axiom,
    ( subset_sets(X,Y)
    | ~ element_of_set(in_1st_set(X,Y),Y) )).

cnf(lemma_1a_1,negated_conjecture,
    ( basis(cx,f) )).

cnf(lemma_1a_2,negated_conjecture,
    ( ~ subset_sets(union_of_members(top_of_basis(f)),cx) )).

%--------------------------------------------------------------------------