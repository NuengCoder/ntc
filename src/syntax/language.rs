use crate::syntax::types::SyntaxLanguage;

pub(super) fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

pub(super) fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

pub(super) fn line_comment_prefix(lang: SyntaxLanguage) -> &'static [u8] {
    match lang {
        SyntaxLanguage::Python
        | SyntaxLanguage::Ruby
        | SyntaxLanguage::Bash
        | SyntaxLanguage::Perl
        | SyntaxLanguage::R
        | SyntaxLanguage::Julia
        | SyntaxLanguage::Yaml
        | SyntaxLanguage::Pro
        | SyntaxLanguage::Dotenv
        | SyntaxLanguage::Ps1 => b"#",
        SyntaxLanguage::Lua | SyntaxLanguage::Haskell | SyntaxLanguage::Elixir => {
            b"--"
        }
        SyntaxLanguage::Erlang => b"%",
        SyntaxLanguage::Bat => b"::",
        SyntaxLanguage::Html | SyntaxLanguage::Json | SyntaxLanguage::Markdown | SyntaxLanguage::Txt | SyntaxLanguage::Css => {
            b""
        }
        _ => b"//",
    }
}

pub(super) fn has_block_comments(lang: SyntaxLanguage) -> bool {
    matches!(
        lang,
        SyntaxLanguage::Rust
            | SyntaxLanguage::C
            | SyntaxLanguage::Cpp
            | SyntaxLanguage::CSharp
            | SyntaxLanguage::Java
            | SyntaxLanguage::Kotlin
            | SyntaxLanguage::Swift
            | SyntaxLanguage::Dart
            | SyntaxLanguage::JavaScript
            | SyntaxLanguage::TypeScript
            | SyntaxLanguage::Go
            | SyntaxLanguage::Scala
            | SyntaxLanguage::Zig
            | SyntaxLanguage::Julia
            | SyntaxLanguage::GoMod
            | SyntaxLanguage::Css
            | SyntaxLanguage::Ps1
    )
}

pub(super) fn has_xml_comments(lang: SyntaxLanguage) -> bool {
    matches!(lang, SyntaxLanguage::Xml | SyntaxLanguage::Html)
}

pub(super) fn has_ps_block_comments(lang: SyntaxLanguage) -> bool {
    matches!(lang, SyntaxLanguage::Ps1)
}

pub(super) fn is_char_literal_supported(lang: SyntaxLanguage) -> bool {
    matches!(
        lang,
        SyntaxLanguage::Rust
            | SyntaxLanguage::C
            | SyntaxLanguage::Cpp
            | SyntaxLanguage::CSharp
            | SyntaxLanguage::Java
            | SyntaxLanguage::Dart
            | SyntaxLanguage::Zig
            | SyntaxLanguage::Julia
    )
}

pub(super) fn has_regex_literals(lang: SyntaxLanguage) -> bool {
    matches!(lang, SyntaxLanguage::JavaScript | SyntaxLanguage::TypeScript)
}

pub(super) fn has_tag_syntax(lang: SyntaxLanguage) -> bool {
    matches!(
        lang,
        SyntaxLanguage::TypeScript
            | SyntaxLanguage::JavaScript
            | SyntaxLanguage::Xml
            | SyntaxLanguage::Html
    )
}

pub(super) fn is_rust_attr_start(lang: SyntaxLanguage) -> bool {
    matches!(lang, SyntaxLanguage::Rust)
}

pub(super) fn has_annotation_at(lang: SyntaxLanguage) -> bool {
    matches!(
        lang,
        SyntaxLanguage::Java
            | SyntaxLanguage::Kotlin
            | SyntaxLanguage::Swift
            | SyntaxLanguage::Python
            | SyntaxLanguage::CSharp
            | SyntaxLanguage::Scala
    )
}

pub(super) fn has_css_at_rules(lang: SyntaxLanguage) -> bool {
    matches!(lang, SyntaxLanguage::Css)
}

pub(super) fn is_css_at_rule(word: &str) -> bool {
    matches!(
        word,
        "@media" | "@import" | "@keyframes" | "@font-face" | "@page"
            | "@supports" | "@namespace" | "@document" | "@charset"
            | "@counter-style" | "@property" | "@layer" | "@scope"
            | "@starting-style" | "@container"
    )
}

