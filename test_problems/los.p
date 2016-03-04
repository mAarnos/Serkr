fof(axiom_1, axiom, ( ! [X, Y, Z] : ( ( p(X, Y) & p(Y, Z) ) => p(X, Z) ) )).
fof(axiom_2, axiom, ( ! [X, Y, Z] : ( ( q(X, Y) & q(Y, Z) ) => q(X, Z) ) )).
fof(axiom_3, axiom, ( ! [X, Y] : ( q(X, Y) => q(Y, X) ) )).
fof(axiom_4, axiom, ( ! [X, Y] : ( p(X, Y) | q(Y, X) ) )).
fof(prove_this, conjecture, ( ( ! [X, Y] : ( p(X, Y) ) ) | ( ! [X, Y] : ( q(X, Y) ) ) )).