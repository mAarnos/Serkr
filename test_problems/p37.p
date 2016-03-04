fof(axiom_1, axiom, ( ! [Z] : ? [W] : ! [X] : ? [Y] : ( ( p(X, Z) => p(Y, W) ) & p(Y, Z) & ( p(Y, W) => ? [U] : q(U, W) ) ) )).
fof(axiom_2, axiom, ( ! [X, Z] : ( ~ p(X, Z) => ? [Y] : q(Y, Z) ) )).
fof(axiom_3, axiom, ( ? [X, Y] : q(X, Y) => ! [X] : r(X, X) )).
fof(prove_this, conjecture, ( ! [X] : ? [Y] : r(X, Y) )).