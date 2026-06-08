use std::path::Path;

use anyhow::Result;

use super::Editor;
use crate::session::EditorSession;

// ── template generation ──────────────────────────────────────────────────────

/// Return a starter template for a given file extension.
pub fn generate_template(ext: &str) -> &'static str {
    match ext {
        "c" => {
            "\
#include <stdio.h>

int main(void) {
    printf(\"Hello, World!\\n\");
    return 0;
}
"
        }
        "cpp" | "cc" | "cxx" => {
            "\
#include <iostream>

int main() {
    std::cout << \"Hello, World!\" << std::endl;
    return 0;
}
"
        }
        "cs" | "csx" => {
            "\
using System;

class Program {
    static void Main() {
        Console.WriteLine(\"Hello, World!\");
    }
}
"
        }
        "go" => {
            "\
package main

import \"fmt\"

func main() {
    fmt.Println(\"Hello, World!\")
}
"
        }
        "java" => {
            "\
public class Main {
    public static void main(String[] args) {
        System.out.println(\"Hello, World!\");
    }
}
"
        }
        "kt" | "kts" => {
            "\
fun main() {
    println(\"Hello, World!\")
}
"
        }
        "swift" => {
            "\
import Foundation

print(\"Hello, World!\")
"
        }
        "dart" => {
            "\
void main() {
    print('Hello, World!');
}
"
        }
        "rs" => {
            "\
fn main() {
    println!(\"Hello, World!\");
}
"
        }
        "py" => {
            "\
def main():
    print(\"Hello, World!\")

if __name__ == \"__main__\":
    main()
"
        }
        "js" => {
            "\
function main() {
    console.log(\"Hello, World!\");
}

main();
"
        }
        "ts" => {
            "\
function main(): void {
    console.log(\"Hello, World!\");
}

main();
"
        }
        "rb" => {
            "\
def main
    puts \"Hello, World!\"
end

main
"
        }
        "sh" | "bash" => {
            "\
#!/usr/bin/env bash

main() {
    echo \"Hello, World!\"
}

main \"$@\"
"
        }
        "lua" => {
            "\
function main()
    print(\"Hello, World!\")
end

main()
"
        }
        "pl" => {
            "\
#!/usr/bin/perl
use strict;
use warnings;

print \"Hello, World!\\n\";
"
        }
        "php" => {
            "<?php

function main() {
    echo \"Hello, World!\\n\";
}

main();
"
        }
        "r" => {
            "\
main <- function() {
    cat(\"Hello, World!\\n\")
}

main()
"
        }
        "hs" => {
            "\
module Main where

main :: IO ()
main = putStrLn \"Hello, World!\"
"
        }
        "ex" | "exs" => {
            "\
defmodule Hello do
  def main do
    IO.puts(\"Hello, World!\")
  end
end

Hello.main()
"
        }
        "zig" => {
            "\
const std = @import(\"std\");

pub fn main() !void {
    std.debug.print(\"Hello, World!\\n\", .{});
}
"
        }
        "nim" => {
            "\
echo \"Hello, World!\"
"
        }
        "scala" => {
            "\
object Main {
    def main(args: Array[String]): Unit = {
        println(\"Hello, World!\")
    }
}
"
        }
        "clj" | "cljs" => {
            "\
(defn -main []
  (println \"Hello, World!\"))

(-main)
"
        }
        "ml" | "mli" => {
            "\
print_endline \"Hello, World!\"
"
        }
        "math" => {
            "\
# ntc.math — math evaluator

# Examples:
PI * 2
sqrt(144)
sin(PI / 2)

# Built-ins: sin, cos, tan, sqrt, pow, abs, floor, ceil, round, ln, log, rand, sum, min, max, avg
"
        }
        _ => "",
    }
}

/// Create a starter file at `path` based on its extension.
/// Returns `true` if a template was written (file did not exist).
pub fn init_file(path: &Path) -> Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let template = generate_template(ext);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, template)?;
    Ok(true)
}

// ── handle __exit__ sentinel ─────────────────────────────────────────────────

pub fn edit_file(path: &Path) -> Result<bool> {
    edit_file_with_session(path, None).map(|(r, _)| r)
}

pub fn edit_file_with_session(path: &Path, restored: Option<EditorSession>) -> Result<(bool, Option<EditorSession>)> {
    let mut editor = Editor::new(path)?;
    if let Some(ref session) = restored {
        // Only restore cursor position if file path matches
        if session.current_file == path {
            editor.restore_from_session(session);
        }
    }
    let result = match editor.run() {
        Ok(v) => v,
        Err(e) if e.to_string() == "__exit__" => false,
        Err(e) => return Err(e),
    };
    let captured = if result {
        None
    } else {
        Some(editor.capture_session())
    };
    Ok((result, captured))
}
