fof(axiom_1, axiom, ( ! [X] : ? [Y] : f(X, Y) )).
fof(axiom_2, axiom, ( ! [X] : ? [Y] : g(X, Y) )).
fof(axiom_3, axiom, ( ! [X, Y] : ( ( f(X, Y) | g(X, Y) ) => ! [Z] : ( ( f(Y, Z) | g(Y, Z) ) =>  h(X, Z) ) ) )).
fof(prove_this, conjecture, ( ! [X] : ? [Y] : h(X, Y) )).