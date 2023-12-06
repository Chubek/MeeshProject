/* Abstract Syntax Tree for POSIX Shell in Rust
 * IEEE FIPS --- Portable OS Interface --- Base Specs Issue. 7 --- XCU
 * Author: Chubak Bidpaa (chubakbidpaa@gmail.com) (Discord: .chubak)
*/

type Token = String;
type Pattern = String;
type Sigil = i8;
type Operator = i8;
type Dub = bool;
type Position = u32;
type FileDesc = i32;

enum Expression {
    Unary(Operator, Token),
    Binary(Token, Operator, Token)
}

enum Parameter {
    Positional(Position),
    Special(Sigil),
    Variable(Token),
    Name(Token)
}

enum ParametricExpansion {
    Expressive(Expression),
    Commandic(CompoundCommand),
    Simple(Parameter),
    Colonial(Sigil, Option<Token>),
    NonColonial(Sigil, Option<Token>),
    SubsParam(Parameter, Sigil, Dub, Option<Token>),
}

enum Expansion {
    Tilde,
    Parametric(ParametricExpansion),
    Commandic(CompoundCommand),
    Arithmetic(Expression),
    Pathname(Pattern),
    NormalText(Token)
}

enum Word {
    SingleQuoted(Token),
    DoubleQuoted(Vec<Expansion>),
    ToExpand(Expansion),
    Slashed(Token),
    Unquoted(Token),
    FDesc(i32),
}


enum Redirection {
    FromInToOut(Option<Word>, Option<FileDesc>),
    FromOutToIn(Option<Word>, Option<FileDesc>),
    AppendInToOut(Option<Word>, Option<FileDesc>),
    AppendOutToIn(Option<Word>, Option<FileDesc>),
    HereStr(Word),
    HereDoc(Word),
}

enum FileDuplication {
    Input(FileDesc, Word),
    Output(FileDesc, Word),
    ReadingAndWriting(FileDesc, Word),
}

enum AtomicCommand {
    VarAssign(Parameter, Option<Word>),
    Redir(Redirection),
    Fdup(FileDuplication),
    MiscWord(Word),
    FunctionCall(Parameter),
}

enum SimpleCommand {
    Thunk(Vec<AtomicCommand>),
    Root(AtomicCommand),
    Trunk(AtomicCommand),
}

enum ListCommand {
    And(SimpleCommand, SimpleCommand),
    Or(SimpleCommand, SimpleCommand),
    Sequential(Vec<SimpleCommand>),
    Parallel(Vec<SimpleCommand>),
}

enum ComplexCommand {
    Pipeline(Vec<SimpleCommand>),
    List(Vec<ListCommand>),
}

enum CompoundCommand {
    Subshell(ComplexCommand),
    CurrentShell(ComplexCommand),
    CompoundList(Vec<ComplexCommand>),
}

enum Statements {
    IfBlock { 
                condition: CompoundCommand,
                elifs: Option<(CompoundCommand, Vec<CompoundCommand>)>,
                stmts: Option<Vec<CompoundCommand>>,
                otherwise: Vec<CompoundCommand>,
    },
    ForBlock { 
        param: Parameter, 
        words: Vec<Word>,
        cmds: Vec<CompoundCommand>
    },
    CaseBlock {
        word: Word,
        cases: Vec<(Pattern, Vec<CompoundCommand>)>
    },
    While {
        cond: CompoundCommand,
        body: Vec<CompoundCommand>,
        until: bool
    },
    FunctionBlock {
        name: Parameter,
        body: Vec<CompoundCommand>
    }
}




