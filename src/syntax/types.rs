use crossterm::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Normal,
    Keyword,
    String,
    Comment,
    Number,
    Type,
    Builtin,
    Function,
    Operator,
    Punctuation,
    Attribute,
    Macro,
    Regex,
    Tag,
    Constant,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    C,
    Cpp,
    CSharp,
    Java,
    Kotlin,
    Swift,
    Dart,
    Ruby,
    Bash,
    /// Vim script — `.vimrc`, `.nvimrc`, `init.vim`, …
    Vim,
    Lua,
    Php,
    Perl,
    Haskell,
    Elixir,
    Zig,
    Nim,
    Scala,
    Clojure,
    OCaml,
    R,
    Julia,
    Glsl,
    Erlang,
    Xml,
    Yaml,
    Pro,
    Html,
    Json,
    Markdown,
    Txt,
    GoMod,
    GoSum,
    Dotenv,
    Css,
    Bat,
    Ps1,
    /// .sql — SQL (generic, MySQL/PostgreSQL/SQLite flavoured keywords)
    Sql,
    /// .fs / .fsi / .fsx / .fsscript — F#
    FSharp,
    /// .asm / .s / .S / .nasm — assembly (x86/GAS/NASM flavoured)
    Asm,
    /// .graphql / .gql — GraphQL schema and query language
    Graphql,
    /// .ntc — ntc automation script file
    NtcScr,
    /// .ntc.ral — ntc run aliases export file (TOML format)
    NtcRal,
    /// .ntc.igcare — ntc ignore/care settings export file (TOML format)
    NtcIgcare,
    /// .ntc.math — ntc math expression file
    NtcMath,
    /// .ntc_theme — ntc theme file (TOML format)
    NtcTheme,
    /// .iss — Inno Setup script
    InnoSetup,
    /// .lock — package lock file (TOML-like format)
    LockFile,
    /// .mq4 / .mq5 / .mqh — MetaQuotes Language (MQL4/MQL5)
    Mql,
}

pub fn color_for(tt: TokenType) -> Color {
    let theme = crate::utils::theme::ThemeManager::current();
    match tt {
        TokenType::Keyword => theme.syntax.keyword.to_crossterm(),
        TokenType::String => theme.syntax.string.to_crossterm(),
        TokenType::Comment => theme.syntax.comment.to_crossterm(),
        TokenType::Number => theme.syntax.number.to_crossterm(),
        TokenType::Type => theme.syntax.r#type.to_crossterm(),
        TokenType::Builtin => theme.syntax.builtin.to_crossterm(),
        TokenType::Function => theme.syntax.function.to_crossterm(),
        TokenType::Operator => theme.syntax.operator.to_crossterm(),
        TokenType::Punctuation => theme.syntax.punctuation.to_crossterm(),
        TokenType::Attribute => theme.syntax.attribute.to_crossterm(),
        TokenType::Macro => theme.syntax.macro_token.to_crossterm(),
        TokenType::Regex => theme.syntax.regex.to_crossterm(),
        TokenType::Tag => theme.syntax.tag.to_crossterm(),
        TokenType::Constant => theme.syntax.constant.to_crossterm(),
        TokenType::Normal => theme.syntax.normal.to_crossterm(),
    }
}

