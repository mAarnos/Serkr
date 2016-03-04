fof(axiom_1, axiom, ( ~ ( ? [X] : ( s(X) & q(X) ) ) )).
fof(axiom_2, axiom, ( ! [X] : ( p(X) => ( q(X) | r(X) ) ) )).
fof(axiom_3, axiom, ( ( ~ ( ? [X] : p(X) ) ) => ( ? [X] : q(X) ) )).
fof(axiom_4, axiom, ( ! [X] : ( ( q(X) | r(X) ) => s(X) ) )).
fof(prove_this, conjecture, ( ? [X] : ( p(X) & r(X) ) )).