pub(super) fn is_css_property(word: &str) -> bool {
    matches!(
        word,
        "color" | "background" | "background-color" | "background-image"
            | "background-size" | "background-position" | "background-repeat"
            | "margin" | "margin-top" | "margin-right" | "margin-bottom" | "margin-left"
            | "padding" | "padding-top" | "padding-right" | "padding-bottom" | "padding-left"
            | "font" | "font-size" | "font-weight" | "font-family" | "font-style"
            | "display" | "position" | "top" | "right" | "bottom" | "left"
            | "width" | "height" | "min-width" | "min-height" | "max-width" | "max-height"
            | "border" | "border-color" | "border-width" | "border-style" | "border-radius"
            | "outline" | "box-shadow" | "text-shadow"
            | "overflow" | "overflow-x" | "overflow-y"
            | "z-index" | "opacity" | "visibility" | "cursor"
            | "transform" | "transition" | "animation"
            | "flex" | "flex-direction" | "flex-wrap" | "flex-grow" | "flex-shrink" | "flex-basis"
            | "align-items" | "align-content" | "align-self"
            | "justify-content" | "gap" | "row-gap" | "column-gap"
            | "grid" | "grid-template" | "grid-column" | "grid-row"
            | "list-style" | "text-align" | "text-decoration" | "text-transform"
            | "white-space" | "word-break" | "word-wrap" | "line-height" | "letter-spacing"
            | "vertical-align" | "float" | "clear"
            | "content" | "counter-increment" | "counter-reset"
            | "user-select" | "pointer-events" | "box-sizing"
            | "filter" | "backdrop-filter" | "clip-path" | "mask"
            | "scroll-behavior" | "scrollbar-width" | "scrollbar-color"
            | "accent-color" | "caret-color" | "color-scheme"
            | "fill" | "stroke" | "stroke-width" | "stroke-linecap"
    )
}