/// Dotfile names that are highlighted even though they carry no extension.
/// Matching is on the lowercased file name.
fn detect_language_for_filename(name: &str) -> Option<SyntaxLanguage> {
    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        // Shell startup files and other name-only scripts.
        ".bashrc" | ".bash_profile" | ".bash_login" | ".bash_logout" | ".bash_aliases"
        | ".bash_functions" | ".zshrc" | ".zprofile" | ".zshenv" | ".zlogin" | ".zlogout"
        | ".profile" | ".kshrc" | ".mkshrc" | ".shrc" | "pkgbuild" | ".pam_environment" => {
            Some(SyntaxLanguage::Bash)
        }
        // Vim / Neovim config files.
        ".vimrc" | ".nvimrc" | ".gvimrc" | ".exrc" | "_vimrc" | "_gvimrc" | "vimrc" | "gvimrc"
        | "init.vim" => Some(SyntaxLanguage::Vim),
        // CMake / Make / Ninja / Bazel / Meson / Just / Docker / systemd / Cargo metadata.
        "cmakelists.txt" | "makefile" | "gnumakefile" | "bsdmakefile" | "ninja" | "meson.build"
        | "meson_options.txt" | "build" | "workspace" | "workspace.bazel" | "module.bazel"
        | "build.bazel" | "dockerfile" | "containerfile" | "justfile" | "procfile"
        | "vagrantfile" | "gemfile" | "rakefile" | "berksfile" | "brewfile" | "podfile"
        | "fastfile" | "appfile" | "matchfile" | "guardfile" | "thorfile" | "capfile"
        | "puppetfile" | "cargo.toml" | "pyproject.toml" | "setup.py" | "setup.cfg" | "tox.ini"
        | "pytest.ini" | "mypy.ini" | "flake8" => Some(SyntaxLanguage::Dotenv),
        // Systemd unit files.
        ".service" | ".socket" | ".timer" | ".target" | ".mount" | ".path" => {
            Some(SyntaxLanguage::Dotenv)
        }
        // env / ignore / rc style dotfiles (key=value, # comments).
        ".env" | ".gitignore" | ".dockerignore" | ".npmignore" | ".eslintignore"
        | ".prettierignore" | ".gcloudignore" | ".hgignore" | ".editorconfig"
        | ".gitattributes" | ".gitmodules" | ".gitconfig" | ".npmrc" | ".yarnrc" | ".curlrc"
        | ".wgetrc" | ".netrc" | ".inputrc" | ".flake8" | ".pylintrc" | ".coveragerc"
        | ".rustfmt.toml" => Some(SyntaxLanguage::Dotenv),
        _ => {
            // `.env.local`, `.env.production`, … carry no Rust extension
            // either, so match the family by prefix.
            if lower.starts_with(".env.") || lower.starts_with(".env-") {
                Some(SyntaxLanguage::Dotenv)
            } else {
                None
            }
        }
    }
}

/// Detect the syntax language for a path: known dotfile names first
/// (`.bashrc`, `.vimrc`, `.env`, …), then the extension.
pub fn detect_language_for_path(path: &std::path::Path) -> Option<SyntaxLanguage> {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if let Some(lang) = detect_language_for_filename(name) {
            return Some(lang);
        }
    }
    detect_language(path.extension().and_then(|e| e.to_str())?)
}

