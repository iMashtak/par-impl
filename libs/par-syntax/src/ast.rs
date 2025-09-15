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
        l: usize,
        r: usize,
    },
    ProcessExpr {
        name: GlobalIdent,
        expr: ProcessExpr,
        l: usize,
        r: usize,
    },
}

// ---
// Process Syntax
// ---

#[derive(Debug)]
pub enum ProcessStep {
    Let {
        name: LocalIdent,
        expr: ProcessExpr,
        l: usize,
        r: usize,
    },
    Link {
        target: LocalIdent,
        expr: ProcessExpr,
        l: usize,
        r: usize,
    },
    Begin {
        target: LocalIdent,
        label: Option<Label>,
        steps: Vec<ProcessStep>,
        l: usize,
        r: usize,
    },
    Case {
        target: LocalIdent,
        alts: Vec<CaseBranch>,
        l: usize,
        r: usize,
    },
    Recv {
        target: LocalIdent,
        name: LocalIdent,
        l: usize,
        r: usize,
    },
    Send {
        target: LocalIdent,
        expr: ProcessExpr,
        l: usize,
        r: usize,
    },
    Signal {
        target: LocalIdent,
        signal: LocalIdent,
        l: usize,
        r: usize,
    },
    Loop {
        label: Option<Label>,
        l: usize,
        r: usize,
    },
    Break {
        label: Option<Label>,
        l: usize,
        r: usize,
    },
}

#[derive(Debug)]
pub enum ProcessExpr {
    Chan {
        name: LocalIdent,
        steps: Vec<ProcessStep>,
        l: usize,
        r: usize,
    },
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
    Unit {
        l: usize,
        r: usize,
    },
}

#[derive(Debug)]
pub struct CaseBranch {
    pub name: LocalIdent,
    pub inner: Vec<ProcessStep>,
    pub l: usize,
    pub r: usize,
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