pub(super) fn is_keyword(lang: SyntaxLanguage, word: &str) -> bool {
    match lang {
        SyntaxLanguage::Rust => matches!(
            word,
            "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn"
                | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl"
                | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub"
                | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
                | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while"
                | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override"
                | "priv" | "try" | "typeof" | "unsized" | "virtual" | "yield"
        ),
        SyntaxLanguage::Python => matches!(
            word,
            "False" | "None" | "True" | "and" | "as" | "assert" | "async" | "await"
                | "break" | "class" | "continue" | "def" | "del" | "elif" | "else"
                | "except" | "finally" | "for" | "from" | "global" | "if" | "import"
                | "in" | "is" | "lambda" | "nonlocal" | "not" | "or" | "pass" | "raise"
                | "return" | "try" | "while" | "with" | "yield"
        ),
        SyntaxLanguage::JavaScript | SyntaxLanguage::TypeScript => matches!(
            word,
            "async" | "await" | "break" | "case" | "catch" | "class" | "const"
                | "continue" | "debugger" | "default" | "delete" | "do" | "else"
                | "enum" | "export" | "extends" | "false" | "finally" | "for"
                | "function" | "if" | "import" | "in" | "instanceof" | "let"
                | "new" | "null" | "of" | "return" | "super" | "switch" | "this"
                | "throw" | "true" | "try" | "typeof" | "var" | "void" | "while"
                | "with" | "yield" | "static" | "get" | "set" | "implements" | "interface"
                | "package" | "private" | "protected" | "public" | "readonly"
        ),
        SyntaxLanguage::Go => matches!(
            word,
            "break" | "case" | "chan" | "const" | "continue" | "default" | "defer"
                | "else" | "fallthrough" | "for" | "func" | "go" | "goto" | "if"
                | "import" | "interface" | "map" | "package" | "range" | "return"
                | "select" | "struct" | "switch" | "type" | "var" | "true" | "false"
                | "nil"
        ),
        SyntaxLanguage::C => matches!(
            word,
            "auto" | "break" | "case" | "char" | "const" | "continue" | "default"
                | "do" | "double" | "else" | "enum" | "extern" | "float" | "for"
                | "goto" | "if" | "inline" | "int" | "long" | "register" | "return"
                | "short" | "signed" | "sizeof" | "static" | "struct" | "switch"
                | "typedef" | "union" | "unsigned" | "void" | "volatile" | "while"
                | "_Bool" | "_Complex" | "_Imaginary"
        ),
        SyntaxLanguage::Cpp => matches!(
            word,
            "alignas" | "alignof" | "auto" | "bool" | "break" | "case" | "catch"
                | "char" | "class" | "const" | "constexpr" | "continue" | "decltype"
                | "default" | "delete" | "do" | "double" | "else" | "enum" | "explicit"
                | "export" | "extern" | "false" | "float" | "for" | "friend" | "goto"
                | "if" | "inline" | "int" | "long" | "mutable" | "namespace" | "new"
                | "noexcept" | "nullptr" | "operator" | "override" | "private"
                | "protected" | "public" | "register" | "return" | "short" | "signed"
                | "sizeof" | "static" | "struct" | "switch" | "template" | "this"
                | "throw" | "true" | "try" | "typedef" | "typeid" | "typename"
                | "union" | "unsigned" | "using" | "virtual" | "void" | "volatile"
                | "while"
        ),
        SyntaxLanguage::CSharp => matches!(
            word,
            "abstract" | "as" | "base" | "bool" | "break" | "byte" | "case" | "catch"
                | "char" | "checked" | "class" | "const" | "continue" | "decimal"
                | "default" | "delegate" | "do" | "double" | "else" | "enum" | "event"
                | "explicit" | "extern" | "false" | "finally" | "fixed" | "float"
                | "for" | "foreach" | "goto" | "if" | "implicit" | "in" | "int"
                | "interface" | "internal" | "is" | "lock" | "long" | "namespace"
                | "new" | "null" | "object" | "operator" | "out" | "override"
                | "params" | "private" | "protected" | "public" | "readonly" | "ref"
                | "return" | "sbyte" | "sealed" | "short" | "sizeof" | "stackalloc"
                | "static" | "string" | "struct" | "switch" | "this" | "throw" | "true"
                | "try" | "typeof" | "uint" | "ulong" | "unchecked" | "unsafe"
                | "ushort" | "using" | "var" | "virtual" | "void" | "volatile" | "while"
        ),
        SyntaxLanguage::Java => matches!(
            word,
            "abstract" | "assert" | "boolean" | "break" | "byte" | "case" | "catch"
                | "char" | "class" | "const" | "continue" | "default" | "do" | "double"
                | "else" | "enum" | "extends" | "false" | "final" | "finally" | "float"
                | "for" | "goto" | "if" | "implements" | "import" | "instanceof" | "int"
                | "interface" | "long" | "native" | "new" | "null" | "package" | "private"
                | "protected" | "public" | "return" | "short" | "static" | "strictfp"
                | "super" | "switch" | "synchronized" | "this" | "throw" | "throws"
                | "transient" | "true" | "try" | "void" | "volatile" | "while" | "var"
                | "record" | "sealed" | "permits" | "yield"
        ),
        SyntaxLanguage::Kotlin => matches!(
            word,
            "abstract" | "annotation" | "as" | "break" | "by" | "catch" | "class"
                | "companion" | "const" | "continue" | "crossinline" | "data" | "do"
                | "else" | "enum" | "false" | "final" | "finally" | "for" | "fun"
                | "if" | "import" | "in" | "inline" | "inner" | "interface" | "is"
                | "lateinit" | "noinline" | "null" | "object" | "open" | "operator"
                | "out" | "override" | "package" | "private" | "protected" | "public"
                | "reified" | "return" | "sealed" | "super" | "suspend" | "tailrec"
                | "this" | "throw" | "true" | "try" | "typealias" | "val" | "var"
                | "vararg" | "when" | "while"
        ),
        SyntaxLanguage::Swift => matches!(
            word,
            "Any" | "as" | "associativity" | "break" | "case" | "catch" | "class"
                | "continue" | "convenience" | "default" | "defer" | "deinit" | "do"
                | "dynamic" | "else" | "enum" | "extension" | "fallthrough" | "false"
                | "fileprivate" | "final" | "for" | "func" | "get" | "guard" | "if"
                | "import" | "in" | "indirect" | "infix" | "init" | "inout" | "internal"
                | "is" | "lazy" | "let" | "mutating" | "nil" | "nonmutating" | "open"
                | "operator" | "optional" | "override" | "package" | "postfix" | "precedence"
                | "prefix" | "private" | "protocol" | "public" | "repeat" | "required"
                | "rethrows" | "return" | "self" | "Self" | "set" | "static" | "struct"
                | "subscript" | "super" | "switch" | "throw" | "throws" | "true" | "try"
                | "typealias" | "var" | "where" | "while"
        ),
        SyntaxLanguage::Dart => matches!(
            word,
            "abstract" | "as" | "assert" | "async" | "await" | "break" | "case"
                | "catch" | "class" | "const" | "continue" | "covariant" | "default"
                | "deferred" | "do" | "dynamic" | "else" | "enum" | "export" | "extends"
                | "extension" | "external" | "false" | "final" | "finally" | "for"
                | "Function" | "get" | "hide" | "if" | "implements" | "import" | "in"
                | "interface" | "is" | "late" | "library" | "mixin" | "new" | "null"
                | "on" | "operator" | "part" | "required" | "rethrow" | "return" | "set"
                | "show" | "static" | "super" | "switch" | "sync" | "this" | "throw"
                | "true" | "try" | "typedef" | "var" | "void" | "while" | "with" | "yield"
        ),
        SyntaxLanguage::Ruby => matches!(
            word,
            "BEGIN" | "END" | "alias" | "and" | "begin" | "break" | "case" | "class"
                | "def" | "defined?" | "do" | "else" | "elsif" | "end" | "ensure"
                | "false" | "for" | "if" | "in" | "module" | "next" | "nil" | "not"
                | "or" | "redo" | "rescue" | "retry" | "return" | "self" | "super"
                | "then" | "true" | "undef" | "unless" | "until" | "when" | "while"
                | "yield"
        ),
        SyntaxLanguage::Bash => matches!(
            word,
            "case" | "do" | "done" | "elif" | "else" | "esac" | "fi" | "for"
                | "function" | "if" | "in" | "select" | "then" | "time" | "until"
                | "while"
        ),
        SyntaxLanguage::Lua => matches!(
            word,
            "and" | "break" | "do" | "else" | "elseif" | "end" | "false" | "for"
                | "function" | "goto" | "if" | "in" | "local" | "nil" | "not" | "or"
                | "repeat" | "return" | "then" | "true" | "until" | "while"
        ),
        SyntaxLanguage::Php => matches!(
            word,
            "abstract" | "and" | "array" | "as" | "break" | "callable" | "case"
                | "catch" | "class" | "clone" | "const" | "continue" | "declare"
                | "default" | "die" | "do" | "echo" | "else" | "elseif" | "empty"
                | "enddeclare" | "endfor" | "endforeach" | "endif" | "endswitch"
                | "endwhile" | "eval" | "exit" | "extends" | "false" | "final"
                | "finally" | "fn" | "for" | "foreach" | "function" | "global" | "goto"
                | "if" | "implements" | "include" | "instanceof" | "interface" | "isset"
                | "list" | "match" | "namespace" | "new" | "null" | "or" | "print"
                | "private" | "protected" | "public" | "readonly" | "return" | "static"
                | "switch" | "throw" | "trait" | "true" | "try" | "unset" | "use" | "var"
                | "while" | "xor" | "yield"
        ),
        SyntaxLanguage::Perl => matches!(
            word,
            "__END__" | "__FILE__" | "__LINE__" | "abs" | "and" | "caller" | "cmp"
                | "continue" | "croak" | "die" | "do" | "else" | "elsif" | "eval"
                | "exit" | "for" | "foreach" | "given" | "goto" | "grep" | "if"
                | "last" | "local" | "map" | "my" | "next" | "no" | "or" | "our"
                | "package" | "print" | "printf" | "redo" | "return" | "say" | "sort"
                | "state" | "sub" | "unless" | "until" | "use" | "wantarray" | "when"
                | "while" | "xor" | "y"
        ),
        SyntaxLanguage::Haskell => matches!(
            word,
            "case" | "class" | "data" | "default" | "deriving" | "do" | "else"
                | "family" | "forall" | "foreign" | "hiding" | "if" | "import"
                | "in" | "infix" | "infixl" | "infixr" | "instance" | "let"
                | "mdo" | "module" | "newtype" | "of" | "packed" | "proc" | "qualified"
                | "rec" | "then" | "type" | "where"
        ),
        SyntaxLanguage::Elixir => matches!(
            word,
            "after" | "and" | "case" | "catch" | "cond" | "def" | "defdelegate"
                | "defexception" | "defguard" | "defimpl" | "defmacro" | "defmodule"
                | "defn" | "defp" | "defprotocol" | "defstruct" | "deftype" | "do"
                | "else" | "end" | "false" | "fn" | "for" | "if" | "in" | "nil"
                | "not" | "or" | "quote" | "raise" | "receive" | "require" | "rescue"
                | "return" | "send" | "super" | "throw" | "true" | "try" | "unquote"
                | "unless" | "unquote_splicing" | "when" | "with"
        ),
        SyntaxLanguage::Zig => matches!(
            word,
            "align" | "allowzero" | "and" | "anyframe" | "anytype" | "asm" | "async"
                | "await" | "break" | "callconv" | "catch" | "comptime" | "const"
                | "continue" | "defer" | "else" | "enum" | "errdefer" | "error"
                | "export" | "extern" | "fn" | "for" | "if" | "inline" | "noalias"
                | "nosuspend" | "or" | "orelse" | "packed" | "pub" | "resume"
                | "return" | "struct" | "suspend" | "switch" | "test" | "try"
                | "union" | "unreachable" | "usingnamespace" | "var" | "volatile"
                | "while"
        ),
        SyntaxLanguage::Nim => matches!(
            word,
            "addr" | "and" | "as" | "asm" | "atomic" | "bind" | "block" | "break"
                | "case" | "cast" | "const" | "continue" | "converter" | "defer"
                | "discard" | "distinct" | "div" | "do" | "elif" | "else" | "end"
                | "enum" | "except" | "export" | "finally" | "for" | "from" | "func"
                | "generic" | "if" | "import" | "in" | "include" | "interface" | "is"
                | "isnot" | "iterator" | "let" | "macro" | "method" | "mixin" | "mod"
                | "nil" | "not" | "notin" | "object" | "of" | "or" | "out" | "proc"
                | "ptr" | "raise" | "ref" | "return" | "shl" | "shr" | "static"
                | "template" | "try" | "tuple" | "type" | "using" | "var" | "when"
                | "while" | "with" | "without" | "xor" | "yield"
        ),
        SyntaxLanguage::Scala => matches!(
            word,
            "abstract" | "case" | "catch" | "class" | "def" | "do" | "else" | "enum"
                | "export" | "extends" | "false" | "final" | "finally" | "for"
                | "forSome" | "given" | "if" | "implicit" | "import" | "lazy" | "match"
                | "new" | "null" | "object" | "override" | "package" | "private"
                | "protected" | "return" | "sealed" | "super" | "then" | "throw" | "trait"
                | "true" | "try" | "type" | "using" | "val" | "var" | "while" | "with"
                | "yield"
        ),
        SyntaxLanguage::Clojure => matches!(
            word,
            "def" | "defn" | "defmacro" | "defmethod" | "defmulti" | "defonce"
                | "defprotocol" | "defrecord" | "deftype" | "do" | "doseq" | "dotimes"
                | "doto" | "fn" | "for" | "if" | "if-let" | "if-not" | "import"
                | "let" | "letfn" | "loop" | "monitor-enter" | "monitor-exit"
                | "ns" | "proxy" | "recur" | "reify" | "require" | "set!"
                | "try" | "when" | "when-first" | "when-let" | "when-not" | "while"
        ),
        SyntaxLanguage::OCaml => matches!(
            word,
            "and" | "as" | "assert" | "begin" | "class" | "constraint" | "do"
                | "done" | "downto" | "else" | "end" | "exception" | "external"
                | "false" | "for" | "fun" | "function" | "functor" | "if" | "in"
                | "include" | "inherit" | "initializer" | "lazy" | "let" | "match"
                | "method" | "module" | "mutable" | "new" | "nonrec" | "object"
                | "of" | "open" | "or" | "private" | "rec" | "sig" | "struct"
                | "then" | "to" | "true" | "try" | "type" | "val" | "virtual"
                | "when" | "while" | "with"
        ),
        SyntaxLanguage::R => matches!(
            word,
            "break" | "else" | "FALSE" | "for" | "function" | "if" | "Inf"
                | "in" | "NA" | "NaN" | "next" | "NULL" | "repeat" | "return"
                | "TRUE" | "while"
        ),
        SyntaxLanguage::Julia => matches!(
            word,
            "abstract" | "baremodule" | "begin" | "break" | "catch" | "ccall"
                | "const" | "continue" | "do" | "else" | "elseif" | "end"
                | "export" | "false" | "finally" | "for" | "function" | "global"
                | "if" | "import" | "in" | "let" | "local" | "macro" | "module"
                | "mutable" | "primitive" | "quote" | "return" | "struct"
                | "true" | "try" | "using" | "where" | "while"
        ),
        SyntaxLanguage::Erlang => matches!(
            word,
            "after" | "and" | "andalso" | "band" | "begin" | "bnot" | "bor"
                | "bsl" | "bsr" | "bxor" | "case" | "catch" | "cond" | "div"
                | "end" | "fun" | "if" | "let" | "not" | "of" | "or" | "orelse"
                | "receive" | "rem" | "try" | "when" | "xor"
        ),
        SyntaxLanguage::Yaml => matches!(
            word,
            "true" | "True" | "TRUE" | "false" | "False" | "FALSE" | "yes"
                | "Yes" | "YES" | "no" | "No" | "NO" | "on" | "On" | "ON"
                | "off" | "Off" | "OFF" | "null" | "Null" | "NULL" | "~"
        ),
        SyntaxLanguage::Json => matches!(word, "true" | "false" | "null"),
        SyntaxLanguage::GoMod => matches!(
            word,
            "module" | "go" | "require" | "replace" | "exclude" | "retract"
        ),
        SyntaxLanguage::Bat => matches!(
            word,
            "echo" | "set" | "if" | "else" | "for" | "in" | "do" | "goto"
                | "call" | "exit" | "rem" | "pause" | "shift" | "choice"
                | "type" | "copy" | "move" | "del" | "ren" | "mkdir"
                | "rmdir" | "cd" | "pushd" | "popd" | "setlocal"
                | "endlocal" | "title" | "color" | "prompt" | "path"
                | "assoc" | "ftype" | "ver" | "vol" | "label"
                | "defined" | "exist" | "errorlevel" | "cmdextversion"
                | "not" | " EQU " | " NEQ " | " LSS " | " LEQ " | " GTR " | " GEQ "
        ),
        SyntaxLanguage::Ps1 => matches!(
            word,
            "if" | "else" | "elseif" | "foreach" | "for" | "while" | "do"
                | "switch" | "continue" | "break" | "return" | "throw"
                | "try" | "catch" | "finally" | "begin" | "process" | "end"
                | "param" | "function" | "filter" | "class" | "enum"
                | "in" | "not" | "and" | "or" | "xor" | "is" | "as"
                | "true" | "false" | "null" | "New-Object"
                | "Write-Host" | "Write-Output" | "Write-Error"
                | "Get-ChildItem" | "Get-Item" | "Get-Content"
                | "Set-Content" | "Add-Content" | "Remove-Item"
                | "New-Item" | "Copy-Item" | "Move-Item" | "Rename-Item"
                | "Test-Path" | "Join-Path" | "Split-Path"
                | "ForEach-Object" | "Where-Object" | "Select-Object"
                | "Sort-Object" | "Group-Object" | "Measure-Object"
                | "Import-Module" | "Export-Module" | "Import-Csv"
                | "Export-Csv" | "ConvertTo-Json" | "ConvertFrom-Json"
                | "Out-File" | "Out-Null" | "Start-Sleep"
                | "Start-Job" | "Receive-Job" | "Wait-Job"
                | "Invoke-Command" | "Invoke-Expression"
                | "Get-Process" | "Stop-Process" | "Get-Service"
                | "Get-Date" | "Get-Random" | "Get-Uptime"
                | "Get-Help" | "Get-Command" | "Get-Module"
                | "Get-Member" | "Format-Table" | "Format-List" | "Format-Wide"
        ),
        SyntaxLanguage::Html | SyntaxLanguage::Xml | SyntaxLanguage::Pro | SyntaxLanguage::Markdown | SyntaxLanguage::Txt | SyntaxLanguage::GoSum | SyntaxLanguage::Dotenv | SyntaxLanguage::Css => false,
    }
}

