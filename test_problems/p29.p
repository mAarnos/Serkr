fof(axiom_1, axiom, ( ? [X] : f(X) )).
fof(axiom_2, axiom, ( ? [X] : g(X) )).
fof(prove_this, conjecture, ( ( ( ! [X] : ( f(X) => h(X) ) ) & ( ! [X] : ( g(X) => j(X) ) ) ) <=> ( ! [X, Y] : ( ( f(X) & g(Y) ) => ( h(X) & j(Y) ) ) ) )).