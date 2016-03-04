fof(axiom_1, axiom, ( ( ! [X] : p(X) ) => ( ! [X] : q(X) ) )).
fof(axiom_2, axiom, ( ( ! [X] : ( q(X) | r(X) ) ) => ( ? [X] : ( q(X) & s(X) ) ) )).
fof(axiom_3, axiom, ( ( ? [X] : s(X) ) => ( ! [X] : ( f(X) => g(X) ) ) )).
fof(prove_this, conjecture, ( ! [X] : ( ( p(X) & f(X) ) => g(X) ) )).