#!/usr/bin/env python3
"""
Highlighter Theme Demo - shows Python syntax highlighting with different themes
"""

from icedpygui import Window, start_session, add_text_editor, add_column

# Sample Python code
PYTHON = '''def fibonacci(n):
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

# For documnetation purposes, the complete list of highlighter tokens are below
# but not used.  You would have to have the proper content in the editor to see the
# effects.  This text_editor is not a complete editing editor in python but by adding the
# appropriate menu to load files copy and paste, etc. it could be made.
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

    # SolarizedDark theme (dark background)
    col1 = add_column(fill=True)
    add_text_editor(
        parent_id=col1,
        content=PYTHON,
        height=200,
        fill=True,
        highlighter_theme="SolarizedDark",
        highlighter_token="py"
    )

    # Base16Mocha theme (dark background)
    col2 = add_column(fill=True)
    add_text_editor(
        parent_id=col2,
        content=PYTHON,
        height=200,
        fill=True,
        highlighter_theme="Base16Mocha",
        highlighter_token="py"
    )

    # InspiredGitHub theme (light background)
    col3 = add_column(fill=True)
    add_text_editor(
        parent_id=col3,
        content=PYTHON,
        height=200,
        fill=True,
        highlighter_theme="InspiredGitHub",
        highlighter_token="py"
    )

start_session()
