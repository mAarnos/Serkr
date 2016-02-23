%------------------------------------------------------------------------------
% File     : ALG211+1 : TPTP v6.2.0. Released v3.1.0.
% Domain   : General Algebra
% Problem  : Vector spaces and bases
% Version  : [Shu96] axioms : Especial.
% English  :

% Refs     : [BG80]  Bishop & Goldberg (1980), Tensor Analysis on Manifolds
%          : [Sch96] Shults (1996), Email to Geoff Sutcliffe
% Source   : [Sch96]
% Names    :

% Status   : Theorem
% Rating   : 0.00 v6.1.0, 0.04 v6.0.0, 0.50 v5.5.0, 0.12 v5.4.0, 0.13 v5.3.0, 0.22 v5.2.0, 0.07 v5.0.0, 0.05 v4.1.0, 0.06 v4.0.1, 0.05 v3.7.0, 0.00 v3.1.0
% Syntax   : Number of formulae    :    6 (   0 unit)
%            Number of atoms       :   19 (   0 equality)
%            Maximal formula depth :    7 (   6 average)
%            Number of connectives :   13 (   0 ~  ;   0  |;   6  &)
%                                         (   1 <=>;   6 =>;   0 <=)
%                                         (   0 <~>;   0 ~|;   0 ~&)
%            Number of predicates  :    5 (   0 propositional; 1-2 arity)
%            Number of functors    :    3 (   1 constant; 0-2 arity)
%            Number of variables   :   17 (   0 singleton;  13 !;   4 ?)
%            Maximal term depth    :    2 (   1 average)
% SPC      : FOF_THM_RFO_NEQ

% Comments :
%------------------------------------------------------------------------------
%----The defintion of a basis
fof(basis_of,axiom,(
    ! [B,V] :
      ( basis_of(B,V)
     => ( lin_ind_subset(B,V)
        & a_subset_of(B,vec_to_class(V)) ) ) )).

%----Proposition 2.2.5 from Bishop and Goldberg
fof(bg_2_2_5,axiom,(
    ! [S,T,V] :
      ( ( lin_ind_subset(S,V)
        & basis_of(T,V) )
     => ? [U] :
          ( a_subset_of(U,T)
          & basis_of(union(S,U),V) ) ) )).

%----The first Remark on page 63 of Bishop and Goldberg
fof(bg_remark_63_a,axiom,(
    ! [A] :
      ( a_vector_space(A)
     => ? [B] : basis_of(B,A) ) )).

%----The definition of a subspace
fof(bg_2_4_a,axiom,(
    ! [A,B] :
      ( a_vector_subspace_of(A,B)
     => a_vector_space(A) ) )).

%----Proposition 2.4.2 in Bishop and Goldberg
fof(bg_2_4_2,axiom,(
    ! [W,V,E] :
      ( ( a_vector_subspace_of(W,V)
        & a_subset_of(E,vec_to_class(W)) )
     => ( lin_ind_subset(E,W)
      <=> lin_ind_subset(E,V) ) ) )).

fof(bg_2_4_3,conjecture,(
    ! [W,V] :
      ( ( a_vector_subspace_of(W,V)
        & a_vector_space(V) )
     => ? [E,F] :
          ( basis_of(union(E,F),V)
          & basis_of(E,W) ) ) )).

%------------------------------------------------------------------------------
