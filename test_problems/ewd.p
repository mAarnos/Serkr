fof(axiom_1, axiom, ( ! [X] : ( f(X) => g(X) ) )).
fof(axiom_2, axiom, ( ? [X] : f(X) )).
fof(axiom_3, axiom, ( ! [X, Y] : ( ( g(X) & g(Y) ) => X = Y ) )).
fof(prove_this, conjecture, ( ! [X] : ( g(X) => f(X) ) )).