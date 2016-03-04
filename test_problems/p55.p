fof(axiom_1, axiom, ( ? [X] : ( l(X) & k(X, a) ) )).
fof(axiom_2, axiom, ( l(a) & l(b) & l(c) )).
fof(axiom_3, axiom, ( ! [X] : ( l(X) => ( X = a | X = b | X = c ) ) )).
fof(axiom_4, axiom, ( ! [X, Y] : ( k(X, Y) => h(X, Y) ) )).
fof(axiom_5, axiom, ( ! [X, Y] : ( k(X, Y) => ~ r(X, Y) ) )).
fof(axiom_6, axiom, ( ! [X] : ( h(a, X) => ~ h(c, X) ) )).
fof(axiom_7, axiom, ( ! [X] : ( X != b => h(a, X) ) )).
fof(axiom_8, axiom, ( ! [X] : ( ~ r(X, a) => h(b, X) ) )).
fof(axiom_9, axiom, ( ! [X] : ( h(a, X) => h(b, X) ) )).
fof(axiom_10, axiom, ( ! [X] : ? [Y] : ( ~ h(X, Y) ) )).
fof(axiom_11, axiom, ( a != b )).
fof(prove_this, conjecture, ( k(a, a) )).
