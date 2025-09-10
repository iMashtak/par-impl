use arcstr::ArcStr;

// ---
// File root
// ---

#[derive(Debug)]
pub struct Program {
    pub defs: Vec<Definition>,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub enum Definition {
    Native {
        name: GlobalIdent,
        typing: TypeExpr,
        l: usize,
        r: usize,
    },
    Value {
        name: GlobalIdent,
        typing: Option<TypeExpr>,
        value: ValueExpr,
        l: usize,
        r: usize,
    },
    Type {
        name: GlobalIdent,
        expr: TypeExpr,
        l: usize,
        r: usize,
    },
}

// ---
// Types
// ---

#[derive(Debug)]
pub enum TypeExpr {
    Error,
    Unit {
        l: usize,
        r: usize,
    },
    Self_ {
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    Ident {
        ident: Ident,
        inner: Option<Box<TypeExpr>>,
        l: usize,
        r: usize,
    },
    Box {
        inner: Box<TypeExpr>,
        l: usize,
        r: usize,
    },
    Dual {
        inner: Box<TypeExpr>,
        l: usize,
        r: usize,
    },
    Tuple {
        inners: Vec<TypeExpr>,
        outer: Box<TypeExpr>,
        l: usize,
        r: usize,
    },
    Function {
        inners: Vec<TypeExpr>,
        outer: Box<TypeExpr>,
        l: usize,
        r: usize,
    },
    Exist {
        inners: Vec<TypeExpr>,
        outer: Option<Box<TypeExpr>>,
        l: usize,
        r: usize,
    },
    ForAll {
        inners: Vec<TypeExpr>,
        outer: Option<Box<TypeExpr>>,
        l: usize,
        r: usize,
    },
    Iterative {
        label: Option<Label>,
        inner: Box<TypeExpr>,
        l: usize,
        r: usize,
    },
    Recursive {
        label: Option<Label>,
        inner: Box<TypeExpr>,
        l: usize,
        r: usize,
    },
    Either {
        branches: Vec<EitherBranchTypeExpr>,
        l: usize,
        r: usize,
    },
    Choice {
        branches: Vec<ChoiceBranchTypeExpr>,
        l: usize,
        r: usize,
    },
}

#[derive(Debug)]
pub struct EitherBranchTypeExpr {
    pub name: LocalIdent,
    pub inner: Box<TypeExpr>,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub struct ChoiceBranchTypeExpr {
    pub name: LocalIdent,
    pub inner: Box<TypeExpr>,
    pub l: usize,
    pub r: usize,
}

// ---
// Values
// ---

#[derive(Debug)]
pub enum ValueExpr {
    Error,
    Str {
        value: ArcStr,
        l: usize,
        r: usize,
    },
    I32 {
        value: i32,
        l: usize,
        r: usize,
    },
    U64 {
        value: u64,
        l: usize,
        r: usize,
    },
    Ident {
        value: Ident,
        l: usize,
        r: usize,
    },
    Let {
        name: LocalIdent,
        typing: Option<TypeExpr>,
        inner: Box<ValueExpr>,
        outer: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Signal {
        expr: Option<Box<ValueExpr>>,
        name: LocalIdent,
        l: usize,
        r: usize,
    },
    Application {
        expr: Box<ValueExpr>,
        applied: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    ConsumingBegin {
        expr: Box<ValueExpr>,
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    ConsumingLoop {
        expr: Box<ValueExpr>,
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    ConsumingCase {
        expr: Box<ValueExpr>,
        branches: Vec<ConsumingCaseBranchValueExpr>,
        l: usize,
        r: usize,
    },
    Begin {
        expr: Box<ValueExpr>,
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    Loop {
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    Case {
        branches: Vec<CaseBranchValueExpr>,
        l: usize,
        r: usize,
    },
    Box {
        expr: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Do {
        process: Vec<ProcessStep>,
        expr: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Chan {
        name: LocalIdent,
        process: Vec<ProcessStep>,
        l: usize,
        r: usize,
    },
    Break {
        l: usize,
        r: usize,
    },
    LambdaTuple {
        inners: Vec<ValueExpr>,
        outer: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    LambdaFunction {
        inners: Vec<LocalIdent>,
        outer: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    LambdaExist {
        inners: Vec<TypeExpr>,
        outer: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    LambdaForAll {
        inners: Vec<LocalIdent>,
        outer: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    ApplicationTuple {
        expr: Box<ValueExpr>,
        args: Vec<ValueExpr>,
        l: usize,
        r: usize,
    },
    ApplicationForAll {
        expr: Box<ValueExpr>,
        args: Vec<TypeExpr>,
        l: usize,
        r: usize,
    },
    ApplicationExist {
        expr: Box<ValueExpr>,
        args: Vec<LocalIdent>,
        l: usize,
        r: usize,
    },
    UnaryMinus {
        expr: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Not {
        expr: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Mul {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Div {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Mod {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Plus {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    BinaryMinus {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Eq {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    NotEq {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Lt {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Gt {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Le {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Ge {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    And {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
    Or {
        left: Box<ValueExpr>,
        right: Box<ValueExpr>,
        l: usize,
        r: usize,
    },
}

#[derive(Debug)]
pub struct CaseBranchValueExpr {
    pub name: LocalIdent,
    pub inner: Box<ValueExpr>,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub struct ConsumingCaseBranchValueExpr {
    pub name: LocalIdent,
    pub pattern: PatternExpr,
    pub inner: Box<ValueExpr>,
    pub l: usize,
    pub r: usize,
}

// ---
// Process
// ---

#[derive(Debug)]
pub enum ProcessStep {
    Let {
        pattern: PatternExpr,
        expr: ProcessExpr,
        l: usize,
        r: usize,
    },
    Recv {
        name: LocalIdent,
        expr: ProcessExpr,
        l: usize,
        r: usize,
    },
    Expr {
        expr: ProcessExpr,
        l: usize,
        r: usize,
    }
}

#[derive(Debug)]
pub enum ProcessExpr {
    Error,
    Str {
        value: ArcStr,
        l: usize,
        r: usize,
    },
    I32 {
        value: i32,
        l: usize,
        r: usize,
    },
    U64 {
        value: u64,
        l: usize,
        r: usize,
    },
    Ident {
        value: Ident,
        l: usize,
        r: usize,
    },
    Signal {
        expr: Option<Box<ProcessExpr>>,
        name: LocalIdent,
        l: usize,
        r: usize,
    },
    Application {
        expr: Box<ProcessExpr>,
        applied: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    ConsumingBegin {
        expr: Box<ProcessExpr>,
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    ConsumingLoop {
        expr: Box<ProcessExpr>,
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    ConsumingCase {
        expr: Box<ProcessExpr>,
        branches: Vec<ConsumingCaseBranchProcessExpr>,
        l: usize,
        r: usize,
    },
    Begin {
        expr: Box<ProcessExpr>,
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    Loop {
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    Case {
        branches: Vec<CaseBranchProcessExpr>,
        l: usize,
        r: usize,
    },
    Box {
        expr: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Break {
        l: usize,
        r: usize,
    },
    LambdaFunction {
        inners: Vec<LocalIdent>,
        outer: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    LambdaExist {
        inners: Vec<TypeExpr>,
        outer: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    LambdaForAll {
        inners: Vec<LocalIdent>,
        outer: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    ApplicationTuple {
        expr: Box<ProcessExpr>,
        args: Vec<ProcessExpr>,
        l: usize,
        r: usize,
    },
    ApplicationForAll {
        expr: Box<ProcessExpr>,
        args: Vec<TypeExpr>,
        l: usize,
        r: usize,
    },
    ApplicationExist {
        expr: Box<ProcessExpr>,
        args: Vec<LocalIdent>,
        l: usize,
        r: usize,
    },
    UnaryMinus {
        expr: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Not {
        expr: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Mul {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Div {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Mod {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Plus {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    BinaryMinus {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Eq {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    NotEq {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Lt {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Gt {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Le {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Ge {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    And {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
    Or {
        left: Box<ProcessExpr>,
        right: Box<ProcessExpr>,
        l: usize,
        r: usize,
    },
}

#[derive(Debug)]
pub struct CaseBranchProcessExpr {
    pub name: LocalIdent,
    pub inner: Vec<ProcessStep>,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub struct ConsumingCaseBranchProcessExpr {
    pub name: LocalIdent,
    pub pattern: PatternExpr,
    pub inner: Vec<ProcessStep>,
    pub l: usize,
    pub r: usize,
}

// ---
// Pattern
// ---

#[derive(Debug)]
pub enum PatternExpr {
    Error,
    Unit {
        l: usize,
        r: usize,
    },
    LocalIdent {
        name: LocalIdent,
        l: usize,
        r: usize,
    },
    Tuple {
        inners: Vec<PatternExpr>,
        outer: Box<PatternExpr>,
        l: usize,
        r: usize,
    },
}

// ---
// Primitives
// ---

#[derive(Debug)]
pub struct Label {
    pub name: LocalIdent,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub struct GlobalIdent {
    pub content: ArcStr,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub struct LocalIdent {
    pub content: ArcStr,
    pub l: usize,
    pub r: usize,
}

#[derive(Debug)]
pub enum Ident {
    Local(LocalIdent),
    Global(GlobalIdent),
}

#[derive(Debug)]
pub struct Comment {
    pub content: ArcStr,
    pub l: usize,
    pub r: usize,
}