pub fn detect_language(ext: &str) -> Option<SyntaxLanguage> {
    match ext.to_lowercase().as_str() {
        "rs" => Some(SyntaxLanguage::Rust),

        "py" | "pyw" | "pyx" | "pyi" | "ipynb" => Some(SyntaxLanguage::Python),

        "js" | "mjs" | "cjs" | "jsx" => Some(SyntaxLanguage::JavaScript),
        "ts" | "tsx" | "mts" | "cts" => Some(SyntaxLanguage::TypeScript),

        "go" => Some(SyntaxLanguage::Go),

        "c" | "h" => Some(SyntaxLanguage::C),
        "cpp" | "cc" | "cxx" | "c++" | "hpp" | "hh" | "hxx" | "ino" | "tpp" | "cu" | "cuh"
        | "ipp" | "obj" => Some(SyntaxLanguage::Cpp),

        "cs" | "csx" | "cshtml" | "razor" => Some(SyntaxLanguage::CSharp),

        "java" | "jsp" | "jspx" | "tag" => Some(SyntaxLanguage::Java),

        "kt" | "kts" | "ktm" => Some(SyntaxLanguage::Kotlin),

        "swift" => Some(SyntaxLanguage::Swift),
        "dart" => Some(SyntaxLanguage::Dart),

        "rb" | "rbw" | "gemspec" => Some(SyntaxLanguage::Ruby),

        "sh" | "bash" | "zsh" | "ksh" | "fish" | "csh" | "tcsh" => Some(SyntaxLanguage::Bash),

        "lua" => Some(SyntaxLanguage::Lua),

        "php" | "phtml" | "php3" | "php4" | "php5" | "php7" | "phps" => Some(SyntaxLanguage::Php),

        "pl" | "pm" | "t" => Some(SyntaxLanguage::Perl),

        "hs" | "lhs" => Some(SyntaxLanguage::Haskell),
        "ex" | "exs" => Some(SyntaxLanguage::Elixir),
        "zig" => Some(SyntaxLanguage::Zig),
        "nim" => Some(SyntaxLanguage::Nim),
        "scala" | "sc" | "sbt" => Some(SyntaxLanguage::Scala),
        "clj" | "cljs" | "cljc" | "edn" => Some(SyntaxLanguage::Clojure),
        "ml" | "mli" | "mll" | "mly" => Some(SyntaxLanguage::OCaml),
        "r" | "rmd" | "rnw" => Some(SyntaxLanguage::R),
        "jl" => Some(SyntaxLanguage::Julia),
        "erl" | "hrl" => Some(SyntaxLanguage::Erlang),

        "xml" | "xsd" | "xsl" | "xslt" | "svg" | "plist" | "rss" | "atom" | "xaml" | "gpx"
        | "kml" | "wsdl" | "dtd" | "xul" | "csproj" | "vbproj" | "fsproj" | "props" | "targets" => {
            Some(SyntaxLanguage::Xml)
        }

        "yml" | "yaml" => Some(SyntaxLanguage::Yaml),
        "pro" => Some(SyntaxLanguage::Pro),

        "html" | "htm" | "xhtml" | "svelte" | "hbs" | "handlebars" | "mustache" | "xht" | "vue"
        | "astro" | "ejs" | "pug" | "jade" | "liquid" | "twig" | "jinja" | "jinja2" | "j2" => {
            Some(SyntaxLanguage::Html)
        }

        "json" | "jsonc" | "json5" | "jsonl" | "ndjson" | "geojson" | "har" => {
            Some(SyntaxLanguage::Json)
        }

        "md" | "markdown" | "mdown" | "mkd" | "mdwn" | "mdx" | "rst" | "adoc" | "asciidoc"
        | "org" | "textile" | "creole" => Some(SyntaxLanguage::Markdown),

        "txt" | "text" | "log" | "gradle" => Some(SyntaxLanguage::Txt),

        "mod" => Some(SyntaxLanguage::GoMod),
        "sum" => Some(SyntaxLanguage::GoSum),

        "glsl" | "vert" | "vs" | "vsh" | "frag" | "fsh" | "geom" | "gs" | "gsh" | "tesc"
        | "tese" | "comp" | "rgen" | "rint" | "rahit" | "rchit" | "rmiss" | "rcall" | "mesh"
        | "task" | "hlsl" | "fx" | "spvasm" | "wgsl" | "metal" => Some(SyntaxLanguage::Glsl),

        "sql" | "psql" | "mysql" | "pgsql" | "plsql" | "tsql" => Some(SyntaxLanguage::Sql),

        "fs" | "fsi" | "fsx" | "fsscript" => Some(SyntaxLanguage::FSharp),

        "asm" | "s" | "nasm" | "masm" | "gas" | "inc" => Some(SyntaxLanguage::Asm),

        "graphql" | "gql" | "graphqls" => Some(SyntaxLanguage::Graphql),

        "css" | "scss" | "less" | "sass" | "qss" => Some(SyntaxLanguage::Css),

        "bat" | "cmd" => Some(SyntaxLanguage::Bat),
        "ps1" | "psm1" | "psd1" | "pssc" => Some(SyntaxLanguage::Ps1),

        "env" | "gitignore" | "dockerignore" | "hgignore" | "editorconfig" | "toml" | "ini"
        | "cfg" | "conf" | "properties" | "hcl" | "tf" | "tfvars" | "nomad" | "nix" | "ron"
        | "hjson" | "yml.tmpl" | "service" | "socket" | "timer" | "target" | "mount" | "path" => {
            Some(SyntaxLanguage::Dotenv)
        }

        "ntc" => Some(SyntaxLanguage::NtcScr),
        "ral" => Some(SyntaxLanguage::NtcRal),
        "igcare" => Some(SyntaxLanguage::NtcIgcare),
        "math" => Some(SyntaxLanguage::NtcMath),
        "ntc_theme" => Some(SyntaxLanguage::NtcTheme),

        "iss" => Some(SyntaxLanguage::InnoSetup),
        "lock" => Some(SyntaxLanguage::LockFile),

        "mq4" | "mq5" | "mqh" => Some(SyntaxLanguage::Mql),

        _ => None,
    }
}
