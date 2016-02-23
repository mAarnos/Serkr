%------------------------------------------------------------------------------
% File     : SWV237+1 : TPTP v6.2.0. Released v3.2.0.
% Domain   : Software Verification (Security)
% Problem  : Visa Security Module (VSM) attack
% Version  : Especial.
% English  : This models the API of the Visa Security Module (VSM). The
%            conjecture allows the discovery of Bond's attack.

% Refs     : [BA01]  Bond & Anderson (2001), API-Level Attacks on Embedded
%          : [Ste06] Steel (2006), Email to G. Sutcliffe
% Source   : [Ste06]
% Names    :

% Status   : Theorem
% Rating   : 0.38 v6.2.0, 0.52 v6.1.0, 0.47 v6.0.0, 0.52 v5.5.0, 0.44 v5.4.0, 0.46 v5.3.0, 0.48 v5.2.0, 0.35 v5.1.0, 0.33 v5.0.0, 0.38 v4.1.0, 0.39 v4.0.1, 0.43 v4.0.0, 0.42 v3.7.0, 0.45 v3.5.0, 0.42 v3.4.0, 0.37 v3.3.0, 0.29 v3.2.0
% Syntax   : Number of formulae    :   25 (  12 unit)
%            Number of atoms       :   62 (   3 equality)
%            Maximal formula depth :    7 (   4 average)
%            Number of connectives :   37 (   0 ~  ;   0  |;  24  &)
%                                         (   0 <=>;  13 =>;   0 <=)
%                                         (   0 <~>;   0 ~|;   0 ~&)
%            Number of predicates  :    2 (   0 propositional; 1-2 arity)
%            Number of functors    :   14 (  12 constant; 0-2 arity)
%            Number of variables   :   42 (   0 singleton;  42 !;   0 ?)
%            Maximal term depth    :    6 (   2 average)
% SPC      : FOF_THM_RFO_SEQ

% Comments : The model is originally due to Mike Bond, converted to tptp and
%            re-labelled by Graham Steel.
%------------------------------------------------------------------------------
fof(enc_dec_cancel,axiom,(
    ! [U,V] : enc(i(U),enc(U,V)) = V )).

fof(dec_enc_cancel,axiom,(
    ! [U,V] : enc(U,enc(i(U),V)) = V )).

fof(double_inverse_cancel,axiom,(
    ! [U] : i(i(U)) = U )).

fof(keys_are_symmetric,axiom,(
    ! [U] :
      ( p(U)
     => p(i(U)) ) )).

fof(key_translate_from_ZCMK_to_TMK,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(tmk,enc(i(enc(i(zcmk),V)),U))) ) )).

fof(key_translate_from_TMK_to_ZCMK,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(i(enc(i(zcmk),V)),enc(i(tmk),U))) ) )).

fof(receive_working_key_from_switch,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(wk,enc(i(tmk),U))) ) )).

fof(encrypt_a_PIN_derivation_key_under_a_TMK,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(enc(i(tmk),V),enc(i(tmk),U))) ) )).

fof(encrypt_a_stored_comms_key,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(enc(i(tmk),V),enc(i(tc),U))) ) )).

%----This command now removed from normal VSM operation to fix attack
fof(encrypt_clear_key_as_Tcomms_key,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(tc,U)) ) )).

fof(data_encrypt,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(enc(i(tc),U),V)) ) )).

fof(data_decrypt,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(i(enc(i(tc),U)),V)) ) )).

fof(data_translate_PIN_from_local_to_interchange_key,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(enc(i(wk),W),enc(i(enc(i(tmk),V)),U))) ) )).

fof(data_translate_between_interchange_keys,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(enc(i(wk),W),enc(i(enc(i(wk),V)),U))) ) )).

%----Bond unsure if this command actually implemented in VSM
fof(data_translate_PIN_from_local_storage_to_interchange_key,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(enc(i(wk),V),enc(i(lp),U))) ) )).

fof(attacker_can_encrypt,axiom,(
    ! [U,V,W] :
      ( ( p(U)
        & p(V)
        & p(W) )
     => p(enc(U,V)) ) )).

%----Initial knowledge of intruder
fof(intruder_knows_1,axiom,(
    p(enc(tmk,pp)) )).

fof(intruder_knows_2,axiom,(
    p(enc(wk,w)) )).

fof(intruder_knows_3,axiom,(
    p(enc(w,t1)) )).

fof(intruder_knows_4,axiom,(
    p(enc(lp,t2)) )).

fof(intruder_knows_5,axiom,(
    p(enc(tc,k)) )).

fof(intruder_knows_6,axiom,(
    p(kk) )).

fof(intruder_knows_7,axiom,(
    p(i(kk)) )).

fof(intruder_knows_8,axiom,(
    p(a) )).

%----Goal for the attacker is to make a PIN (enc(pp,a))
fof(co1,conjecture,(
    p(enc(pp,a)) )).

%------------------------------------------------------------------------------
