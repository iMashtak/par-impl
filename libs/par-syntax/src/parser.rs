// auto-generated: "lalrpop 0.22.2"
// sha3: a1dfc4b1cf81f597ae16c46e1846a7b409ff63a067887d4b6f44e507aa04caea
use lalrpop_util::ErrorRecovery;
use arcstr::ArcStr;
use crate::ast::*;
use crate::tokens::{Token, LexicalError};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as ___lalrpop_util;
#[allow(unused_imports)]
use self::___lalrpop_util::state_machine as ___state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod ___parse___Program {

    use lalrpop_util::ErrorRecovery;
    use arcstr::ArcStr;
    use crate::ast::*;
    use crate::tokens::{Token, LexicalError};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as ___lalrpop_util;
    #[allow(unused_imports)]
    use self::___lalrpop_util::state_machine as ___state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use super::___ToTriple;
    #[allow(dead_code)]
    pub(crate) enum ___Symbol<>
     {
        Variant0(Token),
        Variant1(ArcStr),
        Variant2(i32),
        Variant3(u64),
        Variant4(CaseBranch),
        Variant5(alloc::vec::Vec<CaseBranch>),
        Variant6(usize),
        Variant7(Option<CaseBranch>),
        Variant8(Vec<CaseBranch>),
        Variant9(Comment),
        Variant10(Definition),
        Variant11(alloc::vec::Vec<Definition>),
        Variant12(GlobalIdent),
        Variant13(Ident),
        Variant14(Label),
        Variant15(Option<Label>),
        Variant16(LocalIdent),
        Variant17(ProcessExpr),
        Variant18(ProcessStep),
        Variant19(alloc::vec::Vec<ProcessStep>),
        Variant20(Program),
    }
    const ___ACTION: &[i8] = &[
        // State 0
        0, 30, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 30, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 40, 43, 33, 41, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 6
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 7
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 8
        0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, -49, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 10
        0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, -47, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 40, 43, 33, 41, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 40, 43, 33, 41, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 40, 43, 33, 41, 0,
        // State 18
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 21
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 24
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 25
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 26
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 27
        0, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0,
        // State 33
        0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0,
        // State 35
        0, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0,
        // State 36
        0, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0,
        // State 37
        0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0,
        // State 39
        0, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 40
        0, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, -29, -29, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, -29, 0, -29, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 41
        0, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0,
        // State 42
        0, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 12, 0, 0, 0, 14, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0,
        // State 46
        0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0,
        // State 47
        0, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 48
        0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0,
        // State 57
        0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0,
        // State 60
        0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0,
        // State 61
        0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0,
        // State 65
        0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn ___action(state: i8, integer: usize) -> i8 {
        ___ACTION[(state as usize) * 64 + integer]
    }
    const ___EOF_ACTION: &[i8] = &[
        // State 0
        -54,
        // State 1
        -55,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -21,
        // State 28
        -56,
        // State 29
        0,
        // State 30
        -22,
        // State 31
        0,
        // State 32
        -23,
        // State 33
        -17,
        // State 34
        -24,
        // State 35
        -34,
        // State 36
        -25,
        // State 37
        -18,
        // State 38
        -35,
        // State 39
        -32,
        // State 40
        -29,
        // State 41
        -31,
        // State 42
        -33,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -30,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
    ];
    fn ___goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 22,
            5 => match state {
                22 => 66,
                _ => 62,
            },
            7 => 63,
            9 => match state {
                1 => 30,
                _ => 27,
            },
            11 => 1,
            12 => match state {
                2 => 31,
                3 => 33,
                _ => 34,
            },
            13 => 35,
            14 => match state {
                10 => 50,
                16 => 58,
                _ => 48,
            },
            16 => match state {
                5 => 43,
                6..=7 | 18 | 20..=21 | 24..=26 => 44,
                9 => 49,
                12 => 52,
                14 => 55,
                15 => 56,
                23 => 69,
                _ => 36,
            },
            17 => match state {
                11 => 51,
                13 => 54,
                17 => 60,
                _ => 37,
            },
            18 => match state {
                7 | 21 | 24 | 26 => 46,
                _ => 45,
            },
            20 => match state {
                18 => 21,
                20 => 24,
                25 => 26,
                _ => 7,
            },
            21 => 28,
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const ___TERMINAL: &[&str] = &[
        r###""__error""###,
        r###""native""###,
        r###""def""###,
        r###""type""###,
        r###""let""###,
        r###""recv""###,
        r###""either""###,
        r###""case""###,
        r###""choice""###,
        r###""chan""###,
        r###""dual""###,
        r###""recursive""###,
        r###""iterative""###,
        r###""box""###,
        r###""self""###,
        r###""begin""###,
        r###""loop""###,
        r###""in""###,
        r###""try""###,
        r###""catch""###,
        r###""throw""###,
        r###""do""###,
        r###""unfounded""###,
        r###""and""###,
        r###""or""###,
        r###""not""###,
        r###""break""###,
        r###""!""###,
        r###""?""###,
        r###"":""###,
        r###"";""###,
        r###""[""###,
        r###""]""###,
        r###""(""###,
        r###"")""###,
        r###""{""###,
        r###""}""###,
        r###""<>""###,
        r###""<""###,
        r###"">""###,
        r###""<=""###,
        r###"">=""###,
        r###"",""###,
        r###"".""###,
        r###""/""###,
        r###""=""###,
        r###""==""###,
        r###""!=""###,
        r###""=>""###,
        r###""->""###,
        r###""<-""###,
        r###""*""###,
        r###""-""###,
        r###""+""###,
        r###""%""###,
        r###""@""###,
        r###""::""###,
        r###""'""###,
        r###""str""###,
        r###""i32""###,
        r###""u64""###,
        r###""global_ident""###,
        r###""local_ident""###,
        r###""comment""###,
    ];
    fn ___expected_tokens(___state: i8) -> alloc::vec::Vec<alloc::string::String> {
        ___TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = ___action(___state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn ___expected_tokens_from_states<
        'err,
    >(
        ___states: &[i8],
        _: core::marker::PhantomData<(&'err ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        ___TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if ___accepts(None, ___states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct ___StateMachine<'err>
    where 
    {
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___phantom: core::marker::PhantomData<(&'err ())>,
    }
    impl<'err> ___state_machine::ParserDefinition for ___StateMachine<'err>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = ___Symbol<>;
        type Success = Program;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            ___token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            ___action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            ___action(state, 64 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            ___EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            ___goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            ___token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            ___expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            ___expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: ___state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<___state_machine::SymbolTriple<Self>>,
        ) -> Option<___state_machine::ParseResult<Self>> {
            ___reduce(
                self.errors,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> ___state_machine::SimulatedReduce<Self> {
            ___simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn ___token_to_integer<
        'err,
    >(
        ___token: &Token,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match ___token {
            Token::Error if true => Some(0),
            Token::Native if true => Some(1),
            Token::Def if true => Some(2),
            Token::Type if true => Some(3),
            Token::Let if true => Some(4),
            Token::Recv if true => Some(5),
            Token::Either if true => Some(6),
            Token::Case if true => Some(7),
            Token::Choice if true => Some(8),
            Token::Chan if true => Some(9),
            Token::Dual if true => Some(10),
            Token::Recursive if true => Some(11),
            Token::Iterative if true => Some(12),
            Token::Box if true => Some(13),
            Token::Self_ if true => Some(14),
            Token::Begin if true => Some(15),
            Token::Loop if true => Some(16),
            Token::In if true => Some(17),
            Token::Try if true => Some(18),
            Token::Catch if true => Some(19),
            Token::Throw if true => Some(20),
            Token::Do if true => Some(21),
            Token::Unfounded if true => Some(22),
            Token::And if true => Some(23),
            Token::Or if true => Some(24),
            Token::Not if true => Some(25),
            Token::Break if true => Some(26),
            Token::Unit if true => Some(27),
            Token::Question if true => Some(28),
            Token::Colon if true => Some(29),
            Token::Semicolon if true => Some(30),
            Token::LBracket if true => Some(31),
            Token::RBracket if true => Some(32),
            Token::LParen if true => Some(33),
            Token::RParen if true => Some(34),
            Token::LCurly if true => Some(35),
            Token::RCurly if true => Some(36),
            Token::Link if true => Some(37),
            Token::Lt if true => Some(38),
            Token::Gt if true => Some(39),
            Token::Le if true => Some(40),
            Token::Ge if true => Some(41),
            Token::Comma if true => Some(42),
            Token::Dot if true => Some(43),
            Token::Slash if true => Some(44),
            Token::Eq if true => Some(45),
            Token::EqEq if true => Some(46),
            Token::NotEq if true => Some(47),
            Token::DoubleArrow if true => Some(48),
            Token::Arrow if true => Some(49),
            Token::ReverseArrow if true => Some(50),
            Token::Star if true => Some(51),
            Token::Minus if true => Some(52),
            Token::Plus if true => Some(53),
            Token::Percent if true => Some(54),
            Token::Amp if true => Some(55),
            Token::DoubleColon if true => Some(56),
            Token::SingleQuote if true => Some(57),
            Token::Str(_) if true => Some(58),
            Token::I32(_) if true => Some(59),
            Token::U64(_) if true => Some(60),
            Token::GlobalIdent(_) if true => Some(61),
            Token::LocalIdent(_) if true => Some(62),
            Token::Comment(_) if true => Some(63),
            _ => None,
        }
    }
    fn ___token_to_symbol<
        'err,
    >(
        ___token_index: usize,
        ___token: Token,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> ___Symbol<>
    {
        #[allow(clippy::manual_range_patterns)]match ___token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => ___Symbol::Variant0(___token),
            58 | 61 | 62 | 63 => match ___token {
                Token::Str(___tok0) | Token::GlobalIdent(___tok0) | Token::LocalIdent(___tok0) | Token::Comment(___tok0) if true => ___Symbol::Variant1(___tok0),
                _ => unreachable!(),
            },
            59 => match ___token {
                Token::I32(___tok0) if true => ___Symbol::Variant2(___tok0),
                _ => unreachable!(),
            },
            60 => match ___token {
                Token::U64(___tok0) if true => ___Symbol::Variant3(___tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn ___simulate_reduce<
        'err,
    >(
        ___reduce_index: i8,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> ___state_machine::SimulatedReduce<___StateMachine<'err>>
    {
        match ___reduce_index {
            0 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 5,
                }
            }
            8 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            17 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            27 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            28 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            29 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            30 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            31 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            32 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            33 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            34 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 18,
                }
            }
            36 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            37 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 18,
                }
            }
            38 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 18,
                }
            }
            39 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 18,
                }
            }
            40 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 18,
                }
            }
            41 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 18,
                }
            }
            42 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 18,
                }
            }
            43 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 18,
                }
            }
            44 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            45 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            46 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            47 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            48 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            49 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            50 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            51 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            52 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            53 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            54 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            55 => ___state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {___reduce_index}",)
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl Default for ProgramParser { fn default() -> Self { Self::new() } }
    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'err,
            ___TOKEN: ___ToTriple<'err, >,
            ___TOKENS: IntoIterator<Item=___TOKEN>,
        >(
            &self,
            errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
            ___tokens0: ___TOKENS,
        ) -> Result<Program, ___lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let ___tokens = ___tokens0.into_iter();
            let mut ___tokens = ___tokens.map(|t| ___ToTriple::to_triple(t));
            ___state_machine::Parser::drive(
                ___StateMachine {
                    errors,
                    ___phantom: core::marker::PhantomData::<(&())>,
                },
                ___tokens,
            )
        }
    }
    fn ___accepts<
        'err,
    >(
        ___error_state: Option<i8>,
        ___states: &[i8],
        ___opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> bool
    {
        let mut ___states = ___states.to_vec();
        ___states.extend(___error_state);
        loop {
            let mut ___states_len = ___states.len();
            let ___top = ___states[___states_len - 1];
            let ___action = match ___opt_integer {
                None => ___EOF_ACTION[___top as usize],
                Some(___integer) => ___action(___top, ___integer),
            };
            if ___action == 0 { return false; }
            if ___action > 0 { return true; }
            let (___to_pop, ___nt) = match ___simulate_reduce(-(___action + 1), core::marker::PhantomData::<(&())>) {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                ___state_machine::SimulatedReduce::Accept => return true,
            };
            ___states_len -= ___to_pop;
            ___states.truncate(___states_len);
            let ___top = ___states[___states_len - 1];
            let ___next_state = ___goto(___top, ___nt);
            ___states.push(___next_state);
        }
    }
    fn ___reduce<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___action: i8,
        ___lookahead_start: Option<&usize>,
        ___states: &mut alloc::vec::Vec<i8>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> Option<Result<Program,___lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (___pop_states, ___nonterminal) = match ___action {
            0 => {
                ___reduce0(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                ___reduce1(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                ___reduce2(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                ___reduce3(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                ___reduce4(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                ___reduce5(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                ___reduce6(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                ___reduce7(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                ___reduce8(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                ___reduce9(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                ___reduce10(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                ___reduce11(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                ___reduce12(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                ___reduce13(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                ___reduce14(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                ___reduce15(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                ___reduce16(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                ___reduce17(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                ___reduce18(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                ___reduce19(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                ___reduce20(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                ___reduce21(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                ___reduce22(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                ___reduce23(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                ___reduce24(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                ___reduce25(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                ___reduce26(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                ___reduce27(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                ___reduce28(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                ___reduce29(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                ___reduce30(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                ___reduce31(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                ___reduce32(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                ___reduce33(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                ___reduce34(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                ___reduce35(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                ___reduce36(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                ___reduce37(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                ___reduce38(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                ___reduce39(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                ___reduce40(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                ___reduce41(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                ___reduce42(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                ___reduce43(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                ___reduce44(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                ___reduce45(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                ___reduce46(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                ___reduce47(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                ___reduce48(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                ___reduce49(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                ___reduce50(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                ___reduce51(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                ___reduce52(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                ___reduce53(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                ___reduce54(errors, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                // ___Program = Program => ActionFn(0);
                let ___sym0 = ___pop_Variant20(___symbols);
                let ___start = ___sym0.0;
                let ___end = ___sym0.2;
                let ___nt = super::___action0::<>(errors, ___sym0);
                return Some(Ok(___nt));
            }
            _ => panic!("invalid action code {___action}")
        };
        let ___states_len = ___states.len();
        ___states.truncate(___states_len - ___pop_states);
        let ___state = *___states.last().unwrap();
        let ___next_state = ___goto(___state, ___nonterminal);
        ___states.push(___next_state);
        None
    }
    #[inline(never)]
    fn ___symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn ___pop_Variant1<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, ArcStr, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant1(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant4<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, CaseBranch, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant4(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant9<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Comment, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant9(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant10<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Definition, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant10(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant12<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, GlobalIdent, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant12(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant13<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Ident, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant13(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant14<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Label, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant14(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant16<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, LocalIdent, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant16(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant7<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Option<CaseBranch>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant7(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant15<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Option<Label>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant15(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant17<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, ProcessExpr, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant17(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant18<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, ProcessStep, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant18(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant20<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Program, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant20(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant0<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant0(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant8<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, Vec<CaseBranch>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant8(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant5<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<CaseBranch>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant5(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant11<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<Definition>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant11(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant19<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ProcessStep>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant19(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant2<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant2(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant3<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, u64, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant3(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant6<
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant6(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___reduce0<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<CaseBranch> ",") = CaseBranch, "," => ActionFn(43);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant4(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action43::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant4(___nt), ___end));
        (2, 0)
    }
    fn ___reduce1<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<CaseBranch> ",")* =  => ActionFn(41);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action41::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (0, 1)
    }
    fn ___reduce2<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<CaseBranch> ",")* = (<CaseBranch> ",")+ => ActionFn(42);
        let ___sym0 = ___pop_Variant5(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action42::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (1, 1)
    }
    fn ___reduce3<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<CaseBranch> ",")+ = CaseBranch, "," => ActionFn(46);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant4(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action46::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (2, 2)
    }
    fn ___reduce4<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<CaseBranch> ",")+ = (<CaseBranch> ",")+, CaseBranch, "," => ActionFn(47);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant4(___symbols);
        let ___sym0 = ___pop_Variant5(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action47::<>(errors, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (3, 2)
    }
    fn ___reduce5<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(36);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action36::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant6(___nt), ___end));
        (0, 3)
    }
    fn ___reduce6<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(33);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action33::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant6(___nt), ___end));
        (0, 4)
    }
    fn ___reduce7<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // CaseBranch = ".", LocalIdent, "=>", "{", "}" => ActionFn(108);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action108::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant4(___nt), ___end));
        (5, 5)
    }
    fn ___reduce8<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // CaseBranch = ".", LocalIdent, "=>", "{", ProcessStep+, "}" => ActionFn(109);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant0(___symbols);
        let ___sym4 = ___pop_Variant19(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action109::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant4(___nt), ___end));
        (6, 5)
    }
    fn ___reduce9<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // CaseBranch? = CaseBranch => ActionFn(39);
        let ___sym0 = ___pop_Variant4(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action39::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant7(___nt), ___end));
        (1, 6)
    }
    fn ___reduce10<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // CaseBranch? =  => ActionFn(40);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action40::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant7(___nt), ___end));
        (0, 6)
    }
    fn ___reduce11<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Comma<CaseBranch> = CaseBranch => ActionFn(96);
        let ___sym0 = ___pop_Variant4(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action96::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant8(___nt), ___end));
        (1, 7)
    }
    fn ___reduce12<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Comma<CaseBranch> =  => ActionFn(97);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action97::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant8(___nt), ___end));
        (0, 7)
    }
    fn ___reduce13<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Comma<CaseBranch> = (<CaseBranch> ",")+, CaseBranch => ActionFn(98);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant4(___symbols);
        let ___sym0 = ___pop_Variant5(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action98::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant8(___nt), ___end));
        (2, 7)
    }
    fn ___reduce14<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Comma<CaseBranch> = (<CaseBranch> ",")+ => ActionFn(99);
        let ___sym0 = ___pop_Variant5(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action99::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant8(___nt), ___end));
        (1, 7)
    }
    fn ___reduce15<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Comment = "comment" => ActionFn(74);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action74::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (1, 8)
    }
    fn ___reduce16<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Definition = "native", "def", GlobalIdent => ActionFn(75);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant12(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action75::<>(errors, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant10(___nt), ___end));
        (3, 9)
    }
    fn ___reduce17<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Definition = "def", GlobalIdent, "=", ProcessExpr => ActionFn(76);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant17(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant12(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action76::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant10(___nt), ___end));
        (4, 9)
    }
    fn ___reduce18<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Definition* =  => ActionFn(34);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action34::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (0, 10)
    }
    fn ___reduce19<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Definition* = Definition+ => ActionFn(35);
        let ___sym0 = ___pop_Variant11(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action35::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 10)
    }
    fn ___reduce20<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Definition+ = Definition => ActionFn(37);
        let ___sym0 = ___pop_Variant10(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action37::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 11)
    }
    fn ___reduce21<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Definition+ = Definition+, Definition => ActionFn(38);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant10(___symbols);
        let ___sym0 = ___pop_Variant11(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action38::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (2, 11)
    }
    fn ___reduce22<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // GlobalIdent = "global_ident" => ActionFn(77);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action77::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (1, 12)
    }
    fn ___reduce23<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Ident = GlobalIdent => ActionFn(23);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action23::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (1, 13)
    }
    fn ___reduce24<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Ident = LocalIdent => ActionFn(24);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action24::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (1, 13)
    }
    fn ___reduce25<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Label = "@", LocalIdent => ActionFn(78);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action78::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (2, 14)
    }
    fn ___reduce26<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Label? = Label => ActionFn(31);
        let ___sym0 = ___pop_Variant14(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action31::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (1, 15)
    }
    fn ___reduce27<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Label? =  => ActionFn(32);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action32::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (0, 15)
    }
    fn ___reduce28<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // LocalIdent = "local_ident" => ActionFn(79);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action79::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 16)
    }
    fn ___reduce29<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessExpr = "chan", LocalIdent, "{", ProcessStep+, "}" => ActionFn(80);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant19(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action80::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (5, 17)
    }
    fn ___reduce30<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessExpr = "str" => ActionFn(81);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action81::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 17)
    }
    fn ___reduce31<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessExpr = "i32" => ActionFn(82);
        let ___sym0 = ___pop_Variant2(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action82::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 17)
    }
    fn ___reduce32<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessExpr = "u64" => ActionFn(83);
        let ___sym0 = ___pop_Variant3(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action83::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 17)
    }
    fn ___reduce33<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessExpr = Ident => ActionFn(84);
        let ___sym0 = ___pop_Variant13(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action84::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 17)
    }
    fn ___reduce34<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessExpr = "!" => ActionFn(85);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action85::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 17)
    }
    fn ___reduce35<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = "let", LocalIdent, "=", ProcessExpr => ActionFn(86);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant17(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action86::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (4, 18)
    }
    fn ___reduce36<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, "<>", ProcessExpr => ActionFn(87);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant17(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action87::<>(errors, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (3, 18)
    }
    fn ___reduce37<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, ".", "begin", Label, "{", "}" => ActionFn(110);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant0(___symbols);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant14(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action110::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (6, 18)
    }
    fn ___reduce38<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, ".", "begin", Label, "{", ProcessStep+, "}" => ActionFn(111);
        assert!(___symbols.len() >= 7);
        let ___sym6 = ___pop_Variant0(___symbols);
        let ___sym5 = ___pop_Variant19(___symbols);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant14(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym6.2;
        let ___nt = super::___action111::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5, ___sym6);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (7, 18)
    }
    fn ___reduce39<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, ".", "begin", "{", "}" => ActionFn(112);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action112::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (5, 18)
    }
    fn ___reduce40<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, ".", "begin", "{", ProcessStep+, "}" => ActionFn(113);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant0(___symbols);
        let ___sym4 = ___pop_Variant19(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action113::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (6, 18)
    }
    fn ___reduce41<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, ".", "case", "{", Comma<CaseBranch>, "}" => ActionFn(89);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant0(___symbols);
        let ___sym4 = ___pop_Variant8(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action89::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (6, 18)
    }
    fn ___reduce42<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, "[", LocalIdent, "]" => ActionFn(90);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action90::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (4, 18)
    }
    fn ___reduce43<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, "(", ProcessExpr, ")" => ActionFn(91);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant17(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action91::<>(errors, ___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (4, 18)
    }
    fn ___reduce44<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = LocalIdent, ".", LocalIdent => ActionFn(92);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action92::<>(errors, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (3, 18)
    }
    fn ___reduce45<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = "loop", Label => ActionFn(104);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action104::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (2, 18)
    }
    fn ___reduce46<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = "loop" => ActionFn(105);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action105::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (1, 18)
    }
    fn ___reduce47<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = "break", Label => ActionFn(106);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action106::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (2, 18)
    }
    fn ___reduce48<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep = "break" => ActionFn(107);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action107::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (1, 18)
    }
    fn ___reduce49<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep* =  => ActionFn(29);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action29::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant19(___nt), ___end));
        (0, 19)
    }
    fn ___reduce50<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep* = ProcessStep+ => ActionFn(30);
        let ___sym0 = ___pop_Variant19(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action30::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant19(___nt), ___end));
        (1, 19)
    }
    fn ___reduce51<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep+ = ProcessStep => ActionFn(26);
        let ___sym0 = ___pop_Variant18(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action26::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant19(___nt), ___end));
        (1, 20)
    }
    fn ___reduce52<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ProcessStep+ = ProcessStep+, ProcessStep => ActionFn(27);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant18(___symbols);
        let ___sym0 = ___pop_Variant19(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action27::<>(errors, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant19(___nt), ___end));
        (2, 20)
    }
    fn ___reduce53<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(100);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2)).unwrap_or_default();
        let ___end = ___start;
        let ___nt = super::___action100::<>(errors, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant20(___nt), ___end));
        (0, 21)
    }
    fn ___reduce54<
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Program = Definition+ => ActionFn(101);
        let ___sym0 = ___pop_Variant11(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action101::<>(errors, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant20(___nt), ___end));
        (1, 21)
    }
}
#[allow(unused_imports)]
pub use self::___parse___Program::ProgramParser;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action0<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, Program, usize),
) -> Program
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action1<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, defs, _): (usize, alloc::vec::Vec<Definition>, usize),
    (_, r, _): (usize, usize, usize),
) -> Program
{
    Program {defs, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action2<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, GlobalIdent, usize),
    (_, r, _): (usize, usize, usize),
) -> Definition
{
    Definition::Native {name, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action3<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, GlobalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, expr, _): (usize, ProcessExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> Definition
{
    Definition::ProcessExpr {name, expr, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action4<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, expr, _): (usize, ProcessExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Let {name, expr, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action5<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, target, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, expr, _): (usize, ProcessExpr, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Link {target, expr, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action6<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, target, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, label, _): (usize, Option<Label>, usize),
    (_, _, _): (usize, Token, usize),
    (_, steps, _): (usize, alloc::vec::Vec<ProcessStep>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Begin {target, label, steps, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action7<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, target, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, alts, _): (usize, Vec<CaseBranch>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Case {target, alts, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action8<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, target, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Recv {target, name, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action9<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, target, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, expr, _): (usize, ProcessExpr, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Send {target, expr, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action10<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, target, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, signal, _): (usize, LocalIdent, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Signal {target, signal, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action11<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, label, _): (usize, Option<Label>, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Loop {label, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action12<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, label, _): (usize, Option<Label>, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessStep
{
    ProcessStep::Break {label, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action13<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, steps, _): (usize, alloc::vec::Vec<ProcessStep>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessExpr
{
    ProcessExpr::Chan {name, steps, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action14<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, ArcStr, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessExpr
{
    ProcessExpr::Str {value: x, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action15<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, i32, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessExpr
{
    ProcessExpr::I32 {value: x, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action16<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, u64, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessExpr
{
    ProcessExpr::U64 {value: x, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action17<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, x, _): (usize, Ident, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessExpr
{
    ProcessExpr::Ident {value: x, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action18<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> ProcessExpr
{
    ProcessExpr::Unit {l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action19<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, LocalIdent, usize),
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, inner, _): (usize, alloc::vec::Vec<ProcessStep>, usize),
    (_, _, _): (usize, Token, usize),
    (_, r, _): (usize, usize, usize),
) -> CaseBranch
{
    CaseBranch {name, inner, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action20<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, LocalIdent, usize),
    (_, r, _): (usize, usize, usize),
) -> Label
{
    Label {name, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action21<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, content, _): (usize, ArcStr, usize),
    (_, r, _): (usize, usize, usize),
) -> GlobalIdent
{
    GlobalIdent {content, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action22<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, content, _): (usize, ArcStr, usize),
    (_, r, _): (usize, usize, usize),
) -> LocalIdent
{
    LocalIdent {content, l, r}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action23<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, x, _): (usize, GlobalIdent, usize),
) -> Ident
{
    Ident::Global(x)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action24<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, x, _): (usize, LocalIdent, usize),
) -> Ident
{
    Ident::Local(x)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action25<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, l, _): (usize, usize, usize),
    (_, content, _): (usize, ArcStr, usize),
    (_, r, _): (usize, usize, usize),
) -> Comment
{
    Comment {content, l, r }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action26<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, ProcessStep, usize),
) -> alloc::vec::Vec<ProcessStep>
{
    alloc::vec![___0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action27<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, v, _): (usize, alloc::vec::Vec<ProcessStep>, usize),
    (_, e, _): (usize, ProcessStep, usize),
) -> alloc::vec::Vec<ProcessStep>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action28<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, mut v, _): (usize, alloc::vec::Vec<CaseBranch>, usize),
    (_, e, _): (usize, Option<CaseBranch>, usize),
) -> Vec<CaseBranch>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action29<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<ProcessStep>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action30<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, v, _): (usize, alloc::vec::Vec<ProcessStep>, usize),
) -> alloc::vec::Vec<ProcessStep>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action31<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, Label, usize),
) -> Option<Label>
{
    Some(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action32<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> Option<Label>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::needless_lifetimes)]
fn ___action33<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> usize
{
    *___lookbehind
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action34<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<Definition>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action35<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, v, _): (usize, alloc::vec::Vec<Definition>, usize),
) -> alloc::vec::Vec<Definition>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::needless_lifetimes)]
fn ___action36<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> usize
{
    *___lookahead
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action37<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, Definition, usize),
) -> alloc::vec::Vec<Definition>
{
    alloc::vec![___0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action38<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, v, _): (usize, alloc::vec::Vec<Definition>, usize),
    (_, e, _): (usize, Definition, usize),
) -> alloc::vec::Vec<Definition>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action39<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, CaseBranch, usize),
) -> Option<CaseBranch>
{
    Some(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action40<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> Option<CaseBranch>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action41<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<CaseBranch>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action42<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, v, _): (usize, alloc::vec::Vec<CaseBranch>, usize),
) -> alloc::vec::Vec<CaseBranch>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action43<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, CaseBranch, usize),
    (_, _, _): (usize, Token, usize),
) -> CaseBranch
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action44<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, ___0, _): (usize, CaseBranch, usize),
) -> alloc::vec::Vec<CaseBranch>
{
    alloc::vec![___0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn ___action45<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    (_, v, _): (usize, alloc::vec::Vec<CaseBranch>, usize),
    (_, e, _): (usize, CaseBranch, usize),
) -> alloc::vec::Vec<CaseBranch>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action46<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, CaseBranch, usize),
    ___1: (usize, Token, usize),
) -> alloc::vec::Vec<CaseBranch>
{
    let ___start0 = ___0.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action43(
        errors,
        ___0,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action44(
        errors,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action47<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<CaseBranch>, usize),
    ___1: (usize, CaseBranch, usize),
    ___2: (usize, Token, usize),
) -> alloc::vec::Vec<CaseBranch>
{
    let ___start0 = ___1.0;
    let ___end0 = ___2.2;
    let ___temp0 = ___action43(
        errors,
        ___1,
        ___2,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action45(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action48<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Option<CaseBranch>, usize),
) -> Vec<CaseBranch>
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action41(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action28(
        errors,
        ___temp0,
        ___0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action49<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<CaseBranch>, usize),
    ___1: (usize, Option<CaseBranch>, usize),
) -> Vec<CaseBranch>
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.2;
    let ___temp0 = ___action42(
        errors,
        ___0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action28(
        errors,
        ___temp0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action50<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___5: (usize, Token, usize),
    ___6: (usize, usize, usize),
) -> CaseBranch
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action19(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action51<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
    ___1: (usize, usize, usize),
) -> Comment
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action25(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action52<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, GlobalIdent, usize),
    ___3: (usize, usize, usize),
) -> Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action2(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action53<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, GlobalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, ProcessExpr, usize),
    ___4: (usize, usize, usize),
) -> Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action3(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action54<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
    ___1: (usize, usize, usize),
) -> GlobalIdent
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action21(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action55<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, usize, usize),
) -> Label
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action20(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action56<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
    ___1: (usize, usize, usize),
) -> LocalIdent
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action22(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action57<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___4: (usize, Token, usize),
    ___5: (usize, usize, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action13(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action58<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
    ___1: (usize, usize, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action14(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action59<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, i32, usize),
    ___1: (usize, usize, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action15(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action60<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, u64, usize),
    ___1: (usize, usize, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action16(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action61<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Ident, usize),
    ___1: (usize, usize, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action17(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action62<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, usize, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action18(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action63<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, ProcessExpr, usize),
    ___4: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action4(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action64<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, ProcessExpr, usize),
    ___3: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action5(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action65<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Option<Label>, usize),
    ___4: (usize, Token, usize),
    ___5: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___6: (usize, Token, usize),
    ___7: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action6(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___6,
        ___7,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action66<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, Vec<CaseBranch>, usize),
    ___5: (usize, Token, usize),
    ___6: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action7(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action67<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, LocalIdent, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action8(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action68<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, ProcessExpr, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action9(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action69<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, LocalIdent, usize),
    ___3: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action10(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action70<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Option<Label>, usize),
    ___2: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action11(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action71<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Option<Label>, usize),
    ___2: (usize, usize, usize),
) -> ProcessStep
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action12(
        errors,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action72<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<Definition>, usize),
    ___1: (usize, usize, usize),
) -> Program
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action36(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action1(
        errors,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action73<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___5: (usize, Token, usize),
) -> CaseBranch
{
    let ___start0 = ___5.2;
    let ___end0 = ___5.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action50(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action74<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
) -> Comment
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action51(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action75<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, GlobalIdent, usize),
) -> Definition
{
    let ___start0 = ___2.2;
    let ___end0 = ___2.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action52(
        errors,
        ___0,
        ___1,
        ___2,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action76<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, GlobalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, ProcessExpr, usize),
) -> Definition
{
    let ___start0 = ___3.2;
    let ___end0 = ___3.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action53(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action77<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
) -> GlobalIdent
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action54(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action78<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
) -> Label
{
    let ___start0 = ___1.2;
    let ___end0 = ___1.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action55(
        errors,
        ___0,
        ___1,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action79<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
) -> LocalIdent
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action56(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action80<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___4: (usize, Token, usize),
) -> ProcessExpr
{
    let ___start0 = ___4.2;
    let ___end0 = ___4.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action57(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action81<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, ArcStr, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action58(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action82<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, i32, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action59(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action83<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, u64, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action60(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action84<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Ident, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action61(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action85<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
) -> ProcessExpr
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action62(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action86<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, ProcessExpr, usize),
) -> ProcessStep
{
    let ___start0 = ___3.2;
    let ___end0 = ___3.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action63(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action87<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, ProcessExpr, usize),
) -> ProcessStep
{
    let ___start0 = ___2.2;
    let ___end0 = ___2.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action64(
        errors,
        ___0,
        ___1,
        ___2,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action88<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Option<Label>, usize),
    ___4: (usize, Token, usize),
    ___5: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___6: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___6.2;
    let ___end0 = ___6.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action65(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___6,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action89<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, Vec<CaseBranch>, usize),
    ___5: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___5.2;
    let ___end0 = ___5.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action66(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action90<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, LocalIdent, usize),
    ___3: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___3.2;
    let ___end0 = ___3.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action67(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action91<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, ProcessExpr, usize),
    ___3: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___3.2;
    let ___end0 = ___3.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action68(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action92<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, LocalIdent, usize),
) -> ProcessStep
{
    let ___start0 = ___2.2;
    let ___end0 = ___2.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action69(
        errors,
        ___0,
        ___1,
        ___2,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action93<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Option<Label>, usize),
) -> ProcessStep
{
    let ___start0 = ___1.2;
    let ___end0 = ___1.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action70(
        errors,
        ___0,
        ___1,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action94<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Option<Label>, usize),
) -> ProcessStep
{
    let ___start0 = ___1.2;
    let ___end0 = ___1.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action71(
        errors,
        ___0,
        ___1,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action95<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<Definition>, usize),
) -> Program
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action33(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action72(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action96<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, CaseBranch, usize),
) -> Vec<CaseBranch>
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.2;
    let ___temp0 = ___action39(
        errors,
        ___0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action48(
        errors,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action97<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> Vec<CaseBranch>
{
    let ___start0 = *___lookbehind;
    let ___end0 = *___lookahead;
    let ___temp0 = ___action40(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action48(
        errors,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action98<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<CaseBranch>, usize),
    ___1: (usize, CaseBranch, usize),
) -> Vec<CaseBranch>
{
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action39(
        errors,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action49(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action99<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<CaseBranch>, usize),
) -> Vec<CaseBranch>
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action40(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action49(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action100<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> Program
{
    let ___start0 = *___lookbehind;
    let ___end0 = *___lookahead;
    let ___temp0 = ___action34(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action95(
        errors,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action101<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, alloc::vec::Vec<Definition>, usize),
) -> Program
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.2;
    let ___temp0 = ___action35(
        errors,
        ___0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action95(
        errors,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action102<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Label, usize),
    ___4: (usize, Token, usize),
    ___5: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___6: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___3.0;
    let ___end0 = ___3.2;
    let ___temp0 = ___action31(
        errors,
        ___3,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action88(
        errors,
        ___0,
        ___1,
        ___2,
        ___temp0,
        ___4,
        ___5,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action103<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___5: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___2.2;
    let ___end0 = ___3.0;
    let ___temp0 = ___action32(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action88(
        errors,
        ___0,
        ___1,
        ___2,
        ___temp0,
        ___3,
        ___4,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action104<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Label, usize),
) -> ProcessStep
{
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action31(
        errors,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action93(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action105<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action32(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action93(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action106<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, Label, usize),
) -> ProcessStep
{
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action31(
        errors,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action94(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action107<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___0.2;
    let ___end0 = ___0.2;
    let ___temp0 = ___action32(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action94(
        errors,
        ___0,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action108<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, Token, usize),
) -> CaseBranch
{
    let ___start0 = ___3.2;
    let ___end0 = ___4.0;
    let ___temp0 = ___action29(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action73(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action109<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, Token, usize),
    ___1: (usize, LocalIdent, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___5: (usize, Token, usize),
) -> CaseBranch
{
    let ___start0 = ___4.0;
    let ___end0 = ___4.2;
    let ___temp0 = ___action30(
        errors,
        ___4,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action73(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action110<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Label, usize),
    ___4: (usize, Token, usize),
    ___5: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___4.2;
    let ___end0 = ___5.0;
    let ___temp0 = ___action29(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action102(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___temp0,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action111<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Label, usize),
    ___4: (usize, Token, usize),
    ___5: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___6: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___5.0;
    let ___end0 = ___5.2;
    let ___temp0 = ___action30(
        errors,
        ___5,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action102(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___temp0,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action112<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___3.2;
    let ___end0 = ___4.0;
    let ___temp0 = ___action29(
        errors,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action103(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn ___action113<
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, Token, LexicalError>>,
    ___0: (usize, LocalIdent, usize),
    ___1: (usize, Token, usize),
    ___2: (usize, Token, usize),
    ___3: (usize, Token, usize),
    ___4: (usize, alloc::vec::Vec<ProcessStep>, usize),
    ___5: (usize, Token, usize),
) -> ProcessStep
{
    let ___start0 = ___4.0;
    let ___end0 = ___4.2;
    let ___temp0 = ___action30(
        errors,
        ___4,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action103(
        errors,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
        ___5,
    )
}

#[allow(clippy::type_complexity, dead_code)]
pub trait ___ToTriple<'err, >
{
    fn to_triple(self) -> Result<(usize,Token,usize), ___lalrpop_util::ParseError<usize, Token, LexicalError>>;
}

impl<'err, > ___ToTriple<'err, > for (usize, Token, usize)
{
    fn to_triple(self) -> Result<(usize,Token,usize), ___lalrpop_util::ParseError<usize, Token, LexicalError>> {
        Ok(self)
    }
}
impl<'err, > ___ToTriple<'err, > for Result<(usize, Token, usize), LexicalError>
{
    fn to_triple(self) -> Result<(usize,Token,usize), ___lalrpop_util::ParseError<usize, Token, LexicalError>> {
        self.map_err(|error| ___lalrpop_util::ParseError::User { error })
    }
}
