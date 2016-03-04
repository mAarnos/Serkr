fof(axiom_1, axiom, ( ( ? [X] : p(X) ) <=> ( ? [X] : q(X) ) )).
fof(axiom_2, axiom, ( ! [X, Y] : ( ( p(X) & q(Y) ) => ( r(X) <=> s(Y) ) ) ) ).
fof(prove_this, conjecture, ( ( ! [X] : ( p(X) => r(X) ) ) <=> ( ! [X] : ( q(X) => s(X) ) ) )).