#!/usr/bin/env python3
"""
Highlighter Theme Demo - shows Python syntax highlighting with different themes
"""

from icedpygui import Window, start_session, Container, add_text_editor

# Sample Python code
PYTHON = '''
# ***Scroll to see all ***
def fibonacci(n):
    """Calculate fibonacci sequence."""
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

class DataProcessor:
    def __init__(self, data):
        self.data = data

    def process(self):
        result = [x * 2 for x in self.data]
        return result

if __name__ == "__main__":
    processor = DataProcessor([1, 2, 3, 4, 5])
    print(processor.process())
'''

# Sample Rust code
RUST = '''
//***Scroll to see all ***
///Calculate fibonacci sequence.
fn fibonacci(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

fn main() {
    let count = 10;
    let series: Vec<u64> = (0..count).map(fibonacci).collect();
    println!("First {count} Fibonacci numbers: {series:?}");
}
'''

# sample c++ code
C_PLUS_PLUS = '''
#***Scroll to see all ***
#include <iostream>
#include <vector>

std::vector<unsigned long long> fibonacci(int count) {
    std::vector<unsigned long long> series;
    unsigned long long a = 0, b = 1;
    for (int i = 0; i < count; ++i) {
        series.push_back(a);
        unsigned long long next = a + b;
        a = b;
        b = next;
    }
    return series;
}

int main() {
    int count = 10;
    std::cout << "First " << count << " Fibonacci numbers:";
    for (auto n : fibonacci(count)) {
        std::cout << " " << n;
    }
    std::cout << std::endl;
    return 0;
}
'''

# For documnetation purposes, the complete list of highlighter tokens are below.
# You would have to have the proper content in the editor to see the
# effects. This text_editor demo is not a complete code editor but by adding the
# appropriate menu to load files copy and paste, and of course a code hinting method.
# But if you just want to view a file, it would surfice.
TOKENS = [
    "actionscript", "ada", "apache", "applescript", "asciidoc", "asp", "asm",
        "arm", "x86_64", "awk",
    "bash", "batch", "bibtex",
    "c", "c#", "c++", "cabal", "cfml", "clojure", "cmake", "coffeescript", "crontab",
        "crystal", "css", "csv",
    "d", "dart", "dockerfile", "dotenv", "diff",
    "elixir", "elm", "email", "erlang",
    "f#", "fish", "fortran",
    "git", "glsl", "go", "graphql", "graphviz", "groff", "groovy",
    "haskell", "html",
    "ini",
    "java", "javadoc", "jsp", "javascript", "javascript_babel", "jinja2", "jq", "json", "julia",
    "kotlin",
    "latex", "latex_log", "lean", "less", "lisp", "literate_haskell", "livescript", "llvm", "lua",
    "makefile", "man", "markdown", "matlab", "mediawiki", "multimarkdown",
    "nant", "nginx", "nim", "ninja", "nix", "nsis",
    "objective-c", "objective-c++", "ocaml", "ocamllex", "ocamlyacc", "org",
    "pascal", "perl", "php", "powershell", "protobuf", "puppet", "purescript", "python",
    "qml",
    "r", "racket", "rd", "rego", "regex", "requirements", "rst", "robot", "ruby", "ruby_haml",
        "ruby_on_rails", "ruby_slim", "rust",
    "sass", "scala", "scss", "salt", "sml", "solidity", "sql", "strace", "stylus", "svelte",
        "swift", "systemverilog",
    "tcl", "terraform", "tex", "textile", "todo", "toml", "typescript", "typescript_react",
    "varlink", "verilog", "viml", "vue", "vyper",
    "wgsl",
    "xml",
    "yaml",
    "zig"
]

def on_select(_wid: int, selected: str):
    """Combobox selection"""
    print(selected)


with Window(title="Syntax Highlighter Themes", center=True):

    # Python token
    with Container(fill=True):
        add_text_editor(
            content=PYTHON,
            height=200,
            fill=True,
            highlighter_token="py"
        )

    # Rust token
    with Container(fill=True):
        add_text_editor(
            content=RUST,
            height=200,
            fill=True,
            highlighter_token="r"
        )

    # C++ token
    with Container(fill=True):
        add_text_editor(
            content=C_PLUS_PLUS,
            height=200,
            fill=True,
            highlighter_token="c++"
        )

start_session()