pub(super) fn is_type_name(lang: SyntaxLanguage, word: &str) -> bool {
    match lang {
        SyntaxLanguage::Rust => matches!(
            word,
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                | "u64" | "u128" | "usize" | "f32" | "f64" | "bool" | "char" | "str"
                | "String" | "Vec" | "HashMap" | "Option" | "Result" | "Box" | "Rc"
                | "Arc" | "Cell" | "RefCell" | "Mutex" | "Path" | "PathBuf" | "OsStr"
                | "OsString" | "CStr" | "CString" | "Duration" | "Ordering"
        ),
        SyntaxLanguage::Go => matches!(
            word,
            "bool" | "byte" | "complex64" | "complex128" | "error" | "float32"
                | "float64" | "int" | "int8" | "int16" | "int32" | "int64" | "rune"
                | "string" | "uint" | "uint8" | "uint16" | "uint32" | "uint64"
                | "uintptr"
        ),
        SyntaxLanguage::C => matches!(
            word,
            "size_t" | "ssize_t" | "int8_t" | "int16_t" | "int32_t" | "int64_t"
                | "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t" | "ptrdiff_t"
                | "wchar_t" | "FILE" | "va_list"
        ),
        SyntaxLanguage::Cpp => matches!(
            word,
            "size_t" | "nullptr_t" | "string" | "vector" | "map" | "set" | "list"
                | "shared_ptr" | "unique_ptr" | "weak_ptr" | "function" | "pair"
                | "optional" | "any" | "variant" | "string_view" | "span"
        ),
        SyntaxLanguage::Java => matches!(
            word,
            "byte" | "short" | "int" | "long" | "float" | "double" | "boolean"
                | "char" | "String" | "Object" | "Class" | "Throwable" | "Exception"
                | "Error" | "RuntimeException" | "Iterable" | "Comparable" | "Serializable"
                | "Runnable" | "Callable" | "List" | "Set" | "Map" | "Collection"
                | "ArrayList" | "HashMap" | "HashSet" | "Optional" | "Stream"
        ),
        SyntaxLanguage::Swift => matches!(
            word,
            "Bool" | "Int" | "Int8" | "Int16" | "Int32" | "Int64" | "UInt"
                | "UInt8" | "UInt16" | "UInt32" | "UInt64" | "Float" | "Double"
                | "String" | "Character" | "Array" | "Dictionary" | "Set" | "Optional"
                | "Error" | "Result" | "Substring"
        ),
        SyntaxLanguage::CSharp => matches!(
            word,
            "bool" | "byte" | "sbyte" | "char" | "decimal" | "double" | "float"
                | "int" | "uint" | "long" | "ulong" | "short" | "ushort" | "string"
                | "object" | "dynamic" | "nint" | "nuint"
        ),
        SyntaxLanguage::Julia => matches!(
            word,
            "Int8" | "Int16" | "Int32" | "Int64" | "Int128" | "UInt8" | "UInt16"
                | "UInt32" | "UInt64" | "UInt128" | "Float16" | "Float32" | "Float64"
                | "Bool" | "Char" | "String" | "Vector" | "Matrix" | "Array"
                | "Dict" | "Set" | "Tuple" | "Symbol" | "BigInt" | "BigFloat"
                | "Complex" | "Rational" | "Nothing" | "Missing" | "Any" | "Number"
                | "Integer" | "Real" | "AbstractFloat" | "Signed" | "Unsigned"
                | "AbstractString" | "AbstractArray" | "AbstractDict" | "AbstractSet"
                | "AbstractVector" | "AbstractMatrix" | "DataType" | "Function"
                | "Module" | "Expr" | "IO" | "Ptr"
        ),
        SyntaxLanguage::Erlang => matches!(
            word,
            "integer" | "float" | "boolean" | "atom" | "string" | "list" | "tuple"
                | "map" | "binary" | "function" | "pid" | "port" | "reference"
                | "any" | "dynamic"
        ),
        _ => {
            word.starts_with(|c: char| c.is_uppercase())
                && word.len() > 1
                && !word.bytes().all(|b| b.is_ascii_uppercase() || b == b'_')
        }
    }
}

