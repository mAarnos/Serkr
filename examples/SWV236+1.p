%------------------------------------------------------------------------------
% File     : SWV236+1 : TPTP v6.2.0. Released v3.2.0.
% Domain   : Software Verification (Security)
% Problem  : IBM's known exporter attack on the 4758 CCA API
% Version  : Especial
% English  : This is a model of the key-management commands from the IBM CCA
%            API, which controls the 4758 hardware cryptoprocessor module. The
%            conjecture represents the security of a customer's PIN. This
%            variant is IBM's own known exporter attack they gave in response
%            to the attack discovery.

% Refs     : [BA01]  Bond & Anderson (2001), API-Level Attacks on Embedded
%          : [Ste05] Steel (2005), Deduction with XOR Constraints in Securi
%          : [Ste06] Steel (2006), Email to G. Sutcliffe
% Source   : [Ste06]
% Names    :

% Status   : Theorem
% Rating   : 0.12 v6.2.0, 0.16 v6.1.0, 0.20 v6.0.0, 0.17 v5.5.0, 0.30 v5.4.0, 0.36 v5.3.0, 0.33 v5.2.0, 0.25 v5.1.0, 0.24 v5.0.0, 0.21 v4.1.0, 0.22 v4.0.0, 0.21 v3.7.0, 0.20 v3.5.0, 0.21 v3.4.0, 0.37 v3.3.0, 0.21 v3.2.0
% Syntax   : Number of formulae    :   28 (  15 unit)
%            Number of atoms       :   58 (   5 equality)
%            Maximal formula depth :   11 (   4 average)
%            Number of connectives :   30 (   0 ~  ;   0  |;  18  &)
%                                         (   0 <=>;  12 =>;   0 <=)
%                                         (   0 <~>;   0 ~|;   0 ~&)
%            Number of predicates  :    2 (   0 propositional; 1-2 arity)
%            Number of functors    :   13 (  10 constant; 0-2 arity)
%            Number of variables   :   44 (   0 singleton;  43 !;   1 ?)
%            Maximal term depth    :    5 (   2 average)
% SPC      : FOF_THM_RFO_SEQ

% Comments : Have added in AC-axioms for the xor operator (*)
%------------------------------------------------------------------------------
fof(xor_commutative,axiom,(
    ! [X1,X2] : xor(X1,X2) = xor(X2,X1) )).

fof(xor_associative,axiom,(
    ! [X1,X2,X3] : xor(X1,xor(X2,X3)) = xor(xor(X1,X2),X3) )).

fof(encryption_decryption_cancellation,axiom,(
    ! [X1,X2] : decrypt(X1,crypt(X1,X2)) = X2 )).

fof(xor_rules_1,axiom,(
    ! [X1] : xor(X1,id) = X1 )).

fof(xor_rules_2,axiom,(
    ! [X1] : xor(X1,X1) = id )).

fof(key_import,axiom,(
    ! [Xkek1,Xtype1,Xk1,Xtype2,Xkek2] :
      ( ( p(crypt(xor(Xkek1,Xtype1),Xk1))
        & p(Xtype2)
        & p(crypt(xor(km,imp),Xkek2)) )
     => p(crypt(xor(km,Xtype2),decrypt(xor(Xkek2,Xtype2),crypt(xor(Xkek1,Xtype1),Xk1)))) ) )).

fof(key_export,axiom,(
    ! [Xtype,Xk1,Xkek1] :
      ( ( p(crypt(xor(km,Xtype),Xk1))
        & p(Xtype)
        & p(crypt(xor(km,exp),Xkek1)) )
     => p(crypt(xor(Xkek1,Xtype),Xk1)) ) )).

fof(key_part_import___part_1,axiom,(
    ! [Xk,Xtype] :
      ( ( p(Xk)
        & p(Xtype) )
     => p(crypt(xor(km,xor(kp,Xtype)),Xk)) ) )).

fof(key_part_import___part_2,axiom,(
    ! [Xk1,Xtype,Xk2] :
      ( ( p(Xk1)
        & p(crypt(xor(km,xor(kp,Xtype)),Xk2))
        & p(Xtype) )
     => p(crypt(xor(km,xor(Xtype,kp)),xor(Xk1,Xk2))) ) )).

fof(key_part_import___part_3,axiom,(
    ! [Xk1,Xtype,Xk2] :
      ( ( p(Xk1)
        & p(crypt(xor(km,xor(Xtype,kp)),Xk2))
        & p(Xtype) )
     => p(crypt(xor(km,Xtype),xor(Xk2,Xk1))) ) )).

fof(encrypt_data,axiom,(
    ! [X1,Xk1] :
      ( ( p(X1)
        & p(crypt(xor(km,data),Xk1)) )
     => p(crypt(Xk1,X1)) ) )).

fof(decrypt_data,axiom,(
    ! [X1,Xk1] :
      ( ( p(X1)
        & p(crypt(xor(km,data),Xk1)) )
     => p(decrypt(Xk1,X1)) ) )).

fof(key_translate,axiom,(
    ! [Xk,Xk1,Xtype2,Xkek1,Xkek2,Xtype] :
      ( ( p(crypt(Xk,Xk1))
        & p(Xtype2)
        & p(crypt(xor(km,imp),Xkek1))
        & p(crypt(xor(km,exp),Xkek2)) )
     => p(crypt(xor(Xkek2,Xtype),decrypt(xor(Xtype2,Xkek1),crypt(Xk,Xk1)))) ) )).

fof(combine_with_XOR,axiom,(
    ! [X1,X2] :
      ( ( p(X1)
        & p(X2) )
     => p(xor(X1,X2)) ) )).

fof(decrypt_knowledge,axiom,(
    ! [X1,X2] :
      ( ( p(crypt(X1,X2))
        & p(X1) )
     => p(X2) ) )).

fof(encrypt_knowledge,axiom,(
    ! [X1,X2] :
      ( ( p(X2)
        & p(X1) )
     => p(crypt(X1,X2)) ) )).

% for the IBM attack, we know the data cv is 0

fof(data_cv_is_known_to_be_zero,axiom,(
    ! [X1,X2] :
      ( p(crypt(xor(X1,data),X2))
     => p(crypt(X1,X2)) ) )).

fof(initial_knowledge_of_intruder_1,axiom,(
    p(kp) )).

fof(initial_knowledge_of_intruder_2,axiom,(
    p(imp) )).

fof(initial_knowledge_of_intruder_3,axiom,(
    p(data) )).

fof(initial_knowledge_of_intruder_4,axiom,(
    p(id) )).

fof(initial_knowledge_of_intruder_5,axiom,(
    p(pin) )).

fof(initial_knowledge_of_intruder_6,axiom,(
    p(crypt(xor(km,pin),pp)) )).

%----EURK stands for 'encrypted unknown random key'
fof(initial_knowledge_of_intruder_7,axiom,(
    p(crypt(xor(km,exp),eurk)) )).

fof(initial_knowledge_of_intruder_8,axiom,(
    p(crypt(xor(km,data),eurk)) )).

fof(initial_knowledge_of_intruder_9,axiom,(
    p(exp) )).

fof(an_account_number,axiom,(
    p(a) )).

fof(find_known_exporter,conjecture,(
    ? [X] :
      ( p(crypt(xor(km,exp),X))
      & p(X) ) )).

%------------------------------------------------------------------------------
