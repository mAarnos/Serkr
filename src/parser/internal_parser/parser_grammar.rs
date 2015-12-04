#![allow(unused_imports)]
#![allow(unused_variables)]
use parser::formula::{Term, Formula};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Formula {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use parser::formula::{Term, Formula};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Formula<
        'input,
    >(
        input: &'input str,
    ) -> Result<Formula, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Formula(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        _28_3cTerm_3e_20_22_2c_22_29(Term),
        _28_3cTerm_3e_20_22_2c_22_29_2b(::std::vec::Vec<Term>),
        AndFormula(Formula),
        EquivalentFormula(Formula),
        Formula(Formula),
        Function(Term),
        ImpliesFormula(Formula),
        List_3cTerm_3e(Vec<Term>),
        NotFormula(Formula),
        OrFormula(Formula),
        Predicate(Formula),
        PredicateFormula(Formula),
        PredicateName(String),
        Term(Term),
        Term_3f(::std::option::Option<Term>),
        TermList(Vec<Term>),
        TermName(String),
        Variable(Term),
        ____Formula(Formula),
        ____Term(Term),
    }

    // State 0
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [EOF]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [EOF]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   EquivalentFormula = (*) EquivalentFormula "<=>" ImpliesFormula [EOF]
    //   EquivalentFormula = (*) EquivalentFormula "<=>" ImpliesFormula ["<=>"]
    //   EquivalentFormula = (*) ImpliesFormula [EOF]
    //   EquivalentFormula = (*) ImpliesFormula ["<=>"]
    //   Formula = (*) EquivalentFormula [EOF]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula [EOF]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["<=>"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["==>"]
    //   ImpliesFormula = (*) OrFormula [EOF]
    //   ImpliesFormula = (*) OrFormula ["<=>"]
    //   ImpliesFormula = (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [EOF]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [EOF]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //   __Formula = (*) Formula [EOF]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S1
    //   EquivalentFormula -> S2
    //   Formula -> S3
    //   Function -> S4
    //   ImpliesFormula -> S5
    //   NotFormula -> S6
    //   OrFormula -> S7
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::EquivalentFormula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Formula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::ImpliesFormula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [EOF]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   OrFormula = AndFormula (*) [EOF]
    //   OrFormula = AndFormula (*) ["<=>"]
    //   OrFormula = AndFormula (*) ["==>"]
    //   OrFormula = AndFormula (*) ["\\/"]
    //
    //   EOF -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   "<=>" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   "==>" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   "\\/" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   r#"/\\\\"# -> Shift(S20)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OrFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 2
    //   EquivalentFormula = EquivalentFormula (*) "<=>" ImpliesFormula [EOF]
    //   EquivalentFormula = EquivalentFormula (*) "<=>" ImpliesFormula ["<=>"]
    //   Formula = EquivalentFormula (*) [EOF]
    //
    //   EOF -> Reduce(Formula = EquivalentFormula => Call(ActionFn(2));)
    //   "<=>" -> Shift(S21)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Formula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 3
    //   __Formula = Formula (*) [EOF]
    //
    //   EOF -> Reduce(__Formula = Formula => Call(ActionFn(0));)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Formula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Term = Function (*) ["<>"]
    //   Term = Function (*) ["="]
    //
    //   "<>" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "=" -> Reduce(Term = Function => Call(ActionFn(21));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   EquivalentFormula = ImpliesFormula (*) [EOF]
    //   EquivalentFormula = ImpliesFormula (*) ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula [EOF]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["==>"]
    //
    //   EOF -> Reduce(EquivalentFormula = ImpliesFormula => Call(ActionFn(4));)
    //   "<=>" -> Reduce(EquivalentFormula = ImpliesFormula => Call(ActionFn(4));)
    //   "==>" -> Shift(S22)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::EquivalentFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 6
    //   AndFormula = NotFormula (*) [EOF]
    //   AndFormula = NotFormula (*) ["<=>"]
    //   AndFormula = NotFormula (*) ["==>"]
    //   AndFormula = NotFormula (*) ["\\/"]
    //   AndFormula = NotFormula (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   "<=>" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   "==>" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   "\\/" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   r#"/\\\\"# -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AndFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   ImpliesFormula = OrFormula (*) [EOF]
    //   ImpliesFormula = OrFormula (*) ["<=>"]
    //   ImpliesFormula = OrFormula (*) ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula [EOF]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["<=>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["\\/"]
    //
    //   EOF -> Reduce(ImpliesFormula = OrFormula => Call(ActionFn(6));)
    //   "<=>" -> Reduce(ImpliesFormula = OrFormula => Call(ActionFn(6));)
    //   "==>" -> Reduce(ImpliesFormula = OrFormula => Call(ActionFn(6));)
    //   "\\/" -> Shift(S23)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ImpliesFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 8
    //   PredicateFormula = Predicate (*) [EOF]
    //   PredicateFormula = Predicate (*) ["<=>"]
    //   PredicateFormula = Predicate (*) ["==>"]
    //   PredicateFormula = Predicate (*) ["\\/"]
    //   PredicateFormula = Predicate (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   "<=>" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   "==>" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   "\\/" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   NotFormula = PredicateFormula (*) [EOF]
    //   NotFormula = PredicateFormula (*) ["<=>"]
    //   NotFormula = PredicateFormula (*) ["==>"]
    //   NotFormula = PredicateFormula (*) ["\\/"]
    //   NotFormula = PredicateFormula (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   "<=>" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   "==>" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   "\\/" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   r#"/\\\\"# -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   Predicate = PredicateName (*) [EOF]
    //   Predicate = PredicateName (*) ["<=>"]
    //   Predicate = PredicateName (*) ["==>"]
    //   Predicate = PredicateName (*) ["\\/"]
    //   Predicate = PredicateName (*) [r#"/\\\\"#]
    //   Predicate = PredicateName (*) "(" TermList ")" [EOF]
    //   Predicate = PredicateName (*) "(" TermList ")" ["<=>"]
    //   Predicate = PredicateName (*) "(" TermList ")" ["==>"]
    //   Predicate = PredicateName (*) "(" TermList ")" ["\\/"]
    //   Predicate = PredicateName (*) "(" TermList ")" [r#"/\\\\"#]
    //
    //   EOF -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   "(" -> Shift(S24)
    //   "<=>" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   "==>" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   "\\/" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   r#"/\\\\"# -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state24(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Predicate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 11
    //   PredicateFormula = Term (*) "<>" Term [EOF]
    //   PredicateFormula = Term (*) "<>" Term ["<=>"]
    //   PredicateFormula = Term (*) "<>" Term ["==>"]
    //   PredicateFormula = Term (*) "<>" Term ["\\/"]
    //   PredicateFormula = Term (*) "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = Term (*) "=" Term [EOF]
    //   PredicateFormula = Term (*) "=" Term ["<=>"]
    //   PredicateFormula = Term (*) "=" Term ["==>"]
    //   PredicateFormula = Term (*) "=" Term ["\\/"]
    //   PredicateFormula = Term (*) "=" Term [r#"/\\\\"#]
    //
    //   "<>" -> Shift(S25)
    //   "=" -> Shift(S26)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state25(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state26(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Function = TermName (*) "(" TermList ")" ["<>"]
    //   Function = TermName (*) "(" TermList ")" ["="]
    //   Variable = TermName (*) ["<>"]
    //   Variable = TermName (*) ["="]
    //
    //   "(" -> Shift(S27)
    //   "<>" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "=" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 13
    //   Term = Variable (*) ["<>"]
    //   Term = Variable (*) ["="]
    //
    //   "<>" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "=" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [")"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [")"]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   EquivalentFormula = (*) EquivalentFormula "<=>" ImpliesFormula [")"]
    //   EquivalentFormula = (*) EquivalentFormula "<=>" ImpliesFormula ["<=>"]
    //   EquivalentFormula = (*) ImpliesFormula [")"]
    //   EquivalentFormula = (*) ImpliesFormula ["<=>"]
    //   Formula = (*) EquivalentFormula [")"]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula [")"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["<=>"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["==>"]
    //   ImpliesFormula = (*) OrFormula [")"]
    //   ImpliesFormula = (*) OrFormula ["<=>"]
    //   ImpliesFormula = (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [")"]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [")"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateFormula = "(" (*) Formula ")" [EOF]
    //   PredicateFormula = "(" (*) Formula ")" ["<=>"]
    //   PredicateFormula = "(" (*) Formula ")" ["==>"]
    //   PredicateFormula = "(" (*) Formula ")" ["\\/"]
    //   PredicateFormula = "(" (*) Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S28
    //   EquivalentFormula -> S29
    //   Formula -> S30
    //   Function -> S4
    //   ImpliesFormula -> S31
    //   NotFormula -> S32
    //   OrFormula -> S33
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::EquivalentFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Formula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::ImpliesFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state31(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state32(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   NotFormula = "exists" (*) TermName "." NotFormula [EOF]
    //   NotFormula = "exists" (*) TermName "." NotFormula ["<=>"]
    //   NotFormula = "exists" (*) TermName "." NotFormula ["==>"]
    //   NotFormula = "exists" (*) TermName "." NotFormula ["\\/"]
    //   NotFormula = "exists" (*) TermName "." NotFormula [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["."]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S44)
    //
    //   TermName -> S43
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state44(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state43(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   NotFormula = "forall" (*) TermName "." NotFormula [EOF]
    //   NotFormula = "forall" (*) TermName "." NotFormula ["<=>"]
    //   NotFormula = "forall" (*) TermName "." NotFormula ["==>"]
    //   NotFormula = "forall" (*) TermName "." NotFormula ["\\/"]
    //   NotFormula = "forall" (*) TermName "." NotFormula [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["."]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S44)
    //
    //   TermName -> S45
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state44(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state45(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   NotFormula = "~" (*) NotFormula [EOF]
    //   NotFormula = "~" (*) NotFormula ["<=>"]
    //   NotFormula = "~" (*) NotFormula ["==>"]
    //   NotFormula = "~" (*) NotFormula ["\\/"]
    //   NotFormula = "~" (*) NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S46
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state46(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) [EOF]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["("]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["<=>"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["==>"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["\\/"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "(" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "<=>" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "==>" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "\\/" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   r#"/\\\\"# -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action26(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["("]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["<>"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["="]
    //
    //   "(" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "<>" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "=" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula [EOF]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula ["<=>"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula ["==>"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula ["\\/"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula [r#"/\\\\"#]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S47
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [EOF]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [EOF]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   EquivalentFormula = EquivalentFormula "<=>" (*) ImpliesFormula [EOF]
    //   EquivalentFormula = EquivalentFormula "<=>" (*) ImpliesFormula ["<=>"]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula [EOF]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["<=>"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["==>"]
    //   ImpliesFormula = (*) OrFormula [EOF]
    //   ImpliesFormula = (*) OrFormula ["<=>"]
    //   ImpliesFormula = (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [EOF]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [EOF]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S1
    //   Function -> S4
    //   ImpliesFormula -> S48
    //   NotFormula -> S6
    //   OrFormula -> S7
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::ImpliesFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state48(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 22
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [EOF]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [EOF]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = ImpliesFormula "==>" (*) OrFormula [EOF]
    //   ImpliesFormula = ImpliesFormula "==>" (*) OrFormula ["<=>"]
    //   ImpliesFormula = ImpliesFormula "==>" (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [EOF]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [EOF]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S1
    //   Function -> S4
    //   NotFormula -> S6
    //   OrFormula -> S49
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state49(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [EOF]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [EOF]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = OrFormula "\\/" (*) AndFormula [EOF]
    //   OrFormula = OrFormula "\\/" (*) AndFormula ["<=>"]
    //   OrFormula = OrFormula "\\/" (*) AndFormula ["==>"]
    //   OrFormula = OrFormula "\\/" (*) AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S50
    //   Function -> S4
    //   NotFormula -> S6
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state50(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Predicate = PredicateName "(" (*) TermList ")" [EOF]
    //   Predicate = PredicateName "(" (*) TermList ")" ["<=>"]
    //   Predicate = PredicateName "(" (*) TermList ")" ["==>"]
    //   Predicate = PredicateName "(" (*) TermList ")" ["\\/"]
    //   Predicate = PredicateName "(" (*) TermList ")" [r#"/\\\\"#]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S51
    //   (<Term> ",")+ -> S52
    //   Function -> S53
    //   List<Term> -> S54
    //   Term -> S55
    //   Term? -> S56
    //   TermList -> S57
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state57(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Function = (*) TermName "(" TermList ")" [EOF]
    //   Function = (*) TermName "(" TermList ")" ["<=>"]
    //   Function = (*) TermName "(" TermList ")" ["==>"]
    //   Function = (*) TermName "(" TermList ")" ["\\/"]
    //   Function = (*) TermName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = Term "<>" (*) Term [EOF]
    //   PredicateFormula = Term "<>" (*) Term ["<=>"]
    //   PredicateFormula = Term "<>" (*) Term ["==>"]
    //   PredicateFormula = Term "<>" (*) Term ["\\/"]
    //   PredicateFormula = Term "<>" (*) Term [r#"/\\\\"#]
    //   Term = (*) Function [EOF]
    //   Term = (*) Function ["<=>"]
    //   Term = (*) Function ["==>"]
    //   Term = (*) Function ["\\/"]
    //   Term = (*) Function [r#"/\\\\"#]
    //   Term = (*) Variable [EOF]
    //   Term = (*) Variable ["<=>"]
    //   Term = (*) Variable ["==>"]
    //   Term = (*) Variable ["\\/"]
    //   Term = (*) Variable [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [EOF]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<=>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["==>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["\\/"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Variable = (*) TermName [EOF]
    //   Variable = (*) TermName ["<=>"]
    //   Variable = (*) TermName ["==>"]
    //   Variable = (*) TermName ["\\/"]
    //   Variable = (*) TermName [r#"/\\\\"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S65)
    //
    //   Function -> S61
    //   Term -> S62
    //   TermName -> S63
    //   Variable -> S64
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state65(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state61(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state62(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state63(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state64(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Function = (*) TermName "(" TermList ")" [EOF]
    //   Function = (*) TermName "(" TermList ")" ["<=>"]
    //   Function = (*) TermName "(" TermList ")" ["==>"]
    //   Function = (*) TermName "(" TermList ")" ["\\/"]
    //   Function = (*) TermName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = Term "=" (*) Term [EOF]
    //   PredicateFormula = Term "=" (*) Term ["<=>"]
    //   PredicateFormula = Term "=" (*) Term ["==>"]
    //   PredicateFormula = Term "=" (*) Term ["\\/"]
    //   PredicateFormula = Term "=" (*) Term [r#"/\\\\"#]
    //   Term = (*) Function [EOF]
    //   Term = (*) Function ["<=>"]
    //   Term = (*) Function ["==>"]
    //   Term = (*) Function ["\\/"]
    //   Term = (*) Function [r#"/\\\\"#]
    //   Term = (*) Variable [EOF]
    //   Term = (*) Variable ["<=>"]
    //   Term = (*) Variable ["==>"]
    //   Term = (*) Variable ["\\/"]
    //   Term = (*) Variable [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [EOF]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<=>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["==>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["\\/"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Variable = (*) TermName [EOF]
    //   Variable = (*) TermName ["<=>"]
    //   Variable = (*) TermName ["==>"]
    //   Variable = (*) TermName ["\\/"]
    //   Variable = (*) TermName [r#"/\\\\"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S65)
    //
    //   Function -> S61
    //   Term -> S66
    //   TermName -> S63
    //   Variable -> S64
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state65(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state61(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state66(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state63(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state64(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 27
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   Function = TermName "(" (*) TermList ")" ["<>"]
    //   Function = TermName "(" (*) TermList ")" ["="]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S51
    //   (<Term> ",")+ -> S52
    //   Function -> S53
    //   List<Term> -> S54
    //   Term -> S55
    //   Term? -> S56
    //   TermList -> S67
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state67(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [")"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   OrFormula = AndFormula (*) [")"]
    //   OrFormula = AndFormula (*) ["<=>"]
    //   OrFormula = AndFormula (*) ["==>"]
    //   OrFormula = AndFormula (*) ["\\/"]
    //
    //   ")" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   "<=>" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   "==>" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   "\\/" -> Reduce(OrFormula = AndFormula => Call(ActionFn(8));)
    //   r#"/\\\\"# -> Shift(S68)
    //
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state68(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OrFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 29
    //   EquivalentFormula = EquivalentFormula (*) "<=>" ImpliesFormula [")"]
    //   EquivalentFormula = EquivalentFormula (*) "<=>" ImpliesFormula ["<=>"]
    //   Formula = EquivalentFormula (*) [")"]
    //
    //   ")" -> Reduce(Formula = EquivalentFormula => Call(ActionFn(2));)
    //   "<=>" -> Shift(S69)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state69(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Formula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 30
    //   PredicateFormula = "(" Formula (*) ")" [EOF]
    //   PredicateFormula = "(" Formula (*) ")" ["<=>"]
    //   PredicateFormula = "(" Formula (*) ")" ["==>"]
    //   PredicateFormula = "(" Formula (*) ")" ["\\/"]
    //   PredicateFormula = "(" Formula (*) ")" [r#"/\\\\"#]
    //
    //   ")" -> Shift(S70)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state70(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 31
    //   EquivalentFormula = ImpliesFormula (*) [")"]
    //   EquivalentFormula = ImpliesFormula (*) ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula [")"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["==>"]
    //
    //   ")" -> Reduce(EquivalentFormula = ImpliesFormula => Call(ActionFn(4));)
    //   "<=>" -> Reduce(EquivalentFormula = ImpliesFormula => Call(ActionFn(4));)
    //   "==>" -> Shift(S71)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state71(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::EquivalentFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 32
    //   AndFormula = NotFormula (*) [")"]
    //   AndFormula = NotFormula (*) ["<=>"]
    //   AndFormula = NotFormula (*) ["==>"]
    //   AndFormula = NotFormula (*) ["\\/"]
    //   AndFormula = NotFormula (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   "<=>" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   "==>" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   "\\/" -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //   r#"/\\\\"# -> Reduce(AndFormula = NotFormula => Call(ActionFn(10));)
    //
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AndFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 33
    //   ImpliesFormula = OrFormula (*) [")"]
    //   ImpliesFormula = OrFormula (*) ["<=>"]
    //   ImpliesFormula = OrFormula (*) ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula [")"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["<=>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["\\/"]
    //
    //   ")" -> Reduce(ImpliesFormula = OrFormula => Call(ActionFn(6));)
    //   "<=>" -> Reduce(ImpliesFormula = OrFormula => Call(ActionFn(6));)
    //   "==>" -> Reduce(ImpliesFormula = OrFormula => Call(ActionFn(6));)
    //   "\\/" -> Shift(S72)
    //
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state72(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ImpliesFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 34
    //   PredicateFormula = Predicate (*) [")"]
    //   PredicateFormula = Predicate (*) ["<=>"]
    //   PredicateFormula = Predicate (*) ["==>"]
    //   PredicateFormula = Predicate (*) ["\\/"]
    //   PredicateFormula = Predicate (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   "<=>" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   "==>" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   "\\/" -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = Predicate => Call(ActionFn(17));)
    //
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 35
    //   NotFormula = PredicateFormula (*) [")"]
    //   NotFormula = PredicateFormula (*) ["<=>"]
    //   NotFormula = PredicateFormula (*) ["==>"]
    //   NotFormula = PredicateFormula (*) ["\\/"]
    //   NotFormula = PredicateFormula (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   "<=>" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   "==>" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   "\\/" -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //   r#"/\\\\"# -> Reduce(NotFormula = PredicateFormula => Call(ActionFn(14));)
    //
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 36
    //   Predicate = PredicateName (*) [")"]
    //   Predicate = PredicateName (*) ["<=>"]
    //   Predicate = PredicateName (*) ["==>"]
    //   Predicate = PredicateName (*) ["\\/"]
    //   Predicate = PredicateName (*) [r#"/\\\\"#]
    //   Predicate = PredicateName (*) "(" TermList ")" [")"]
    //   Predicate = PredicateName (*) "(" TermList ")" ["<=>"]
    //   Predicate = PredicateName (*) "(" TermList ")" ["==>"]
    //   Predicate = PredicateName (*) "(" TermList ")" ["\\/"]
    //   Predicate = PredicateName (*) "(" TermList ")" [r#"/\\\\"#]
    //
    //   "(" -> Shift(S73)
    //   ")" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   "<=>" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   "==>" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   "\\/" -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //   r#"/\\\\"# -> Reduce(Predicate = PredicateName => Call(ActionFn(20));)
    //
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state73(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Predicate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 37
    //   PredicateFormula = Term (*) "<>" Term [")"]
    //   PredicateFormula = Term (*) "<>" Term ["<=>"]
    //   PredicateFormula = Term (*) "<>" Term ["==>"]
    //   PredicateFormula = Term (*) "<>" Term ["\\/"]
    //   PredicateFormula = Term (*) "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = Term (*) "=" Term [")"]
    //   PredicateFormula = Term (*) "=" Term ["<=>"]
    //   PredicateFormula = Term (*) "=" Term ["==>"]
    //   PredicateFormula = Term (*) "=" Term ["\\/"]
    //   PredicateFormula = Term (*) "=" Term [r#"/\\\\"#]
    //
    //   "<>" -> Shift(S74)
    //   "=" -> Shift(S75)
    //
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state74(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state75(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 38
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [")"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [")"]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   EquivalentFormula = (*) EquivalentFormula "<=>" ImpliesFormula [")"]
    //   EquivalentFormula = (*) EquivalentFormula "<=>" ImpliesFormula ["<=>"]
    //   EquivalentFormula = (*) ImpliesFormula [")"]
    //   EquivalentFormula = (*) ImpliesFormula ["<=>"]
    //   Formula = (*) EquivalentFormula [")"]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula [")"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["<=>"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["==>"]
    //   ImpliesFormula = (*) OrFormula [")"]
    //   ImpliesFormula = (*) OrFormula ["<=>"]
    //   ImpliesFormula = (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [")"]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [")"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateFormula = "(" (*) Formula ")" [")"]
    //   PredicateFormula = "(" (*) Formula ")" ["<=>"]
    //   PredicateFormula = "(" (*) Formula ")" ["==>"]
    //   PredicateFormula = "(" (*) Formula ")" ["\\/"]
    //   PredicateFormula = "(" (*) Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S28
    //   EquivalentFormula -> S29
    //   Formula -> S76
    //   Function -> S4
    //   ImpliesFormula -> S31
    //   NotFormula -> S32
    //   OrFormula -> S33
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::EquivalentFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Formula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state76(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::ImpliesFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state31(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state32(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 39
    //   NotFormula = "exists" (*) TermName "." NotFormula [")"]
    //   NotFormula = "exists" (*) TermName "." NotFormula ["<=>"]
    //   NotFormula = "exists" (*) TermName "." NotFormula ["==>"]
    //   NotFormula = "exists" (*) TermName "." NotFormula ["\\/"]
    //   NotFormula = "exists" (*) TermName "." NotFormula [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["."]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S44)
    //
    //   TermName -> S77
    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state44(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state77(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 40
    //   NotFormula = "forall" (*) TermName "." NotFormula [")"]
    //   NotFormula = "forall" (*) TermName "." NotFormula ["<=>"]
    //   NotFormula = "forall" (*) TermName "." NotFormula ["==>"]
    //   NotFormula = "forall" (*) TermName "." NotFormula ["\\/"]
    //   NotFormula = "forall" (*) TermName "." NotFormula [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["."]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S44)
    //
    //   TermName -> S78
    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state44(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state78(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 41
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   NotFormula = "~" (*) NotFormula [")"]
    //   NotFormula = "~" (*) NotFormula ["<=>"]
    //   NotFormula = "~" (*) NotFormula ["==>"]
    //   NotFormula = "~" (*) NotFormula ["\\/"]
    //   NotFormula = "~" (*) NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S79
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state79(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 42
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["("]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) [")"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["<=>"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["==>"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) ["\\/"]
    //   PredicateName = r#"[A-Z][a-zA-Z0-9]*"# (*) [r#"/\\\\"#]
    //
    //   "(" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   ")" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "<=>" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "==>" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   "\\/" -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //   r#"/\\\\"# -> Reduce(PredicateName = r#"[A-Z][a-zA-Z0-9]*"# => Call(ActionFn(26));)
    //
    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action26(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 43
    //   NotFormula = "exists" TermName (*) "." NotFormula [EOF]
    //   NotFormula = "exists" TermName (*) "." NotFormula ["<=>"]
    //   NotFormula = "exists" TermName (*) "." NotFormula ["==>"]
    //   NotFormula = "exists" TermName (*) "." NotFormula ["\\/"]
    //   NotFormula = "exists" TermName (*) "." NotFormula [r#"/\\\\"#]
    //
    //   "." -> Shift(S80)
    //
    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state80(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 44
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["."]
    //
    //   "." -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 45
    //   NotFormula = "forall" TermName (*) "." NotFormula [EOF]
    //   NotFormula = "forall" TermName (*) "." NotFormula ["<=>"]
    //   NotFormula = "forall" TermName (*) "." NotFormula ["==>"]
    //   NotFormula = "forall" TermName (*) "." NotFormula ["\\/"]
    //   NotFormula = "forall" TermName (*) "." NotFormula [r#"/\\\\"#]
    //
    //   "." -> Shift(S81)
    //
    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state81(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 46
    //   NotFormula = "~" NotFormula (*) [EOF]
    //   NotFormula = "~" NotFormula (*) ["<=>"]
    //   NotFormula = "~" NotFormula (*) ["==>"]
    //   NotFormula = "~" NotFormula (*) ["\\/"]
    //   NotFormula = "~" NotFormula (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   "<=>" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   "==>" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   "\\/" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   r#"/\\\\"# -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //
    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action11(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 47
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) [EOF]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) ["<=>"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) ["==>"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) ["\\/"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   "<=>" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   "==>" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   "\\/" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   r#"/\\\\"# -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //
    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AndFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 48
    //   EquivalentFormula = EquivalentFormula "<=>" ImpliesFormula (*) [EOF]
    //   EquivalentFormula = EquivalentFormula "<=>" ImpliesFormula (*) ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula [EOF]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["==>"]
    //
    //   EOF -> Reduce(EquivalentFormula = EquivalentFormula, "<=>", ImpliesFormula => Call(ActionFn(3));)
    //   "<=>" -> Reduce(EquivalentFormula = EquivalentFormula, "<=>", ImpliesFormula => Call(ActionFn(3));)
    //   "==>" -> Shift(S22)
    //
    pub fn __state48<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::EquivalentFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 49
    //   ImpliesFormula = ImpliesFormula "==>" OrFormula (*) [EOF]
    //   ImpliesFormula = ImpliesFormula "==>" OrFormula (*) ["<=>"]
    //   ImpliesFormula = ImpliesFormula "==>" OrFormula (*) ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula [EOF]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["<=>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["\\/"]
    //
    //   EOF -> Reduce(ImpliesFormula = ImpliesFormula, "==>", OrFormula => Call(ActionFn(5));)
    //   "<=>" -> Reduce(ImpliesFormula = ImpliesFormula, "==>", OrFormula => Call(ActionFn(5));)
    //   "==>" -> Reduce(ImpliesFormula = ImpliesFormula, "==>", OrFormula => Call(ActionFn(5));)
    //   "\\/" -> Shift(S23)
    //
    pub fn __state49<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ImpliesFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 50
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [EOF]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   OrFormula = OrFormula "\\/" AndFormula (*) [EOF]
    //   OrFormula = OrFormula "\\/" AndFormula (*) ["<=>"]
    //   OrFormula = OrFormula "\\/" AndFormula (*) ["==>"]
    //   OrFormula = OrFormula "\\/" AndFormula (*) ["\\/"]
    //
    //   EOF -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   "<=>" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   "==>" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   "\\/" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   r#"/\\\\"# -> Shift(S20)
    //
    pub fn __state50<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OrFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 51
    //   (<Term> ",")+ = (<Term> ",") (*) [r#"[a-z][a-zA-Z0-9]*"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Reduce((<Term> ",")+ = (<Term> ",") => Call(ActionFn(32));)
    //
    pub fn __state51<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action32(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 52
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (<Term> ",")+ (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   List<Term> = (<Term> ",")+ (*) Term [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S82
    //   Function -> S53
    //   Term -> S83
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state52<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state82(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state83(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 53
    //   Term = Function (*) [")"]
    //   Term = Function (*) [","]
    //
    //   ")" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "," -> Reduce(Term = Function => Call(ActionFn(21));)
    //
    pub fn __state53<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 54
    //   TermList = List<Term> (*) [")"]
    //
    //   ")" -> Reduce(TermList = List<Term> => Call(ActionFn(24));)
    //
    pub fn __state54<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action24(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermList(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 55
    //   (<Term> ",") = Term (*) "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   Term? = Term (*) [")"]
    //
    //   ")" -> Reduce(Term? = Term => Call(ActionFn(30));)
    //   "," -> Shift(S84)
    //
    pub fn __state55<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state84(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action30(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 56
    //   List<Term> = Term? (*) [")"]
    //
    //   ")" -> Reduce(List<Term> = Term? => Call(ActionFn(29));)
    //
    pub fn __state56<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::option::Option<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action29(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::List_3cTerm_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 57
    //   Predicate = PredicateName "(" TermList (*) ")" [EOF]
    //   Predicate = PredicateName "(" TermList (*) ")" ["<=>"]
    //   Predicate = PredicateName "(" TermList (*) ")" ["==>"]
    //   Predicate = PredicateName "(" TermList (*) ")" ["\\/"]
    //   Predicate = PredicateName "(" TermList (*) ")" [r#"/\\\\"#]
    //
    //   ")" -> Shift(S85)
    //
    pub fn __state57<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state85(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 58
    //   Function = TermName (*) "(" TermList ")" [")"]
    //   Function = TermName (*) "(" TermList ")" [","]
    //   Variable = TermName (*) [")"]
    //   Variable = TermName (*) [","]
    //
    //   "(" -> Shift(S86)
    //   ")" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "," -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //
    pub fn __state58<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state86(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 59
    //   Term = Variable (*) [")"]
    //   Term = Variable (*) [","]
    //
    //   ")" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "," -> Reduce(Term = Variable => Call(ActionFn(22));)
    //
    pub fn __state59<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 60
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["("]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [")"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [","]
    //
    //   "(" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   ")" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "," -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state60<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 61
    //   Term = Function (*) [EOF]
    //   Term = Function (*) ["<=>"]
    //   Term = Function (*) ["==>"]
    //   Term = Function (*) ["\\/"]
    //   Term = Function (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "<=>" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "==>" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "\\/" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   r#"/\\\\"# -> Reduce(Term = Function => Call(ActionFn(21));)
    //
    pub fn __state61<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 62
    //   PredicateFormula = Term "<>" Term (*) [EOF]
    //   PredicateFormula = Term "<>" Term (*) ["<=>"]
    //   PredicateFormula = Term "<>" Term (*) ["==>"]
    //   PredicateFormula = Term "<>" Term (*) ["\\/"]
    //   PredicateFormula = Term "<>" Term (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   "<=>" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   "==>" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   "\\/" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //
    pub fn __state62<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action16(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 63
    //   Function = TermName (*) "(" TermList ")" [EOF]
    //   Function = TermName (*) "(" TermList ")" ["<=>"]
    //   Function = TermName (*) "(" TermList ")" ["==>"]
    //   Function = TermName (*) "(" TermList ")" ["\\/"]
    //   Function = TermName (*) "(" TermList ")" [r#"/\\\\"#]
    //   Variable = TermName (*) [EOF]
    //   Variable = TermName (*) ["<=>"]
    //   Variable = TermName (*) ["==>"]
    //   Variable = TermName (*) ["\\/"]
    //   Variable = TermName (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "(" -> Shift(S87)
    //   "<=>" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "==>" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "\\/" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   r#"/\\\\"# -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //
    pub fn __state63<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state87(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 64
    //   Term = Variable (*) [EOF]
    //   Term = Variable (*) ["<=>"]
    //   Term = Variable (*) ["==>"]
    //   Term = Variable (*) ["\\/"]
    //   Term = Variable (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "<=>" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "==>" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "\\/" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   r#"/\\\\"# -> Reduce(Term = Variable => Call(ActionFn(22));)
    //
    pub fn __state64<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 65
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [EOF]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["("]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["<=>"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["==>"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["\\/"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "(" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "<=>" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "==>" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "\\/" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   r#"/\\\\"# -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state65<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 66
    //   PredicateFormula = Term "=" Term (*) [EOF]
    //   PredicateFormula = Term "=" Term (*) ["<=>"]
    //   PredicateFormula = Term "=" Term (*) ["==>"]
    //   PredicateFormula = Term "=" Term (*) ["\\/"]
    //   PredicateFormula = Term "=" Term (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   "<=>" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   "==>" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   "\\/" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //
    pub fn __state66<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 67
    //   Function = TermName "(" TermList (*) ")" ["<>"]
    //   Function = TermName "(" TermList (*) ")" ["="]
    //
    //   ")" -> Shift(S88)
    //
    pub fn __state67<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state88(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 68
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula [")"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula ["<=>"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula ["==>"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula ["\\/"]
    //   AndFormula = AndFormula r#"/\\\\"# (*) NotFormula [r#"/\\\\"#]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S89
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state68<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state89(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 69
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [")"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [")"]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   EquivalentFormula = EquivalentFormula "<=>" (*) ImpliesFormula [")"]
    //   EquivalentFormula = EquivalentFormula "<=>" (*) ImpliesFormula ["<=>"]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula [")"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["<=>"]
    //   ImpliesFormula = (*) ImpliesFormula "==>" OrFormula ["==>"]
    //   ImpliesFormula = (*) OrFormula [")"]
    //   ImpliesFormula = (*) OrFormula ["<=>"]
    //   ImpliesFormula = (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [")"]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [")"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S28
    //   Function -> S4
    //   ImpliesFormula -> S90
    //   NotFormula -> S32
    //   OrFormula -> S33
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state69<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::ImpliesFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state90(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 70
    //   PredicateFormula = "(" Formula ")" (*) [EOF]
    //   PredicateFormula = "(" Formula ")" (*) ["<=>"]
    //   PredicateFormula = "(" Formula ")" (*) ["==>"]
    //   PredicateFormula = "(" Formula ")" (*) ["\\/"]
    //   PredicateFormula = "(" Formula ")" (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   "<=>" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   "==>" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   "\\/" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //
    pub fn __state70<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Formula>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action18(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 71
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [")"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [")"]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   ImpliesFormula = ImpliesFormula "==>" (*) OrFormula [")"]
    //   ImpliesFormula = ImpliesFormula "==>" (*) OrFormula ["<=>"]
    //   ImpliesFormula = ImpliesFormula "==>" (*) OrFormula ["==>"]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = (*) AndFormula [")"]
    //   OrFormula = (*) AndFormula ["<=>"]
    //   OrFormula = (*) AndFormula ["==>"]
    //   OrFormula = (*) AndFormula ["\\/"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula [")"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["<=>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["==>"]
    //   OrFormula = (*) OrFormula "\\/" AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S28
    //   Function -> S4
    //   NotFormula -> S32
    //   OrFormula -> S91
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state71<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::OrFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state91(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 72
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [")"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = (*) AndFormula r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   AndFormula = (*) NotFormula [")"]
    //   AndFormula = (*) NotFormula ["<=>"]
    //   AndFormula = (*) NotFormula ["==>"]
    //   AndFormula = (*) NotFormula ["\\/"]
    //   AndFormula = (*) NotFormula [r#"/\\\\"#]
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   OrFormula = OrFormula "\\/" (*) AndFormula [")"]
    //   OrFormula = OrFormula "\\/" (*) AndFormula ["<=>"]
    //   OrFormula = OrFormula "\\/" (*) AndFormula ["==>"]
    //   OrFormula = OrFormula "\\/" (*) AndFormula ["\\/"]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   AndFormula -> S92
    //   Function -> S4
    //   NotFormula -> S32
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state72<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::AndFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state92(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state32(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 73
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Predicate = PredicateName "(" (*) TermList ")" [")"]
    //   Predicate = PredicateName "(" (*) TermList ")" ["<=>"]
    //   Predicate = PredicateName "(" (*) TermList ")" ["==>"]
    //   Predicate = PredicateName "(" (*) TermList ")" ["\\/"]
    //   Predicate = PredicateName "(" (*) TermList ")" [r#"/\\\\"#]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S51
    //   (<Term> ",")+ -> S52
    //   Function -> S53
    //   List<Term> -> S54
    //   Term -> S55
    //   Term? -> S56
    //   TermList -> S93
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state73<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state93(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 74
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" ["<=>"]
    //   Function = (*) TermName "(" TermList ")" ["==>"]
    //   Function = (*) TermName "(" TermList ")" ["\\/"]
    //   Function = (*) TermName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = Term "<>" (*) Term [")"]
    //   PredicateFormula = Term "<>" (*) Term ["<=>"]
    //   PredicateFormula = Term "<>" (*) Term ["==>"]
    //   PredicateFormula = Term "<>" (*) Term ["\\/"]
    //   PredicateFormula = Term "<>" (*) Term [r#"/\\\\"#]
    //   Term = (*) Function [")"]
    //   Term = (*) Function ["<=>"]
    //   Term = (*) Function ["==>"]
    //   Term = (*) Function ["\\/"]
    //   Term = (*) Function [r#"/\\\\"#]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable ["<=>"]
    //   Term = (*) Variable ["==>"]
    //   Term = (*) Variable ["\\/"]
    //   Term = (*) Variable [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<=>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["==>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["\\/"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName ["<=>"]
    //   Variable = (*) TermName ["==>"]
    //   Variable = (*) TermName ["\\/"]
    //   Variable = (*) TermName [r#"/\\\\"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S98)
    //
    //   Function -> S94
    //   Term -> S95
    //   TermName -> S96
    //   Variable -> S97
    pub fn __state74<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state98(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state94(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state95(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state96(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state97(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 75
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" ["<=>"]
    //   Function = (*) TermName "(" TermList ")" ["==>"]
    //   Function = (*) TermName "(" TermList ")" ["\\/"]
    //   Function = (*) TermName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = Term "=" (*) Term [")"]
    //   PredicateFormula = Term "=" (*) Term ["<=>"]
    //   PredicateFormula = Term "=" (*) Term ["==>"]
    //   PredicateFormula = Term "=" (*) Term ["\\/"]
    //   PredicateFormula = Term "=" (*) Term [r#"/\\\\"#]
    //   Term = (*) Function [")"]
    //   Term = (*) Function ["<=>"]
    //   Term = (*) Function ["==>"]
    //   Term = (*) Function ["\\/"]
    //   Term = (*) Function [r#"/\\\\"#]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable ["<=>"]
    //   Term = (*) Variable ["==>"]
    //   Term = (*) Variable ["\\/"]
    //   Term = (*) Variable [r#"/\\\\"#]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<=>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["==>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["\\/"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName ["<=>"]
    //   Variable = (*) TermName ["==>"]
    //   Variable = (*) TermName ["\\/"]
    //   Variable = (*) TermName [r#"/\\\\"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S98)
    //
    //   Function -> S94
    //   Term -> S99
    //   TermName -> S96
    //   Variable -> S97
    pub fn __state75<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state98(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state94(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state99(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state96(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state97(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 76
    //   PredicateFormula = "(" Formula (*) ")" [")"]
    //   PredicateFormula = "(" Formula (*) ")" ["<=>"]
    //   PredicateFormula = "(" Formula (*) ")" ["==>"]
    //   PredicateFormula = "(" Formula (*) ")" ["\\/"]
    //   PredicateFormula = "(" Formula (*) ")" [r#"/\\\\"#]
    //
    //   ")" -> Shift(S100)
    //
    pub fn __state76<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state100(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 77
    //   NotFormula = "exists" TermName (*) "." NotFormula [")"]
    //   NotFormula = "exists" TermName (*) "." NotFormula ["<=>"]
    //   NotFormula = "exists" TermName (*) "." NotFormula ["==>"]
    //   NotFormula = "exists" TermName (*) "." NotFormula ["\\/"]
    //   NotFormula = "exists" TermName (*) "." NotFormula [r#"/\\\\"#]
    //
    //   "." -> Shift(S101)
    //
    pub fn __state77<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state101(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 78
    //   NotFormula = "forall" TermName (*) "." NotFormula [")"]
    //   NotFormula = "forall" TermName (*) "." NotFormula ["<=>"]
    //   NotFormula = "forall" TermName (*) "." NotFormula ["==>"]
    //   NotFormula = "forall" TermName (*) "." NotFormula ["\\/"]
    //   NotFormula = "forall" TermName (*) "." NotFormula [r#"/\\\\"#]
    //
    //   "." -> Shift(S102)
    //
    pub fn __state78<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state102(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 79
    //   NotFormula = "~" NotFormula (*) [")"]
    //   NotFormula = "~" NotFormula (*) ["<=>"]
    //   NotFormula = "~" NotFormula (*) ["==>"]
    //   NotFormula = "~" NotFormula (*) ["\\/"]
    //   NotFormula = "~" NotFormula (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   "<=>" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   "==>" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   "\\/" -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //   r#"/\\\\"# -> Reduce(NotFormula = "~", NotFormula => Call(ActionFn(11));)
    //
    pub fn __state79<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action11(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 80
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = "exists" TermName "." (*) NotFormula [EOF]
    //   NotFormula = "exists" TermName "." (*) NotFormula ["<=>"]
    //   NotFormula = "exists" TermName "." (*) NotFormula ["==>"]
    //   NotFormula = "exists" TermName "." (*) NotFormula ["\\/"]
    //   NotFormula = "exists" TermName "." (*) NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S103
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state80<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state103(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 81
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [EOF]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [EOF]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = "forall" TermName "." (*) NotFormula [EOF]
    //   NotFormula = "forall" TermName "." (*) NotFormula ["<=>"]
    //   NotFormula = "forall" TermName "." (*) NotFormula ["==>"]
    //   NotFormula = "forall" TermName "." (*) NotFormula ["\\/"]
    //   NotFormula = "forall" TermName "." (*) NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [EOF]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [EOF]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [EOF]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [EOF]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [EOF]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [EOF]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [EOF]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [EOF]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S14)
    //   "exists" -> Shift(S15)
    //   "forall" -> Shift(S16)
    //   "~" -> Shift(S17)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S18)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S104
    //   Predicate -> S8
    //   PredicateFormula -> S9
    //   PredicateName -> S10
    //   Term -> S11
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state81<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state104(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 82
    //   (<Term> ",")+ = (<Term> ",")+ (<Term> ",") (*) [r#"[a-z][a-zA-Z0-9]*"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Reduce((<Term> ",")+ = (<Term> ",")+, (<Term> ",") => Call(ActionFn(33));)
    //
    pub fn __state82<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Term>>,
        __sym1: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 83
    //   (<Term> ",") = Term (*) "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   List<Term> = (<Term> ",")+ Term (*) [")"]
    //
    //   ")" -> Reduce(List<Term> = (<Term> ",")+, Term => Call(ActionFn(28));)
    //   "," -> Shift(S84)
    //
    pub fn __state83<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Term>>,
        __sym1: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state84(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action28(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::List_3cTerm_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 84
    //   (<Term> ",") = Term "," (*) [r#"[a-z][a-zA-Z0-9]*"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Reduce((<Term> ",") = Term, "," => Call(ActionFn(34));)
    //
    pub fn __state84<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action34(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 85
    //   Predicate = PredicateName "(" TermList ")" (*) [EOF]
    //   Predicate = PredicateName "(" TermList ")" (*) ["<=>"]
    //   Predicate = PredicateName "(" TermList ")" (*) ["==>"]
    //   Predicate = PredicateName "(" TermList ")" (*) ["\\/"]
    //   Predicate = PredicateName "(" TermList ")" (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   "<=>" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   "==>" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   "\\/" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   r#"/\\\\"# -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //
    pub fn __state85<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action19(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Predicate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 86
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   Function = TermName "(" (*) TermList ")" [")"]
    //   Function = TermName "(" (*) TermList ")" [","]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S51
    //   (<Term> ",")+ -> S52
    //   Function -> S53
    //   List<Term> -> S54
    //   Term -> S55
    //   Term? -> S56
    //   TermList -> S105
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state86<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state105(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 87
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   Function = TermName "(" (*) TermList ")" [EOF]
    //   Function = TermName "(" (*) TermList ")" ["<=>"]
    //   Function = TermName "(" (*) TermList ")" ["==>"]
    //   Function = TermName "(" (*) TermList ")" ["\\/"]
    //   Function = TermName "(" (*) TermList ")" [r#"/\\\\"#]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S51
    //   (<Term> ",")+ -> S52
    //   Function -> S53
    //   List<Term> -> S54
    //   Term -> S55
    //   Term? -> S56
    //   TermList -> S106
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state87<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state106(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 88
    //   Function = TermName "(" TermList ")" (*) ["<>"]
    //   Function = TermName "(" TermList ")" (*) ["="]
    //
    //   "<>" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "=" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //
    pub fn __state88<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Function(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 89
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) [")"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) ["<=>"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) ["==>"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) ["\\/"]
    //   AndFormula = AndFormula r#"/\\\\"# NotFormula (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   "<=>" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   "==>" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   "\\/" -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //   r#"/\\\\"# -> Reduce(AndFormula = AndFormula, r#"/\\\\"#, NotFormula => Call(ActionFn(9));)
    //
    pub fn __state89<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::AndFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 90
    //   EquivalentFormula = EquivalentFormula "<=>" ImpliesFormula (*) [")"]
    //   EquivalentFormula = EquivalentFormula "<=>" ImpliesFormula (*) ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula [")"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["<=>"]
    //   ImpliesFormula = ImpliesFormula (*) "==>" OrFormula ["==>"]
    //
    //   ")" -> Reduce(EquivalentFormula = EquivalentFormula, "<=>", ImpliesFormula => Call(ActionFn(3));)
    //   "<=>" -> Reduce(EquivalentFormula = EquivalentFormula, "<=>", ImpliesFormula => Call(ActionFn(3));)
    //   "==>" -> Shift(S71)
    //
    pub fn __state90<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state71(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::EquivalentFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 91
    //   ImpliesFormula = ImpliesFormula "==>" OrFormula (*) [")"]
    //   ImpliesFormula = ImpliesFormula "==>" OrFormula (*) ["<=>"]
    //   ImpliesFormula = ImpliesFormula "==>" OrFormula (*) ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula [")"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["<=>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["==>"]
    //   OrFormula = OrFormula (*) "\\/" AndFormula ["\\/"]
    //
    //   ")" -> Reduce(ImpliesFormula = ImpliesFormula, "==>", OrFormula => Call(ActionFn(5));)
    //   "<=>" -> Reduce(ImpliesFormula = ImpliesFormula, "==>", OrFormula => Call(ActionFn(5));)
    //   "==>" -> Reduce(ImpliesFormula = ImpliesFormula, "==>", OrFormula => Call(ActionFn(5));)
    //   "\\/" -> Shift(S72)
    //
    pub fn __state91<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state72(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::ImpliesFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 92
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [")"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["<=>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["==>"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula ["\\/"]
    //   AndFormula = AndFormula (*) r#"/\\\\"# NotFormula [r#"/\\\\"#]
    //   OrFormula = OrFormula "\\/" AndFormula (*) [")"]
    //   OrFormula = OrFormula "\\/" AndFormula (*) ["<=>"]
    //   OrFormula = OrFormula "\\/" AndFormula (*) ["==>"]
    //   OrFormula = OrFormula "\\/" AndFormula (*) ["\\/"]
    //
    //   ")" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   "<=>" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   "==>" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   "\\/" -> Reduce(OrFormula = OrFormula, "\\/", AndFormula => Call(ActionFn(7));)
    //   r#"/\\\\"# -> Shift(S68)
    //
    pub fn __state92<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Formula>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state68(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action7(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::OrFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 93
    //   Predicate = PredicateName "(" TermList (*) ")" [")"]
    //   Predicate = PredicateName "(" TermList (*) ")" ["<=>"]
    //   Predicate = PredicateName "(" TermList (*) ")" ["==>"]
    //   Predicate = PredicateName "(" TermList (*) ")" ["\\/"]
    //   Predicate = PredicateName "(" TermList (*) ")" [r#"/\\\\"#]
    //
    //   ")" -> Shift(S107)
    //
    pub fn __state93<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state107(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 94
    //   Term = Function (*) [")"]
    //   Term = Function (*) ["<=>"]
    //   Term = Function (*) ["==>"]
    //   Term = Function (*) ["\\/"]
    //   Term = Function (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "<=>" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "==>" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "\\/" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   r#"/\\\\"# -> Reduce(Term = Function => Call(ActionFn(21));)
    //
    pub fn __state94<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 95
    //   PredicateFormula = Term "<>" Term (*) [")"]
    //   PredicateFormula = Term "<>" Term (*) ["<=>"]
    //   PredicateFormula = Term "<>" Term (*) ["==>"]
    //   PredicateFormula = Term "<>" Term (*) ["\\/"]
    //   PredicateFormula = Term "<>" Term (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   "<=>" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   "==>" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   "\\/" -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = Term, "<>", Term => Call(ActionFn(16));)
    //
    pub fn __state95<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action16(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 96
    //   Function = TermName (*) "(" TermList ")" [")"]
    //   Function = TermName (*) "(" TermList ")" ["<=>"]
    //   Function = TermName (*) "(" TermList ")" ["==>"]
    //   Function = TermName (*) "(" TermList ")" ["\\/"]
    //   Function = TermName (*) "(" TermList ")" [r#"/\\\\"#]
    //   Variable = TermName (*) [")"]
    //   Variable = TermName (*) ["<=>"]
    //   Variable = TermName (*) ["==>"]
    //   Variable = TermName (*) ["\\/"]
    //   Variable = TermName (*) [r#"/\\\\"#]
    //
    //   "(" -> Shift(S108)
    //   ")" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "<=>" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "==>" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "\\/" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   r#"/\\\\"# -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //
    pub fn __state96<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state108(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 97
    //   Term = Variable (*) [")"]
    //   Term = Variable (*) ["<=>"]
    //   Term = Variable (*) ["==>"]
    //   Term = Variable (*) ["\\/"]
    //   Term = Variable (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "<=>" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "==>" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "\\/" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   r#"/\\\\"# -> Reduce(Term = Variable => Call(ActionFn(22));)
    //
    pub fn __state97<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 98
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["("]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [")"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["<=>"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["==>"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["\\/"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [r#"/\\\\"#]
    //
    //   "(" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   ")" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "<=>" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "==>" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "\\/" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   r#"/\\\\"# -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state98<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 99
    //   PredicateFormula = Term "=" Term (*) [")"]
    //   PredicateFormula = Term "=" Term (*) ["<=>"]
    //   PredicateFormula = Term "=" Term (*) ["==>"]
    //   PredicateFormula = Term "=" Term (*) ["\\/"]
    //   PredicateFormula = Term "=" Term (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   "<=>" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   "==>" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   "\\/" -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = Term, "=", Term => Call(ActionFn(15));)
    //
    pub fn __state99<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action15(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 100
    //   PredicateFormula = "(" Formula ")" (*) [")"]
    //   PredicateFormula = "(" Formula ")" (*) ["<=>"]
    //   PredicateFormula = "(" Formula ")" (*) ["==>"]
    //   PredicateFormula = "(" Formula ")" (*) ["\\/"]
    //   PredicateFormula = "(" Formula ")" (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   "<=>" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   "==>" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   "\\/" -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //   r#"/\\\\"# -> Reduce(PredicateFormula = "(", Formula, ")" => Call(ActionFn(18));)
    //
    pub fn __state100<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Formula>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action18(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::PredicateFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 101
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = "exists" TermName "." (*) NotFormula [")"]
    //   NotFormula = "exists" TermName "." (*) NotFormula ["<=>"]
    //   NotFormula = "exists" TermName "." (*) NotFormula ["==>"]
    //   NotFormula = "exists" TermName "." (*) NotFormula ["\\/"]
    //   NotFormula = "exists" TermName "." (*) NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S109
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state101<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state109(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 102
    //   Function = (*) TermName "(" TermList ")" ["<>"]
    //   Function = (*) TermName "(" TermList ")" ["="]
    //   NotFormula = (*) PredicateFormula [")"]
    //   NotFormula = (*) PredicateFormula ["<=>"]
    //   NotFormula = (*) PredicateFormula ["==>"]
    //   NotFormula = (*) PredicateFormula ["\\/"]
    //   NotFormula = (*) PredicateFormula [r#"/\\\\"#]
    //   NotFormula = (*) "exists" TermName "." NotFormula [")"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "exists" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "exists" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "forall" TermName "." NotFormula [")"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["<=>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["==>"]
    //   NotFormula = (*) "forall" TermName "." NotFormula ["\\/"]
    //   NotFormula = (*) "forall" TermName "." NotFormula [r#"/\\\\"#]
    //   NotFormula = "forall" TermName "." (*) NotFormula [")"]
    //   NotFormula = "forall" TermName "." (*) NotFormula ["<=>"]
    //   NotFormula = "forall" TermName "." (*) NotFormula ["==>"]
    //   NotFormula = "forall" TermName "." (*) NotFormula ["\\/"]
    //   NotFormula = "forall" TermName "." (*) NotFormula [r#"/\\\\"#]
    //   NotFormula = (*) "~" NotFormula [")"]
    //   NotFormula = (*) "~" NotFormula ["<=>"]
    //   NotFormula = (*) "~" NotFormula ["==>"]
    //   NotFormula = (*) "~" NotFormula ["\\/"]
    //   NotFormula = (*) "~" NotFormula [r#"/\\\\"#]
    //   Predicate = (*) PredicateName [")"]
    //   Predicate = (*) PredicateName ["<=>"]
    //   Predicate = (*) PredicateName ["==>"]
    //   Predicate = (*) PredicateName ["\\/"]
    //   Predicate = (*) PredicateName [r#"/\\\\"#]
    //   Predicate = (*) PredicateName "(" TermList ")" [")"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["<=>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["==>"]
    //   Predicate = (*) PredicateName "(" TermList ")" ["\\/"]
    //   Predicate = (*) PredicateName "(" TermList ")" [r#"/\\\\"#]
    //   PredicateFormula = (*) Predicate [")"]
    //   PredicateFormula = (*) Predicate ["<=>"]
    //   PredicateFormula = (*) Predicate ["==>"]
    //   PredicateFormula = (*) Predicate ["\\/"]
    //   PredicateFormula = (*) Predicate [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "<>" Term [")"]
    //   PredicateFormula = (*) Term "<>" Term ["<=>"]
    //   PredicateFormula = (*) Term "<>" Term ["==>"]
    //   PredicateFormula = (*) Term "<>" Term ["\\/"]
    //   PredicateFormula = (*) Term "<>" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) Term "=" Term [")"]
    //   PredicateFormula = (*) Term "=" Term ["<=>"]
    //   PredicateFormula = (*) Term "=" Term ["==>"]
    //   PredicateFormula = (*) Term "=" Term ["\\/"]
    //   PredicateFormula = (*) Term "=" Term [r#"/\\\\"#]
    //   PredicateFormula = (*) "(" Formula ")" [")"]
    //   PredicateFormula = (*) "(" Formula ")" ["<=>"]
    //   PredicateFormula = (*) "(" Formula ")" ["==>"]
    //   PredicateFormula = (*) "(" Formula ")" ["\\/"]
    //   PredicateFormula = (*) "(" Formula ")" [r#"/\\\\"#]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["("]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [")"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["<=>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["==>"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# ["\\/"]
    //   PredicateName = (*) r#"[A-Z][a-zA-Z0-9]*"# [r#"/\\\\"#]
    //   Term = (*) Function ["<>"]
    //   Term = (*) Function ["="]
    //   Term = (*) Variable ["<>"]
    //   Term = (*) Variable ["="]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["<>"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["="]
    //   Variable = (*) TermName ["<>"]
    //   Variable = (*) TermName ["="]
    //
    //   "(" -> Shift(S38)
    //   "exists" -> Shift(S39)
    //   "forall" -> Shift(S40)
    //   "~" -> Shift(S41)
    //   r#"[A-Z][a-zA-Z0-9]*"# -> Shift(S42)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S19)
    //
    //   Function -> S4
    //   NotFormula -> S110
    //   Predicate -> S34
    //   PredicateFormula -> S35
    //   PredicateName -> S36
    //   Term -> S37
    //   TermName -> S12
    //   Variable -> S13
    pub fn __state102<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state42(input, __lookbehind, __tokens, __sym3));
            }
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::NotFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state110(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::Predicate(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateFormula(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::PredicateName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state37(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 103
    //   NotFormula = "exists" TermName "." NotFormula (*) [EOF]
    //   NotFormula = "exists" TermName "." NotFormula (*) ["<=>"]
    //   NotFormula = "exists" TermName "." NotFormula (*) ["==>"]
    //   NotFormula = "exists" TermName "." NotFormula (*) ["\\/"]
    //   NotFormula = "exists" TermName "." NotFormula (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   "<=>" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   "==>" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   "\\/" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   r#"/\\\\"# -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //
    pub fn __state103<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 104
    //   NotFormula = "forall" TermName "." NotFormula (*) [EOF]
    //   NotFormula = "forall" TermName "." NotFormula (*) ["<=>"]
    //   NotFormula = "forall" TermName "." NotFormula (*) ["==>"]
    //   NotFormula = "forall" TermName "." NotFormula (*) ["\\/"]
    //   NotFormula = "forall" TermName "." NotFormula (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   "<=>" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   "==>" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   "\\/" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   r#"/\\\\"# -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //
    pub fn __state104<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 105
    //   Function = TermName "(" TermList (*) ")" [")"]
    //   Function = TermName "(" TermList (*) ")" [","]
    //
    //   ")" -> Shift(S111)
    //
    pub fn __state105<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state111(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 106
    //   Function = TermName "(" TermList (*) ")" [EOF]
    //   Function = TermName "(" TermList (*) ")" ["<=>"]
    //   Function = TermName "(" TermList (*) ")" ["==>"]
    //   Function = TermName "(" TermList (*) ")" ["\\/"]
    //   Function = TermName "(" TermList (*) ")" [r#"/\\\\"#]
    //
    //   ")" -> Shift(S112)
    //
    pub fn __state106<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state112(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 107
    //   Predicate = PredicateName "(" TermList ")" (*) [")"]
    //   Predicate = PredicateName "(" TermList ")" (*) ["<=>"]
    //   Predicate = PredicateName "(" TermList ")" (*) ["==>"]
    //   Predicate = PredicateName "(" TermList ")" (*) ["\\/"]
    //   Predicate = PredicateName "(" TermList ")" (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   "<=>" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   "==>" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   "\\/" -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //   r#"/\\\\"# -> Reduce(Predicate = PredicateName, "(", TermList, ")" => Call(ActionFn(19));)
    //
    pub fn __state107<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action19(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Predicate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 108
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   Function = TermName "(" (*) TermList ")" [")"]
    //   Function = TermName "(" (*) TermList ")" ["<=>"]
    //   Function = TermName "(" (*) TermList ")" ["==>"]
    //   Function = TermName "(" (*) TermList ")" ["\\/"]
    //   Function = TermName "(" (*) TermList ")" [r#"/\\\\"#]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S60)
    //
    //   (<Term> ",") -> S51
    //   (<Term> ",")+ -> S52
    //   Function -> S53
    //   List<Term> -> S54
    //   Term -> S55
    //   Term? -> S56
    //   TermList -> S113
    //   TermName -> S58
    //   Variable -> S59
    pub fn __state108<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state60(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state51(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state53(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state54(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state55(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state56(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state113(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state58(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state59(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 109
    //   NotFormula = "exists" TermName "." NotFormula (*) [")"]
    //   NotFormula = "exists" TermName "." NotFormula (*) ["<=>"]
    //   NotFormula = "exists" TermName "." NotFormula (*) ["==>"]
    //   NotFormula = "exists" TermName "." NotFormula (*) ["\\/"]
    //   NotFormula = "exists" TermName "." NotFormula (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   "<=>" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   "==>" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   "\\/" -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //   r#"/\\\\"# -> Reduce(NotFormula = "exists", TermName, ".", NotFormula => Call(ActionFn(13));)
    //
    pub fn __state109<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action13(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 110
    //   NotFormula = "forall" TermName "." NotFormula (*) [")"]
    //   NotFormula = "forall" TermName "." NotFormula (*) ["<=>"]
    //   NotFormula = "forall" TermName "." NotFormula (*) ["==>"]
    //   NotFormula = "forall" TermName "." NotFormula (*) ["\\/"]
    //   NotFormula = "forall" TermName "." NotFormula (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   "<=>" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   "==>" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   "\\/" -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //   r#"/\\\\"# -> Reduce(NotFormula = "forall", TermName, ".", NotFormula => Call(ActionFn(12));)
    //
    pub fn __state110<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<String>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<Formula>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::NotFormula(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 111
    //   Function = TermName "(" TermList ")" (*) [")"]
    //   Function = TermName "(" TermList ")" (*) [","]
    //
    //   ")" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "," -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //
    pub fn __state111<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Function(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 112
    //   Function = TermName "(" TermList ")" (*) [EOF]
    //   Function = TermName "(" TermList ")" (*) ["<=>"]
    //   Function = TermName "(" TermList ")" (*) ["==>"]
    //   Function = TermName "(" TermList ")" (*) ["\\/"]
    //   Function = TermName "(" TermList ")" (*) [r#"/\\\\"#]
    //
    //   EOF -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "<=>" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "==>" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "\\/" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   r#"/\\\\"# -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //
    pub fn __state112<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Function(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 113
    //   Function = TermName "(" TermList (*) ")" [")"]
    //   Function = TermName "(" TermList (*) ")" ["<=>"]
    //   Function = TermName "(" TermList (*) ")" ["==>"]
    //   Function = TermName "(" TermList (*) ")" ["\\/"]
    //   Function = TermName "(" TermList (*) ")" [r#"/\\\\"#]
    //
    //   ")" -> Shift(S114)
    //
    pub fn __state113<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state114(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 114
    //   Function = TermName "(" TermList ")" (*) [")"]
    //   Function = TermName "(" TermList ")" (*) ["<=>"]
    //   Function = TermName "(" TermList ")" (*) ["==>"]
    //   Function = TermName "(" TermList ")" (*) ["\\/"]
    //   Function = TermName "(" TermList ")" (*) [r#"/\\\\"#]
    //
    //   ")" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "<=>" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "==>" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "\\/" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   r#"/\\\\"# -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //
    pub fn __state114<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Function(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Formula::parse_Formula;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use parser::formula::{Term, Formula};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Term<
        'input,
    >(
        input: &'input str,
    ) -> Result<Term, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Term(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        _28_3cTerm_3e_20_22_2c_22_29(Term),
        _28_3cTerm_3e_20_22_2c_22_29_2b(::std::vec::Vec<Term>),
        AndFormula(Formula),
        EquivalentFormula(Formula),
        Formula(Formula),
        Function(Term),
        ImpliesFormula(Formula),
        List_3cTerm_3e(Vec<Term>),
        NotFormula(Formula),
        OrFormula(Formula),
        Predicate(Formula),
        PredicateFormula(Formula),
        PredicateName(String),
        Term(Term),
        Term_3f(::std::option::Option<Term>),
        TermList(Vec<Term>),
        TermName(String),
        Variable(Term),
        ____Formula(Formula),
        ____Term(Term),
    }

    // State 0
    //   Function = (*) TermName "(" TermList ")" [EOF]
    //   Term = (*) Function [EOF]
    //   Term = (*) Variable [EOF]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [EOF]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   Variable = (*) TermName [EOF]
    //   __Term = (*) Term [EOF]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S5)
    //
    //   Function -> S1
    //   Term -> S2
    //   TermName -> S3
    //   Variable -> S4
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Function(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Term = Function (*) [EOF]
    //
    //   EOF -> Reduce(Term = Function => Call(ActionFn(21));)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   __Term = Term (*) [EOF]
    //
    //   EOF -> Reduce(__Term = Term => Call(ActionFn(1));)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   Function = TermName (*) "(" TermList ")" [EOF]
    //   Variable = TermName (*) [EOF]
    //
    //   EOF -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "(" -> Shift(S6)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 4
    //   Term = Variable (*) [EOF]
    //
    //   EOF -> Reduce(Term = Variable => Call(ActionFn(22));)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [EOF]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["("]
    //
    //   EOF -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "(" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (0, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   Function = TermName "(" (*) TermList ")" [EOF]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S16)
    //
    //   (<Term> ",") -> S7
    //   (<Term> ",")+ -> S8
    //   Function -> S9
    //   List<Term> -> S10
    //   Term -> S11
    //   Term? -> S12
    //   TermList -> S13
    //   TermName -> S14
    //   Variable -> S15
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state13(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   (<Term> ",")+ = (<Term> ",") (*) [r#"[a-z][a-zA-Z0-9]*"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Reduce((<Term> ",")+ = (<Term> ",") => Call(ActionFn(32));)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action32(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (<Term> ",")+ (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   List<Term> = (<Term> ",")+ (*) Term [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S16)
    //
    //   (<Term> ",") -> S17
    //   Function -> S9
    //   Term -> S18
    //   TermName -> S14
    //   Variable -> S15
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Term = Function (*) [")"]
    //   Term = Function (*) [","]
    //
    //   ")" -> Reduce(Term = Function => Call(ActionFn(21));)
    //   "," -> Reduce(Term = Function => Call(ActionFn(21));)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action21(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   TermList = List<Term> (*) [")"]
    //
    //   ")" -> Reduce(TermList = List<Term> => Call(ActionFn(24));)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action24(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermList(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   (<Term> ",") = Term (*) "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   Term? = Term (*) [")"]
    //
    //   ")" -> Reduce(Term? = Term => Call(ActionFn(30));)
    //   "," -> Shift(S19)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action30(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 12
    //   List<Term> = Term? (*) [")"]
    //
    //   ")" -> Reduce(List<Term> = Term? => Call(ActionFn(29));)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::option::Option<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action29(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::List_3cTerm_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Function = TermName "(" TermList (*) ")" [EOF]
    //
    //   ")" -> Shift(S20)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Function = TermName (*) "(" TermList ")" [")"]
    //   Function = TermName (*) "(" TermList ")" [","]
    //   Variable = TermName (*) [")"]
    //   Variable = TermName (*) [","]
    //
    //   "(" -> Shift(S21)
    //   ")" -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //   "," -> Reduce(Variable = TermName => Call(ActionFn(25));)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action25(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Variable(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Term = Variable (*) [")"]
    //   Term = Variable (*) [","]
    //
    //   ")" -> Reduce(Term = Variable => Call(ActionFn(22));)
    //   "," -> Reduce(Term = Variable => Call(ActionFn(22));)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) ["("]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [")"]
    //   TermName = r#"[a-z][a-zA-Z0-9]*"# (*) [","]
    //
    //   "(" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   ")" -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //   "," -> Reduce(TermName = r#"[a-z][a-zA-Z0-9]*"# => Call(ActionFn(27));)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action27(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::TermName(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 17
    //   (<Term> ",")+ = (<Term> ",")+ (<Term> ",") (*) [r#"[a-z][a-zA-Z0-9]*"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Reduce((<Term> ",")+ = (<Term> ",")+, (<Term> ",") => Call(ActionFn(33));)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Term>>,
        __sym1: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (14, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //   (<Term> ",") = Term (*) "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   List<Term> = (<Term> ",")+ Term (*) [")"]
    //
    //   ")" -> Reduce(List<Term> = (<Term> ",")+, Term => Call(ActionFn(28));)
    //   "," -> Shift(S19)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<Term>>,
        __sym1: &mut Option<Term>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action28(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::List_3cTerm_3e(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 19
    //   (<Term> ",") = Term "," (*) [r#"[a-z][a-zA-Z0-9]*"#]
    //
    //   r#"[a-z][a-zA-Z0-9]*"# -> Reduce((<Term> ",") = Term, "," => Call(ActionFn(34));)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Term>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action34(input, __sym0, __sym1);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   Function = TermName "(" TermList ")" (*) [EOF]
    //
    //   EOF -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Function(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 21
    //   (<Term> ",") = (*) Term "," [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   (<Term> ",")+ = (*) (<Term> ",")+ (<Term> ",") [r#"[a-z][a-zA-Z0-9]*"#]
    //   Function = (*) TermName "(" TermList ")" [")"]
    //   Function = (*) TermName "(" TermList ")" [","]
    //   Function = TermName "(" (*) TermList ")" [")"]
    //   Function = TermName "(" (*) TermList ")" [","]
    //   List<Term> = (*) (<Term> ",")+ Term [")"]
    //   List<Term> = (*) Term? [")"]
    //   Term = (*) Function [")"]
    //   Term = (*) Function [","]
    //   Term = (*) Variable [")"]
    //   Term = (*) Variable [","]
    //   Term? = (*) [")"]
    //   Term? = (*) Term [")"]
    //   TermList = (*) List<Term> [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# ["("]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [")"]
    //   TermName = (*) r#"[a-z][a-zA-Z0-9]*"# [","]
    //   Variable = (*) TermName [")"]
    //   Variable = (*) TermName [","]
    //
    //   ")" -> Reduce(Term? =  => Call(ActionFn(31));)
    //   r#"[a-z][a-zA-Z0-9]*"# -> Shift(S16)
    //
    //   (<Term> ",") -> S7
    //   (<Term> ",")+ -> S8
    //   Function -> S9
    //   List<Term> -> S10
    //   Term -> S11
    //   Term? -> S12
    //   TermList -> S22
    //   TermName -> S14
    //   Variable -> S15
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (14, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (1, _), _)) => {
                let __nt = super::__action31(input, );
                __result = (__lookbehind, __lookahead, __Nonterminal::Term_3f(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::_28_3cTerm_3e_20_22_2c_22_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Function(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::List_3cTerm_3e(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term_3f(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TermList(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::TermName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Variable(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 22
    //   Function = TermName "(" TermList (*) ")" [")"]
    //   Function = TermName "(" TermList (*) ")" [","]
    //
    //   ")" -> Shift(S23)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 23
    //   Function = TermName "(" TermList ")" (*) [")"]
    //   Function = TermName "(" TermList ")" (*) [","]
    //
    //   ")" -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //   "," -> Reduce(Function = TermName, "(", TermList, ")" => Call(ActionFn(23));)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<String>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Vec<Term>>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action23(input, __sym0, __sym1, __sym2, __sym3);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Function(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Term::parse_Term;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '(' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        ',' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '.' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '/' => {
                            __current_state = 5;
                            continue;
                        }
                        '<' => {
                            __current_state = 6;
                            continue;
                        }
                        '=' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '\\' => {
                            __current_state = 9;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '~' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\\' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_state = 16;
                            continue;
                        }
                        '>' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '=' => {
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '/' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '>' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '>' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((14, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    __0: Term,
) -> Term
{
    (__0)
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    l: Formula,
    _: &'input str,
    r: Formula,
) -> Formula
{
    Formula::Equivalent(Box::new(l), Box::new(r))
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    l: Formula,
    _: &'input str,
    r: Formula,
) -> Formula
{
    Formula::Implies(Box::new(l), Box::new(r))
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    l: Formula,
    _: &'input str,
    r: Formula,
) -> Formula
{
    Formula::Or(Box::new(l), Box::new(r))
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    l: Formula,
    _: &'input str,
    r: Formula,
) -> Formula
{
    Formula::And(Box::new(l), Box::new(r))
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    _: &'input str,
    f: Formula,
) -> Formula
{
    Formula::Not(Box::new(f))
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    _: &'input str,
    s: String,
    _: &'input str,
    f: Formula,
) -> Formula
{
    Formula::Forall(s, Box::new(f))
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    _: &'input str,
    s: String,
    _: &'input str,
    f: Formula,
) -> Formula
{
    Formula::Exists(s, Box::new(f))
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    l: Term,
    _: &'input str,
    r: Term,
) -> Formula
{
    Formula::Predicate("=".to_owned(), vec!(l, r))
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    l: Term,
    _: &'input str,
    r: Term,
) -> Formula
{
    Formula::Not(Box::new(Formula::Predicate("=".to_owned(), vec!(l, r))))
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: Formula,
) -> Formula
{
    (__0)
}

pub fn __action18<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Formula,
    _: &'input str,
) -> Formula
{
    (__0)
}

pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: String,
    _: &'input str,
    __1: Vec<Term>,
    _: &'input str,
) -> Formula
{
    Formula::Predicate(__0, __1)
}

pub fn __action20<
    'input,
>(
    input: &'input str,
    s: String,
) -> Formula
{
    match &*s {
            "T" => Formula::True, 
            "F" => Formula::False,
            _ => Formula::Predicate(s, Vec::new())
        }
}

pub fn __action21<
    'input,
>(
    input: &'input str,
    __0: Term,
) -> Term
{
    (__0)
}

pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: Term,
) -> Term
{
    (__0)
}

pub fn __action23<
    'input,
>(
    input: &'input str,
    __0: String,
    _: &'input str,
    __1: Vec<Term>,
    _: &'input str,
) -> Term
{
    Term::Function(__0, __1)
}

pub fn __action24<
    'input,
>(
    input: &'input str,
    __0: Vec<Term>,
) -> Vec<Term>
{
    (__0)
}

pub fn __action25<
    'input,
>(
    input: &'input str,
    __0: String,
) -> Term
{
    Term::Variable(__0)
}

pub fn __action26<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> String
{
    s.to_string()
}

pub fn __action27<
    'input,
>(
    input: &'input str,
    s: &'input str,
) -> String
{
    s.to_string()
}

pub fn __action28<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Term>,
    t: Term,
) -> Vec<Term>
{
    {
            let mut v = v;
            v.push(t);
            v
        }
}

pub fn __action29<
    'input,
>(
    input: &'input str,
    opt_t: ::std::option::Option<Term>,
) -> Vec<Term>
{
    match opt_t {
            Some(t) => vec!(t),
            None => vec!()
        }
}

pub fn __action30<
    'input,
>(
    input: &'input str,
    __0: Term,
) -> ::std::option::Option<Term>
{
    Some(__0)
}

pub fn __action31<
    'input,
>(
    input: &'input str,
) -> ::std::option::Option<Term>
{
    None
}

pub fn __action32<
    'input,
>(
    input: &'input str,
    __0: Term,
) -> ::std::vec::Vec<Term>
{
    vec![__0]
}

pub fn __action33<
    'input,
>(
    input: &'input str,
    v: ::std::vec::Vec<Term>,
    e: Term,
) -> ::std::vec::Vec<Term>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action34<
    'input,
>(
    input: &'input str,
    __0: Term,
    _: &'input str,
) -> Term
{
    (__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
