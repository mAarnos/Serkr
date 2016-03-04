fof(axiom_1, axiom, ( ! [X, Y, Z] : f(f(X, Y), Z) = f(X, f(Y, Z)) )).
fof(axiom_2, axiom, ( ! [X] : f(a, X) = X )).
fof(axiom_3, axiom, ( ! [X] : ? [Y] : f(Y, X) = a )).
fof(prove_this, conjecture, ( ! [X, Y, Z] : ( f(X, Y) = f(Z, Y) => X = Z ) )).
