(function() {var type_impls = {
"hir_def":[],
"hir_expand":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-InFileWrapper%3CHirFileId,+N%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hir_expand/files.rs.html#353-384\">source</a><a href=\"#impl-InFileWrapper%3CHirFileId,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;N: AstNode&gt; <a class=\"type\" href=\"hir_expand/files/type.InFile.html\" title=\"type hir_expand::files::InFile\">InFile</a>&lt;N&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.original_ast_node_rooted\" class=\"method\"><a class=\"src rightside\" href=\"src/hir_expand/files.rs.html#354-383\">source</a><h4 class=\"code-header\">pub fn <a href=\"hir_expand/files/type.InFile.html#tymethod.original_ast_node_rooted\" class=\"fn\">original_ast_node_rooted</a>(\n    self,\n    db: &amp;dyn <a class=\"trait\" href=\"hir_expand/db/trait.ExpandDatabase.html\" title=\"trait hir_expand::db::ExpandDatabase\">ExpandDatabase</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.76.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"type\" href=\"hir_expand/files/type.InRealFile.html\" title=\"type hir_expand::files::InRealFile\">InRealFile</a>&lt;N&gt;&gt;</h4></section></div></details>",0,"hir_expand::ast_id_map::AstId","hir_expand::ast_id_map::ErasedAstId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CInFileWrapper%3CMacroFileId,+T%3E%3E-for-InFileWrapper%3CHirFileId,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hir_expand/files.rs.html#52-56\">source</a><a href=\"#impl-From%3CInFileWrapper%3CMacroFileId,+T%3E%3E-for-InFileWrapper%3CHirFileId,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"hir_expand/files/struct.InFileWrapper.html\" title=\"struct hir_expand::files::InFileWrapper\">InFileWrapper</a>&lt;<a class=\"struct\" href=\"hir_expand/struct.MacroFileId.html\" title=\"struct hir_expand::MacroFileId\">MacroFileId</a>, T&gt;&gt; for <a class=\"type\" href=\"hir_expand/files/type.InFile.html\" title=\"type hir_expand::files::InFile\">InFile</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hir_expand/files.rs.html#53-55\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(_: <a class=\"type\" href=\"hir_expand/files/type.InMacroFile.html\" title=\"type hir_expand::files::InMacroFile\">InMacroFile</a>&lt;T&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<InFileWrapper<MacroFileId, T>>","hir_expand::ast_id_map::AstId","hir_expand::ast_id_map::ErasedAstId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CInFileWrapper%3CFileId,+T%3E%3E-for-InFileWrapper%3CHirFileId,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hir_expand/files.rs.html#58-62\">source</a><a href=\"#impl-From%3CInFileWrapper%3CFileId,+T%3E%3E-for-InFileWrapper%3CHirFileId,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"hir_expand/files/struct.InFileWrapper.html\" title=\"struct hir_expand::files::InFileWrapper\">InFileWrapper</a>&lt;FileId, T&gt;&gt; for <a class=\"type\" href=\"hir_expand/files/type.InFile.html\" title=\"type hir_expand::files::InFile\">InFile</a>&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hir_expand/files.rs.html#59-61\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(_: <a class=\"type\" href=\"hir_expand/files/type.InRealFile.html\" title=\"type hir_expand::files::InRealFile\">InRealFile</a>&lt;T&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<InFileWrapper<FileId, T>>","hir_expand::ast_id_map::AstId","hir_expand::ast_id_map::ErasedAstId"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()