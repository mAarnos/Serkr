%--------------------------------------------------------------------------
% File     : GEO076-4 : TPTP v6.2.0. Released v1.0.0.
% Domain   : Geometry (Hilbert)
% Problem  : There is no point on every line
% Version  : [Ben92] axioms.
% English  :

% Refs     : [Ben92] Benanav (1992), Recognising Unnecessary Clauses in Res
% Source   : [Ben92]
% Names    : G15 [Ben92]

% Status   : Unsatisfiable
% Rating   : 0.18 v6.2.0, 0.20 v6.1.0, 0.36 v6.0.0, 0.40 v5.5.0, 0.55 v5.3.0, 0.61 v5.2.0, 0.56 v5.1.0, 0.59 v5.0.0, 0.43 v4.1.0, 0.38 v4.0.1, 0.45 v4.0.0, 0.36 v3.7.0, 0.30 v3.5.0, 0.36 v3.4.0, 0.42 v3.3.0, 0.43 v3.2.0, 0.46 v3.1.0, 0.45 v2.7.0, 0.42 v2.6.0, 0.30 v2.5.0, 0.58 v2.4.0, 0.44 v2.2.1, 0.67 v2.2.0, 0.78 v2.1.0, 0.89 v2.0.0
% Syntax   : Number of clauses     :   33 (  18 non-Horn;   2 unit;  33 RR)
%            Number of atoms       :  177 (  43 equality)
%            Maximal clause size   :   16 (   5 average)
%            Number of predicates  :    6 (   0 propositional; 1-3 arity)
%            Number of functors    :   11 (   2 constant; 0-3 arity)
%            Number of variables   :   71 (   0 singleton)
%            Maximal term depth    :    2 (   1 average)
% SPC      : CNF_UNS_RFO_SEQ_NHN

% Comments :
%--------------------------------------------------------------------------
%----Include axioms for Hilbert geometry
include('examples/GEO003-0.ax').
%--------------------------------------------------------------------------
cnf(there_is_a_point,hypothesis,
    ( point(a_point) )).

cnf(prove_point_is_not_on_every_line,negated_conjecture,
    ( ~ line(Line)
    | on(a_point,Line) )).

%--------------------------------------------------------------------------