pub(super) fn is_type_name_upper_heuristic(lang: SyntaxLanguage) -> bool {
    matches!(
        lang,
        SyntaxLanguage::Julia
            | SyntaxLanguage::Kotlin
            | SyntaxLanguage::Scala
            | SyntaxLanguage::Dart
            | SyntaxLanguage::Haskell
    )
}

pub(super) fn is_builtin(lang: SyntaxLanguage, word: &str) -> bool {
    match lang {
        SyntaxLanguage::Python => matches!(
            word,
            "print" | "len" | "range" | "int" | "str" | "float" | "list" | "dict"
                | "set" | "tuple" | "bool" | "type" | "super" | "isinstance"
                | "hasattr" | "getattr" | "setattr" | "open" | "input" | "map"
                | "filter" | "zip" | "enumerate" | "reversed" | "sorted" | "any"
                | "all" | "sum" | "min" | "max" | "abs" | "round" | "ord" | "chr"
                | "repr" | "format" | "iter" | "next" | "property" | "staticmethod"
                | "classmethod" | "object" | "Exception" | "BaseException" | "ValueError"
                | "TypeError" | "KeyError" | "IndexError" | "AttributeError" | "IOError"
                | "ImportError" | "StopIteration" | "RuntimeError" | "NotImplementedError"
                | "self" | "cls"
        ),
        SyntaxLanguage::JavaScript | SyntaxLanguage::TypeScript => matches!(
            word,
            "console" | "window" | "document" | "Math" | "JSON" | "Array" | "Object"
                | "String" | "Number" | "Boolean" | "Date" | "RegExp" | "Map" | "Set"
                | "Promise" | "Symbol" | "Error" | "undefined" | "NaN" | "Infinity"
                | "setTimeout" | "setInterval" | "clearTimeout" | "clearInterval"
                | "fetch" | "require" | "module" | "exports" | "process" | "Buffer"
                | "__dirname" | "__filename" | "global" | "setImmediate"
                | "isNaN" | "parseInt" | "parseFloat" | "encodeURI" | "decodeURI"
                | "encodeURIComponent" | "decodeURIComponent" | "eval"
                | "BigInt" | "Reflect" | "Proxy" | "Intl"
        ),
        SyntaxLanguage::Ruby => matches!(
            word,
            "puts" | "print" | "p" | "require" | "include" | "extend" | "attr_accessor"
                | "attr_reader" | "attr_writer" | "raise" | "fail" | "catch" | "throw"
                | "lambda" | "proc" | "block_given?" | "iterator?" | "loop" | "sleep"
                | "system" | "exec" | "spawn" | "fork" | "trap" | "Signal" | "trace"
                | "warn" | "abort" | "exit" | "at_exit" | "load" | "autoload"
                | "private" | "protected" | "public" | "module_function"
        ),
        SyntaxLanguage::Rust => matches!(
            word,
            "Some" | "None" | "Ok" | "Err" | "println" | "eprintln" | "format"
                | "vec" | "dbg" | "todo" | "unreachable" | "panic" | "assert"
                | "assert_eq" | "assert_ne" | "write" | "writeln" | "include_str"
                | "include_bytes" | "stringify" | "concat" | "env" | "option_env"
                | "cfg" | "column" | "file" | "line" | "macro_rules"
        ),
        SyntaxLanguage::Php => matches!(
            word,
            "echo" | "print" | "die" | "exit" | "include" | "require" | "isset"
                | "unset" | "empty" | "array" | "list" | "eval" | "var_dump"
                | "print_r" | "strlen" | "count" | "array_push" | "array_pop"
                | "json_encode" | "json_decode" | "file_get_contents" | "file_put_contents"
                | "header" | "session_start" | "session_destroy" | "mysql_connect"
        ),
        SyntaxLanguage::Julia => matches!(
            word,
            "println" | "print" | "show" | "display" | "write" | "read" | "open"
                | "close" | "push!" | "pop!" | "pushfirst!" | "popfirst!" | "append!"
                | "empty!" | "length" | "size" | "eltype" | "eachindex" | "keys"
                | "values" | "haskey" | "get" | "get!" | "setindex!" | "delete!"
                | "iterate" | "similar" | "copy" | "deepcopy" | "zero" | "one"
                | "zeros" | "ones" | "rand" | "randn" | "true" | "false" | "nothing"
                | "missing" | "Inf" | "Inf32" | "NaN" | "NaN32" | "π" | "pi" | "ℯ"
                | "im" | "undef" | "map" | "filter" | "reduce"
                | "sum" | "prod" | "minimum" | "maximum" | "findall" | "findfirst"
                | "findnext" | "findprev" | "sort" | "sort!" | "reverse" | "reverse!"
                | "unique" | "unique!" | "intersect" | "union" | "setdiff" | "symdiff"
                | "in" | "∈" | "∉" | "isempty" | "hasmethod" | "fieldnames" | "fieldtypes"
                | "propertynames" | "isnothing" | "ismissing" | "isdefined" | "isassigned"
                | "isequal" | "isless" | "isapprox" | "isfinite" | "isinf" | "isnan"
                | "typemin" | "typemax" | "eps" | "floatmax" | "floatmin" | "intmax"
                | "intmin" | "widemul" | "muladd" | "fma" | "precision" | "ndigits"
                | "digits" | "base" | "bin" | "oct" | "hex" | "string" | "repr"
                | "parse" | "tryparse" | "convert" | "promote" | "oftype"
                | "AbstractRange" | "StepRange" | "UnitRange" | "LinRange"
        ),
        _ => matches!(
            word,
            "true" | "false" | "null" | "nil" | "None" | "self" | "Self" | "this" | "super"
        ),
    }
}

pub(super) fn is_function_call(_word: &str, next_byte: Option<u8>) -> bool {
    next_byte == Some(b'(')
}