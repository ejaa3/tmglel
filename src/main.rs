/*
 * SPDX-FileCopyrightText: 2024 Eduardo Javier Alvarado Aarón <eduardo.javier.alvarado.aaron@gmail.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{env, fmt::Write, fs::File, io::Read, io::Write as IoWrite};
use std::collections::{BTreeMap, BTreeSet};
use anyhow::{anyhow, Context};
use indoc::{indoc, writedoc};

type Lang = [&'static str; 7];

const LANGS: [Lang; 5] = [JS, RHAI, RUST, TOML, YAML];

fn main() -> anyhow::Result<()> {
	let mut buffer = String::new();
	let save_toml = "toml" == env::args().nth(1).as_deref().unwrap_or("");
	
	File::open("TMGLEL.toml").context("missing file `TMGLEL.toml`")?
		.read_to_string(&mut buffer).context("cannot read `TMGLEL.toml`")?;
	
	let map: BTreeMap<String, BTreeSet<String>> = toml::de::from_str(&buffer)
		.context("`TMGLEL.toml` is invalid TOML")?;
	
	let mut manifest = indoc!(/*TOML*/ "
		name = 'tmglel'
		displayName = 'TMGLEL'
		description = 'TextMate Grammars for Labeled Embedded Languages'
		license = 'Apache-2.0'
		version = '0.0.1'
		publisher = 'ejaa3'
		private = true
		engines = { vscode = '^1.92.0' }
		repository = 'github:ejaa3/tmglel'
		bugs = 'https://github.com/ejaa3/tmglel/issues'
	").to_string();
	
	let mut ats = [Vec::new(), Vec::new(), Vec::new()];
	
	for [id, name, scope, prelude, content, dry_1, dry_2] in LANGS {
		let Some(labels) = map.get(id) else { continue };
		buffer.clear();
		for ats in &mut ats { ats.clear() }
		
		writedoc!(manifest, "
			
			[[contributes.grammars]]
			path = './syntaxes/{id}.tmLanguage.json'
			scopeName = '{scope}.tmglel'
			injectTo = [
		")?;
		
		for [_, _, scope, ..] in LANGS { write!(manifest, "'{scope}',")? }
		manifest.push_str("]\n\n[contributes.grammars.embeddedLanguages]\n");
		
		writedoc!(buffer, /*TOML*/ "
			name = 'TMGLEL for {name}'
			scopeName = '{scope}.tmglel'
			injectionSelector = 'L:{scope}, L:meta.embedded.block.{id}'
				
			{prelude}")?;
		
		#[derive(Clone, Copy)]
		enum At { Dry1 = 1, Dry2, Id, Scope, List, Patterns }
		let mut search = [""; 3];
		
		for (i, content) in [(0, content), (1, dry_1), (2, dry_2)] {
			search[i] = content;
			
			while let Some((left, right)) = search[i].split_once("@_") {
				let (at, d) = if right.starts_with("ID") { (At::Id, 2) }
					else if right.starts_with("SCOPE") { (At::Scope, 5) }
					else if right.starts_with("LIST") { (At::List, 4) }
					else if right.starts_with("PATTERNS") { (At::Patterns, 8) }
					else if right.starts_with("DRY_1") { (At::Dry1, 5) }
					else if right.starts_with("DRY_2") { (At::Dry2, 5) } // unused
					else { Err(anyhow!("malformed content for {name}"))? };
				
				search[i] = right.split_at(d).1;
				ats[i].push((left, at));
			}
		}
		
		for [id, scope, list, patterns] in LABELS {
			if !labels.contains(id) { continue }
			
			writeln!(manifest, /*TOML*/ "'meta.embedded.block.{id}' = '{id}'")?;
			
			for (content, at) in ats[0].iter().cloned() {
				buffer.push_str(content);
				match at {
					At::Id => buffer.push_str(id),
					At::List => buffer.push_str(list),
					At::Scope => buffer.push_str(scope),
					At::Patterns => buffer.push_str(patterns),
					dry => {
						for (content, at) in ats[dry as usize].iter().cloned() {
							buffer.push_str(content);
							buffer.push_str(match at {
								At::Id => id,
								At::Scope => scope,
								At::List => list,
								At::Patterns => patterns,
								_ => Err(anyhow!("inner dry for {name}"))?,
							});
						}
						buffer.push_str(search[dry as usize])
					}
				}
			}
			buffer.push_str(search[0]);
		}
		
		std::fs::create_dir_all("syntaxes").context("cannot create directory `syntaxes`")?;
		
		if save_toml { File::options().write(true).truncate(true).create(true)
			.open(format!("syntaxes/{id}.tmLanguage.toml"))
			.with_context(|| format!("cannot open `syntaxes/{id}.tmLanguage.toml`"))?
			.write_all(buffer.as_bytes())
			.with_context(|| format!("cannot write `syntaxes/{id}.tmLanguage.toml`"))? }
		
		buffer = serde_json::to_string_pretty(
			&toml::de::from_str::<serde_json::Value>(&buffer)
				.with_context(|| format!("invalid TOML grammar for {id}"))?
		).with_context(|| format!("cannot serialize `syntaxes/{id}.tmLanguage.json`"))?;
		
		File::options().write(true).truncate(true).create(true)
			.open(format!("syntaxes/{id}.tmLanguage.json"))
			.with_context(|| format!("cannot open `syntaxes/{id}.tmLanguage.json`"))?
			.write_all(buffer.as_bytes())
			.with_context(|| format!("cannot save `syntaxes/{id}.tmLanguage.json`"))?
	}
	
	if save_toml { File::options().write(true).truncate(true).create(true)
		.open("package.toml").context("cannot open `package.toml`")?
		.write_all(manifest.as_bytes()).context("cannot write `package.toml`")? }
	
	buffer = serde_json::to_string_pretty(
		&toml::de::from_str::<serde_json::Value>(&manifest)
			.context("invalid TOML manifest")?
	).context("cannot serialize `package.json`")?;
	
	File::options().write(true).truncate(true).create(true)
		.open("package.json").context("cannot open `package.json`")?
		.write_all(buffer.as_bytes()).context("cannot write `package.json`")
}

const JS: Lang = ["javascript", "JavaScript", "source.js", indoc!(/*TOML*/ r"
	[[patterns]] #
	begin = '\${'
	beginCaptures.0.name = 'punctuation.definition.template-expression.begin.js.tmglel'
	end = '}'
	endCaptures.0.name = 'punctuation.definition.template-expression.end.js.tmglel'
	name = 'meta.template.expression.js.tmglel'
	contentName = 'meta.embedded.line.js.tmglel'
	patterns = [{ include = 'source.js' }]
	
"), indoc!(/*TOML*/ r#"
	[[patterns]] # regexp
	begin = '(//)\s*((?i:@_LIST))\W.*' # regexp
	end = "(?<='|\"|`)"
	beginCaptures.0.name = 'comment.line.double-slash.js.tmglel'
	beginCaptures.1.name = 'punctuation.definition.comment.js.tmglel'
	beginCaptures.2.name = 'fenced_code.block.language'
	
	[[patterns.patterns]] # regexp
	begin = "((')|(\"))|((`))"
	@_DRY_1
	[[patterns.patterns]]
	include = 'source.js'
	
	[[patterns]] # regexp
	begin = '''(?<=/\*\s*(?i:@_LIST)(?:\W.*?)?\*/\s*)(?:((')|("))|((`)))'''
	@_DRY_1
"#), indoc!(/*TOML*/ r#" # regexp
	end = '((\2)(?<!\\)(\3))((\5))'
	contentName = 'meta.embedded.block.@_ID'
	patterns = [{ include = '@_SCOPE' }]
	beginCaptures.1.name = 'punctuation.definition.string.begin.js.tmglel'
	beginCaptures.2.name = 'string.quoted.single.js.tmglel'
	beginCaptures.3.name = 'string.quoted.double.js.tmglel'
	beginCaptures.4.name = 'string.template.js.tmglel'
	beginCaptures.5.name = 'punctuation.definition.string.template.begin.js.tmglel'
	endCaptures.1.name = 'punctuation.definition.string.end.js.tmglel'
	endCaptures.2.name = 'string.quoted.single.js.tmglel'
	endCaptures.3.name = 'string.quoted.double.js.tmglel'
	endCaptures.4.name = 'string.template.js.tmglel'
	endCaptures.5.name = 'punctuation.definition.string.template.end.js.tmglel'
"#), ""];

const RHAI: Lang = ["rhai", "Rhai", "source.rhai", indoc!(/*TOML*/ r"
	[[patterns]]
	begin = '\${'
	beginCaptures.0.name = 'punctuation.section.interpolation.begin.rhai.tmglel'
	end = '}'
	endCaptures.0.name = 'punctuation.section.interpolation.end.rhai.tmglel'
	name = 'meta.interpolation.rhai.tmglel'
	patterns = [{ include = 'source.rhai'}]
	
"), indoc!(/*TOML*/ r#"
	[[patterns]] # regexp
	begin = '(//)\s*((?i:@_LIST))\W.*' # regexp
	end = '(?<=`|")'
	beginCaptures.0.name = 'comment.line.double-slash.rhai.tmglel'
	beginCaptures.1.name = 'punctuation.definition.comment.double-slash.rhai.tmglel'
	beginCaptures.2.name = 'fenced_code.block.language'
	
	[[patterns.patterns]] # regexp
	begin = '((`))|(("))'
	@_DRY_1
	[[patterns.patterns]]
	include = 'source.rhai'
	
	[[patterns]] # regexp
	begin = '(?<=/\*\s*(?i:@_LIST)(?:\W.*?)?\*/\s*)(?:((`))|((")))'
	@_DRY_1
"#), indoc!(/*TOML*/ r#" # regexp
	end = '((\2))(?<!\\)((\4))'
	contentName = 'meta.embedded.block.@_ID'
	patterns = [{ include = '@_SCOPE' }]
	beginCaptures.1.name = 'string.interpolated.rhai.tmglel'
	beginCaptures.2.name = 'punctuation.definition.string.begin.rhai.tmglel'
	beginCaptures.3.name = 'string.quoted.double.rhai.tmglel'
	beginCaptures.4.name = 'punctuation.definition.string.begin.rhai.tmglel'
	endCaptures.1.name = 'string.interpolated.rhai.tmglel'
	endCaptures.2.name = 'punctuation.definition.string.end.rhai.tmglel'
	endCaptures.3.name = 'string.quoted.double.rhai.tmglel'
	endCaptures.4.name = 'punctuation.definition.string.end.rhai.tmglel'
"#), ""];

const RUST: Lang = ["rust", "Rust", "source.rust", indoc!(/*TOML*/ r"
	[repository.interpolation] # regexp
	match = '({)(?=\S).*?(?<=\S)(})'
	name = 'meta.interpolation.rust.tmglel'
	captures.1.name = 'punctuation.definition.interpolation.rust.tmglel'
	captures.2.name = 'punctuation.definition.interpolation.rust.tmglel'
	
"), indoc!(/*TOML*/ r##"
	[[patterns]] # regexp
	begin = '(?<=//\s*(?i:@_LIST)\W.*)' # regexp
	end = '(?<!\\)(?<="#*)'
	
	[[patterns.patterns]] # regexp
	begin = '(b)?(r)?(#*)?(")'
	@_DRY_1
	[[patterns.patterns]]
	include = 'source.rust'
	
	[[patterns]] # regexp
	begin = '(?<=/\*\s*(?i:@_LIST)(?:\W.*?)?\*/\s*)(b)?(r)?(#*)?(")'
	@_DRY_1
"##), indoc!(/*TOML*/ r#" # regexp
	end = '(?<!\\)(")(\3)'
	contentName = 'meta.embedded.block.@_ID'
	patterns = [{ include = '#interpolation' }, { include = '@_SCOPE' }]
	
	beginCaptures.0.name = 'string.quoted.double.rust.tmglel'
	beginCaptures.1.name = 'string.quoted.byte.raw.rust.tmglel'
	beginCaptures.2.name = 'string.quoted.byte.raw.rust.tmglel'
	beginCaptures.3.name = 'punctuation.definition.string.raw.rust.tmglel'
	beginCaptures.4.name = 'punctuation.definition.string.rust.tmglel'
	
	endCaptures.0.name = 'string.quoted.double.rust.tmglel'
	endCaptures.1.name = 'punctuation.definition.string.rust.tmglel'
	endCaptures.2.name = 'punctuation.definition.string.raw.rust.tmglel'
"#), ""];

const TOML: Lang = ["toml", "TOML", "source.toml", "", indoc!(/*TOML*/ r#"
	[[patterns]] # regexp
	begin = '(?<=#\s*(?i:@_LIST)\W.*)' # regexp
	end = "(?<='''|\"\"\"|'|\")"
	
	[[patterns.patterns]] # regexp
	begin = "(?<!^)(?:(''')|(\"\"\")|(')|(\"))" # regexp
	end = '(\1)(\2)(\3)(?<!\\)(\4)'
	contentName = 'meta.embedded.block.@_ID'
	patterns = [{ include = '@_SCOPE' }]
	
	[patterns.patterns.captures]
	1.name = 'string.quoted.triple.literal.block.toml.tmglel'
	2.name = 'string.quoted.triple.basic.block.toml.tmglel'
	3.name = 'string.quoted.single.literal.line.toml.tmglel'
	4.name = 'string.quoted.single.basic.line.toml.tmglel'
	
	[[patterns.patterns]]
	include = 'source.toml'
"#), "", ""];

const YAML: Lang = ["yaml", "YAML", "source.yaml", "", indoc!(/*TOML*/ r#"
	[[patterns]] # regexp
	begin = '(?<=^(\s*)(?:-( ))?.*?:\s+)(\|)?(>)?(-)?\s*((#)\s*((?i:@_LIST))\W.*)' # regexp
	while = '^\1\2\2  '
	contentName = 'meta.embedded.block.@_ID'
	patterns = [{ include = '@_SCOPE' }]
	
	[patterns.beginCaptures]
	3.name = 'keyword.control.flow.block-scalar.literal.yaml.tmglel'
	4.name = 'keyword.control.flow.block-scalar.folded.yaml.tmglel'
	5.name = 'storage.modifier.chomping-indicator.yaml.tmglel'
	6.name = 'comment.line.number-sign.yaml.tmglel'
	7.name = 'punctuation.definition.comment.yaml.tmglel'
	8.name = 'fenced_code.block.language'
"#), "", ""]; /*
	# [[patterns]] # regexp
	# begin = '(?<!:\s+\|?>?-?\s*)(?<=#\s*(?i:@_LIST)\W.*)' # regexp
	# end = '''(?<=:.*(?:'|"))'''
	
	# [[patterns.patterns]] # regexp
	# begin = '''(?<=:\s+)(?:(')|("))''' # regexp
	# end = '(\1)(?<!\\)(\2)'
	# contentName = 'meta.embedded.block.@_ID'
	# patterns = [{ include = '@_SCOPE' }]
	
	# [patterns.patterns.beginCaptures]
	# 0.name = 'punctuation.definition.string.begin.yaml.tmglel'
	# 1.name = 'string.quoted.single.yaml.tmglel'
	# 2.name = 'string.quoted.double.yaml.tmglel'
	
	# [patterns.patterns.endCaptures]
	# 0.name = 'punctuation.definition.string.end.yaml.tmglel'
	# 1.name = 'string.quoted.single.yaml.tmglel'
	# 2.name = 'string.quoted.double.yaml.tmglel'
	
	# # [[patterns.patterns]] # regexp
	# # begin = '(?<=:) ' # regexp
	# # end = '\n|\s(?=\s*#)'
	# # name = 'meta.embedded.block.@_ID'
	# # patterns = [{ include = '@_SCOPE' }]
	
	# [[patterns.patterns]]
	# include = 'source.yaml'
*/

// WATCH https://github.com/microsoft/vscode-markdown-tm-grammar/blob/main/syntaxes/markdown.tmLanguage
// replace the following regular expression by: \t["$2", "$3", "$1", "$4"]\n
// \(\?i:\((.*?)\)(?:.*\n)*?(?=.*meta).*block\.(.*)<(?:.*\n)*?(?=.*include).*\n.*?>(.*)<(?:(?:.*\n){4}.*?string>(.*)<)?(?:.*\n)*?(?=.*\(\?i).*?(?=\(\?i)
// copy, paste and probably C++ and PHP should be fixed (compare to previous commits):

const LABELS: [[&str; 4]; 59] = [ // [id, scope, list, patterns]
	["css", "source.css", "css|css.erb", ""],
	["html", "text.html.basic", "html|htm|shtml|xhtml|inc|tmpl|tpl", ""],
	["ini", "source.ini", "ini|conf", ""],
	["java", "source.java", "java|bsh", ""],
	["lua", "source.lua", "lua", ""],
	["makefile", "source.makefile", "Makefile|makefile|GNUmakefile|OCamlMakefile", ""],
	["perl", "source.perl", "perl|pl|pm|pod|t|PL|psgi|vcl", ""],
	["r", "source.r", "R|r|s|S|Rprofile|\\{\\.r.+?\\}", ""],
	["ruby", "source.ruby", "ruby|rb|rbx|rjs|Rakefile|rake|cgi|fcgi|gemspec|irbrc|Capfile|ru|prawn|Cheffile|Gemfile|Guardfile|Hobofile|Vagrantfile|Appraisals|Rantfile|Berksfile|Berksfile.lock|Thorfile|Puppetfile", ""],
	["php", "source.php", "php|php3|php4|php5|phpt|phtml|aw|ctp", "\n  - include: text.html.basic"],
	["sql", "source.sql", "sql|ddl|dml", ""],
	["vs_net", "source.asp.vb.net", "vb", ""],
	["xml", "text.xml", "xml|xsd|tld|jsp|pt|cpt|dtml|rss|opml", ""],
	["xsl", "text.xml.xsl", "xsl|xslt", ""],
	["yaml", "source.yaml", "yaml|yml", ""],
	["dosbatch", "source.batchfile", "bat|batch", ""],
	["clojure", "source.clojure", "clj|cljs|clojure", ""],
	["coffee", "source.coffee", "coffee|Cakefile|coffee.erb", ""],
	["c", "source.c", "c|h", ""],
	["cpp", "source.cpp", "cpp|c\\+\\+|cxx", ""],
	["diff", "source.diff", "patch|diff|rej", ""],
	["dockerfile", "source.dockerfile", "dockerfile|Dockerfile", ""],
	["git_commit", "text.git-commit", "COMMIT_EDITMSG|MERGE_MSG", ""],
	["git_rebase", "text.git-rebase", "git-rebase-todo", ""],
	["go", "source.go", "go|golang", ""],
	["groovy", "source.groovy", "groovy|gvy", ""],
	["pug", "text.pug", "jade|pug", ""],
	["javascript", "source.js", "js|jsx|javascript|es6|mjs|cjs|dataviewjs|\\{\\.js.+?\\}", ""],
	["js_regexp", "source.js.regexp", "regexp", ""],
	["json", "source.json", "json|json5|sublime-settings|sublime-menu|sublime-keymap|sublime-mousemap|sublime-theme|sublime-build|sublime-project|sublime-completions", ""],
	["jsonc", "source.json.comments", "jsonc", ""],
	["less", "source.css.less", "less", ""],
	["objc", "source.objc", "objectivec|objective-c|mm|objc|obj-c|m|h", ""],
	["swift", "source.swift", "swift", ""],
	["scss", "source.css.scss", "scss", ""],
	["perl6", "source.perl.6", "perl6|p6|pl6|pm6|nqp", ""],
	["powershell", "source.powershell", "powershell|ps1|psm1|psd1|pwsh", ""],
	["python", "source.python", "python|py|py3|rpy|pyw|cpy|SConstruct|Sconstruct|sconstruct|SConscript|gyp|gypi|\\{\\.python.+?\\}", ""],
	["julia", "source.julia", "julia|\\{\\.julia.+?\\}", ""],
	["regexp_python", "source.regexp.python", "re", ""],
	["rust", "source.rust", "rust|rs|\\{\\.rust.+?\\}", ""],
	["scala", "source.scala", "scala|sbt", ""],
	["shellscript", "source.shell", "shell|sh|bash|zsh|bashrc|bash_profile|bash_login|profile|bash_logout|.textmate_init|\\{\\.bash.+?\\}", ""],
	["typescript", "source.ts", "typescript|ts", ""],
	["typescriptreact", "source.tsx", "tsx", ""],
	["csharp", "source.cs", "cs|csharp|c#", ""],
	["fsharp", "source.fsharp", "fs|fsharp|f#", ""],
	["dart", "source.dart", "dart", ""],
	["handlebars", "text.html.handlebars", "handlebars|hbs", ""],
	["markdown", "text.html.markdown", "markdown|md", ""],
	["log", "text.log", "log", ""],
	["erlang", "source.erlang", "erlang", ""],
	["elixir", "source.elixir", "elixir", ""],
	["latex", "text.tex.latex", "latex|tex", ""],
	["bibtex", "text.bibtex", "bibtex", ""],
	["twig", "source.twig", "twig", ""],
	
	["reaper", "source.txt", "reaper", ""], // vscode-reaper-theme
	["rhai", "source.rhai", "rhai", ""], // vscode-rhai
	["toml", "source.toml", "toml", ""], // even-better-toml
];
