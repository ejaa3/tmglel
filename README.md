<!--
	SPDX-FileCopyrightText: 2024 Eduardo Javier Alvarado Aarón <eduardo.javier.alvarado.aaron@gmail.com>
	
	SPDX-License-Identifier: CC-BY-SA-4.0
-->

# TMGLEL

Stands for _TextMate Grammars for Labeled Embedded Languages_, a hacky way to have code chunks with a different syntax highlighting than the original language, like in Markdown fenced code blocks.

__Requirements__:
- Cargo (Rust).
- VS Code or maybe another code editor with support for TextMate grammars.
- For VS Code, a theme that does not color `string` directly, but a derivative like `string.quoted`, or alternatively disable semantic highlighting.

__Testing__:

1. Clone the repository and enter its directory.

2. Create a file called `TMGLEL.toml` with the following structure:
   ~~~ TOML
   '<language_id>' = ['<labels_id>', ...]
   ...: ...
   ~~~
   For example, to highlight CSS and JSON in Rust, and XML in TOML, it would be:
   ~~~ TOML
   rust = ['css', 'json'] # Use only the labels you
   toml = ['xml']         # need for best performance
   ~~~

3. Execute `cargo run` and check file `package.json` and directory `syntaxes/`.  
   Or `cargo run toml` to also generate corresponding TOML files.

4. For use in VS Code, see [packaging extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#packaging-extensions).

## Usage

For most languages ​​a comment starting with a case-insensitive label should highlight the next string in the same scope, not in the external or internal one:

~~~ JavaScript
 /* SQL */  "SELECT * FROM some_table" ; // is highlighted
 /* SQL */ ["SELECT * FROM some_table"]; // is not highlighted
[/* SQL */  "SELECT * FROM some_table"]; // is highlighted
~~~

### Supported languages

| Name: `ID` | Notice |
| - | - |
| JavaScript: `javascript` | <ul><li>Any `${...}` is highlighted as an interpolation (template expression) even if it is not, so that it is more likely to be highlighted in the labeled string.</li><li>The comment must be just before the string. [See below](#javascript).</li></ul> |
| Rhai: `rhai` | Same as the first point of JavaScript. |
| Rust: `rust` | Interpolations in labeled strings could be highlighted as Rust-like if there is no space between the brace and the content. For example: `{var}` is likely to be highlighted, but `{ var }` never is. |
| TOML: `toml` | Strings at the beginning of a line are unhighlightable. [See below](#toml). |
| YAML: `yaml` | It is not possible to highlight quoted strings or comment before the field. [See below](#yaml). |

#### JavaScript

~~~ js
// SQL (VS Code's JS grammar gets in the way)
let not_highlighted = "SELECT * FROM some_table";

let highlighted = // SQL
	"SELECT * FROM some_table";

let alternative = /* SQL */ "SELECT * FROM some_table";
~~~

#### TOML

~~~ TOML
# SQL
query = 'SELECT * FROM some_table'

# JavaScript
code = '''
console.log('Hello world!');
'''

# CSS (not within the scope of the array, so its string is ignored)
styles.light = ['body { background: white }']

[styles]
# the above commented label will highlight this:
dark = 'body { color: red }'

not_highlighted = [ # CSS       the string is at the
'body { background: white }'] # beginning of the line

highlighted = [ # CSS is highlighted by the indent
	'body { background: white }']
~~~

#### YAML

~~~ YAML
query: # SQL
  SELECT * FROM some_table

style: > # CSS (can be `|`)
  body { color: red }

script: # JavaScript
  "console.log('Hello world!');"
  # the above line is highlighted as a JS string
  # instead of highlighting what is in quotes as JS

# JavaScript (will not be highlighted even without quotes)
unsupported: "console.log('Hello world!');"
~~~

## Language labels

Should be the same as [markdown] code blocks, among others:

[markdown]: https://github.com/microsoft/vscode-markdown-tm-grammar/blob/main/syntaxes/markdown.tmLanguage

| ID | Labels |
| - | - |
| bibtex | bibtex |
| c | c, h |
| clojure | clj, cljs, clojure |
| coffee | coffee, Cakefile, coffee.erb |
| cpp | cpp, c++, cxx |
| csharp | cs, csharp, c# |
| css | css, css.erb |
| dart | dart |
| diff | patch, diff, rej |
| dockerfile | dockerfile, Dockerfile |
| dosbatch | bat, batch |
| elixir | elixir |
| erlang | erlang |
| fsharp | fs, fsharp, f# |
| git_commit | COMMIT_EDITMSG, MERGE_MSG |
| git_rebase | git-rebase-todo |
| go | go, golang |
| groovy | groovy, gvy |
| handlebars | handlebars, hbs |
| html | html, htm, shtml, xhtml, inc, tmpl, tpl |
| ini | ini, conf |
| java | java, bsh |
| javascript | js, jsx, javascript, es6, mjs, cjs, dataviewjs, {.js.*} |
| js_regexp | regexp |
| json | json, json5, sublime-settings, sublime-menu, sublime-keymap, sublime-mousemap, sublime-theme, sublime-build, sublime-project, sublime-completions |
| jsonc | jsonc |
| julia | julia, {.julia.*} |
| latex | latex, tex |
| less | less |
| log | log |
| lua | lua |
| makefile | Makefile, makefile, GNUmakefile, OCamlMakefile |
| markdown | markdown, md |
| objc | objectivec, objective-c, mm, objc, obj-c, m, h |
| perl | perl, pl, pm, pod, t, PL, psgi, vcl |
| perl6 | perl6, p6, pl6, pm6, nqp |
| php | php, php3, php4, php5, phpt, phtml, aw, ctp |
| powershell | powershell, ps1, psm1, psd1, pwsh |
| pug | jade, pug |
| python | python, py, py3, rpy, pyw, cpy, SConstruct, Sconstruct, sconstruct, SConscript, gyp, gypi, {.python.*} |
| r | R, r, s, S, Rprofile, {.r.*} |
| reaper | reaper |
| regexp_python | re |
| rhai | rhai |
| ruby | ruby, rb, rbx, rjs, Rakefile, rake, cgi, fcgi, gemspec, irbrc, Capfile, ru, prawn, Cheffile, Gemfile, Guardfile, Hobofile, Vagrantfile, Appraisals, Rantfile, Berksfile, Berksfile.lock, Thorfile, Puppetfile |
| rust | rust, rs, {.rust.*} |
| scala | scala, sbt |
| scss | scss |
| shellscript | shell, sh, bash, zsh, bashrc, bash_profile, bash_login, profile, bash_logout, .textmate_init, {.bash.*} |
| sql | sql, ddl, dml |
| swift | swift |
| toml | toml |
| twig | twig |
| typescript | typescript, ts |
| typescriptreact | tsx |
| vs_net | vb |
| xml | xml, xsd, tld, jsp, pt, cpt, dtml, rss, opml |
| xsl | xsl, xslt |
| yaml | yaml, yml |
