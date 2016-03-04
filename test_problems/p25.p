fof(axiom_1, axiom, ( ? [X] : p(X) )).
fof(axiom_2, axiom, ( ! [X] : ( f(X) => ( ~ g(X) & r(X) ) ) )).
fof(axiom_3, axiom, ( ! [X] : ( p(X) => ( g(X) & f(X) ) ) )).
fof(axiom_4, axiom, ( ( ! [X] : ( p(X) => q(X) ) ) | ( ? [X] : ( p(X) & r(X) ) ) )).
fof(prove_this, conjecture, ( ? [X] : ( q(X) & p(X) ) )).