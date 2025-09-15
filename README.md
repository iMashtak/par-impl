# Par Implementation

Origin: [Par Lang](https://github.com/faiface/par-lang).

This repo contains alternative implementation of Par language.

## Execution

To run an example project:

```bash
cargo run -p parc -- examples/simple_project
```

It will transpile Par code from that directory to Rust project in relative `target` directory. Then you should build generated Rust project and run run it:

```bash
cd examples/simple_project/target
cargo run
```

On every transpilation files are removed from `target`, so to rerun generated project without messing with `cd` you my just run:

```bash
cd . && cargo run
```

Be careful - there is no typechecking! If your program is not correct, either generated Rust project will not compile or it will stuck forever.

## Parsing

### Why?

There are three main intensions for me:

- i wanted to dive deep into language constructs, understand it not from documentation but from real work;
- original grammar do not support unary/binary operators which are essential for any programming language;
- i've seen some solutions that doesn't make good for language, for example unnecessary sugar or verbose constructions or constructions that are really hard to read and understand.

Parser is built with [`lalrpop`](https://lalrpop.github.io/lalrpop/index.html) crate. Grammar is located at [./libs/par-syntax/parser.lalrpop](./libs/par-syntax/parser.lalrpop).

### Syntax differences

All made changes may be concern as my personal vision of good programming language. I have no illusion that all changes i made are absolutely positive, but:

- to be less verbose and have a division between type and value arguments, type arguments now should be declared in `<...>` braces without `type` keyword:
    ```rust
    type Id = <a> [a] a
    ```
    existential types accepts arguments in curly braces form:
    ```rust
    type Any = {a} a

    type DropMe = {a} (a) choice {
        .drop: [a] !,
    }
    ```
- to be consistent with div operator, `/`-label is changed to `@`-label:
    ```rust
    type Queue = <a>
        iterative@push recursive@pop choice {
            ... self@push ... self@pop
        }
    ```
- to be more readable, `choice` and `either` now have delimiter `:` between name and type instead of nothing;
- to not to be overshugared, `choice` and `either` are deprived of `(a)` syntax, for now it needs to explicitly write `[a]` after `:`, mainly because this syntax is too related to tuple syntax which is confuses:
    ```rust
    type Queue = <a>
        iterative@push recursive@pop
        choice {
            .push: [a] self@push,
            .pop: either {
                .end: !,
                .item: [a] self@pop,
            }
        }
    ```
- to make strict separation of definition type signature and get rid of repeated `dec X ... def X`, type signature now should be declared after name of definition:
    ```rust
    def Replace: [String] String = ...
    ```
- to remove similarity with array/map indexing, receiving of values now have syntax:
    ```rust
    recv str <- strings
    ```
    instead of:
    ```rust
    strings[str]
    ```
    while it is understandable that this syntax is inspired by pi-calculus, array/map indexing is far more adopted concept in programming languages;
- to make grammar correct and able to support operator-expressions, in process syntax each command must be semicolon-separated:
    ```rust
    def LetsBuildStrings = do {
        let builder = StringBuilder;
        builder.add("Hello");
        builder.add(", ").add2(",")(":").add2(",", ":").add("1");
        builder.add("World");
        builder.add("!");
    } in builder.build
    ```
- to make syntax consistent with previous changes, case expressions now have explicit "pattern matching"-section delimited by `:` with respect of duality:
    ```rust
    def PrintSeq: [Seq<String>] ! = [seq] do {
        seq.begin.case {
            .end: ! => {},
            .item: (x) xs => {
                console.print(x);
                xs.loop;
            }
        };
    } in !
    ```
- to remove parsing ambiguity of type args `List<String>` and lt operator `<`, type args sending must have `::` prefix:
    ```rust
    def Main: ! = do {
        let seq = StaticSeq::<String>.builder.add("x").add("y").build < "xy";
        let console = ConsoleOpen;
        console.print(seq);
        console.close;
    } in !
    ```
- to remove parsing ambiguity with `!` symbol, logical operators are named as words `and`, `or` and `not`;
- to make grammar correct and remove ambiguity with complex operator expressions, pair construction needs `.` symbol:
    ```rust
    def Main: ! = do {
        let pair = .(x) y;
        let pair = (.(x) y); // still correct, we've just wrapped pair in parens
        ...
    } in !
    ```
- to make grammar correct and remove ambiguity with complex operator expressions, application needs `<-` symbol:
    ```rust
    def Reverse: <a> [List<a>] List<a> = <a> [list]
        let acc: List<a> = .end <- !
        in list.begin.case {
            .end: ! => acc,
            .item: (x) xs => let acc = .item(x) <- acc in xs.loop,
        }
    ```
