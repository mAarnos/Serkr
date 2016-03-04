fof(axiom_1, axiom, ( ! [X] : ( ( f(X) & ! [Y] : ( ( f(Y) & h(Y, X) ) => g(Y) ) ) => g(X) ) )).
fof(axiom_2, axiom, ( ( ? [X] : ( f(X) & ~ g(X) ) ) => ? [X] : ( f(X) & ~ g(X) & ! [Y] : ( ( f(Y) & ~ g(Y) ) => j(X, Y) ) ) )).
fof(axiom_3, axiom, ( ! [X, Y] : ( ( f(X) & f(Y) & h(X, Y) ) => ~ j(Y, X) ) )).
fof(prove_this, conjecture, ( ! [X] : ( f(X) => g(X) ) )).