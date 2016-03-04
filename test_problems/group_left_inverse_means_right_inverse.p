% Associativity of multiplication.
fof(associativity, axiom, ( ! [X, Y, Z] : ( mult(X, mult(Y, Z)) = mult(mult(X, Y), Z) ) )).

% Identity element.
fof(identity, axiom, ( ! [X] : ( mult(identity, X) = X ) )).

% Inverse function.
fof(inverse, axiom, ( ! [X] : ( mult(inverse(X), X) = identity ) )).

% And the pretty trivial result. 
fof(group_left_inverse_means_right_inverse, conjecture, ( ! [X] : ( mult(inverse(X), X) = mult(X, inverse(X)) ) )).