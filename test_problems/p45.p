fof(axiom_1, axiom, ( ! [X] : ( ( f(X) & ! [Y] : ( ( g(Y) & h(X, Y) ) => j(X, Y) ) ) => ! [Y] : ( ( g(Y) & h(X, Y) ) => k(Y) ) ) )).
fof(axiom_2, axiom, ( ~ ( ? [Y] : ( l(Y) & k(Y) ) ) )).
fof(axiom_3, axiom, ( ? [X] : ( f(X) & ! [Y] : ( h(X, Y) => l(Y) ) & ! [Y] : ( ( g(Y) & h(X, Y) ) => j(X, Y) ) ) )).
fof(prove_this, conjecture, ( ? [X] : ( f(X) & ~ ? [Y] : ( g(Y) & h(X, Y) ) ) )).