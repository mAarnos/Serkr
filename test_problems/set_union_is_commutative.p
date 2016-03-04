% The normal definition of union.
fof(union, axiom, ( ! [A, B, X] : ( in(X, union(A, B)) <=> ( in(X, A) | in(X, B) ) ) )).

% The normal definition of set equality.
fof(set_equality, axiom, ( ! [A, B] : ( ( ! [X] : ( in(X, A) <=> in(X, B) ) ) => A = B ) )).

% And the obvious result. 
fof(set_union_is_commutative, conjecture, ( ! [A, B] : ( union(A, B) = union(B, A) ) )).