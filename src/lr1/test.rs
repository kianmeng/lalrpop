use intern::intern;
use grammar::repr::*;
use test_util::{expect_debug, normalized_grammar};
use super::{Items, Lookahead, LR1};
use super::Lookahead::EOF;

fn nt(t: &str) -> NonterminalString {
    NonterminalString(intern(t))
}

fn items<'g>(grammar: &'g Grammar, nonterminal: &str, index: usize, la: Lookahead)
                      -> Items<'g>
{
    let lr1 = LR1::new(&grammar);
    let items =
        lr1.transitive_closure(
            lr1.items(nt(nonterminal), index, la));
    items
}

#[test]
fn start_state() {
    let grammar = normalized_grammar(r#"
grammar Foo {
    token Tok where { };
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1);
        => None;
    };
}
"#);
    let items = items(&grammar, "A", 0, EOF);
    expect_debug(items, r#"[
    A = (*) B "C" [EOF],
    B = (*) "D" ["C"],
    B = (*) ["C"]
]"#);
}

#[test]
fn start_state_1() {
    let grammar = normalized_grammar(r#"
grammar Foo {
    token Tok where { };
    A = B C;
    B: Option<u32> = {
        "B1" => Some(1);
        => None;
    };
    C: Option<u32> = {
        "C1" => Some(1);
        => None;
    };
}
"#);

    expect_debug(items(&grammar, "A", 0, EOF), r#"[
    A = (*) B C [EOF],
    B = (*) "B1" ["C1"],
    B = (*) ["C1"],
    B = (*) "B1" [EOF],
    B = (*) [EOF]
]"#);

    expect_debug(items(&grammar, "A", 1, EOF), r#"[
    A = B (*) C [EOF],
    C = (*) "C1" [EOF],
    C = (*) [EOF]
]"#);
}

#[test]
fn expr_grammar1() {
    let grammar = normalized_grammar(r#"
grammar Foo {
    token Tok where { };

    S: () =
        E => ();

    E: () = {
        E "-" T => ();
        T => ();
    };

    T: () = {
        "N" => ();
        "(" E ")" => ();
    };
}
"#);

    let lr1 = LR1::new(&grammar);

    // for now, just test that process does not result in an error
    // and yields expected number of states.
    let states = lr1.build_states(nt("S")).unwrap();
    assert_eq!(states.len(), 16);
}
