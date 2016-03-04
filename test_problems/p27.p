fof(axiom_1, axiom, ( ? [X] : ( f(X) & ~ g(X) ) )).
fof(axiom_2, axiom, ( ! [X] : ( f(X) => h(X) ) )).
fof(axiom_3, axiom, ( ! [X] : ( ( j(X) & i(X) ) => f(X) ) )).
fof(axiom_4, axiom, ( ( ? [X] : ( h(X) & ~ g(X) ) ) => ! [X] : ( i(X) => ~ h(X) ) )).
fof(prove_this, conjecture, ( ! [X] : ( j(X) => ~ i(X) ) )).