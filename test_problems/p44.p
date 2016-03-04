fof(axiom_1, axiom, ( ! [X] : ( f(X) => ( ( ? [Y] : ( g(Y) & h(X, Y) ) ) & ( ? [Y] : ( g(Y) & ~ h(X, Y) ) ) ) ) )).
fof(axiom_2, axiom, ( ? [X] : ( j(X) & ! [Y] : ( g(Y) => h(X, Y) ) ) )).
fof(prove_this, conjecture, ( ? [X] : ( j(X) & ~ f(X) ) )).