//! When *constructing* `hir`, we start at some parent syntax node and recursively
//! lower the children.
//!
//! This module allows one to go in the opposite direction: start with a syntax
//! node for a *child*, and get its hir.

use either::Either;
use hir_expand::{attrs::collect_attrs, HirFileId};

use crate::{
    db::DefDatabase,
    dyn_map::{
        keys::{self, Key},
        DynMap,
    },
    item_scope::ItemScope,
    item_tree::ItemTreeModItemNode,
    nameres::DefMap,
    src::{HasChildSource, HasSource},
    AdtId, AssocItemId, AssocItemLoc, DefWithBodyId, EnumId, FieldId, ImplId, ItemLoc, Lookup,
    MacroId, ModuleDefId, ModuleId, TraitId, VariantId,
};

pub trait ChildBySource {
    fn child_by_source(&self, db: &dyn DefDatabase, file_id: HirFileId) -> DynMap {
        let mut res = DynMap::default();
        self.child_by_source_to(db, &mut res, file_id);
        res
    }
    fn child_by_source_to(&self, db: &dyn DefDatabase, map: &mut DynMap, file_id: HirFileId);
}

impl ChildBySource for TraitId {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId) {
        let data = db.trait_data(*self);

        data.attribute_calls().filter(|(ast_id, _)| ast_id.file_id == file_id).for_each(
            |(ast_id, call_id)| {
                res[keys::ATTR_MACRO_CALL].insert(ast_id.to_node(db.upcast()), call_id);
            },
        );
        data.items.iter().for_each(|&(_, item)| {
            add_assoc_item(db, res, file_id, item);
        });
    }
}

impl ChildBySource for ImplId {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId) {
        let data = db.impl_data(*self);
        data.attribute_calls().filter(|(ast_id, _)| ast_id.file_id == file_id).for_each(
            |(ast_id, call_id)| {
                res[keys::ATTR_MACRO_CALL].insert(ast_id.to_node(db.upcast()), call_id);
            },
        );
        data.items.iter().for_each(|&item| {
            add_assoc_item(db, res, file_id, item);
        });
    }
}

fn add_assoc_item(db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId, item: AssocItemId) {
    match item {
        AssocItemId::FunctionId(func) => {
            insert_assoc_item_loc(db, res, file_id, func, keys::FUNCTION)
        }
        AssocItemId::ConstId(konst) => insert_assoc_item_loc(db, res, file_id, konst, keys::CONST),
        AssocItemId::TypeAliasId(ty) => {
            insert_assoc_item_loc(db, res, file_id, ty, keys::TYPE_ALIAS)
        }
    }
}

impl ChildBySource for ModuleId {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId) {
        let def_map = self.def_map(db);
        let module_data = &def_map[self.local_id];
        module_data.scope.child_by_source_to(db, res, file_id);
    }
}

impl ChildBySource for ItemScope {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId) {
        self.declarations().for_each(|item| add_module_def(db, res, file_id, item));
        self.impls().for_each(|imp| insert_item_loc(db, res, file_id, imp, keys::IMPL));
        self.extern_crate_decls()
            .for_each(|ext| insert_item_loc(db, res, file_id, ext, keys::EXTERN_CRATE));
        self.use_decls().for_each(|ext| insert_item_loc(db, res, file_id, ext, keys::USE));
        self.unnamed_consts(db)
            .for_each(|konst| insert_assoc_item_loc(db, res, file_id, konst, keys::CONST));
        self.attr_macro_invocs().filter(|(id, _)| id.file_id == file_id).for_each(
            |(ast_id, call_id)| {
                res[keys::ATTR_MACRO_CALL].insert(ast_id.to_node(db.upcast()), call_id);
            },
        );
        self.legacy_macros().for_each(|(_, ids)| {
            ids.iter().for_each(|&id| {
                if let MacroId::MacroRulesId(id) = id {
                    let loc = id.lookup(db);
                    if loc.id.file_id() == file_id {
                        res[keys::MACRO_RULES].insert(loc.source(db).value, id);
                    }
                }
            })
        });
        self.derive_macro_invocs().filter(|(id, _)| id.file_id == file_id).for_each(
            |(ast_id, calls)| {
                let adt = ast_id.to_node(db.upcast());
                calls.for_each(|(attr_id, call_id, calls)| {
                    if let Some((_, Either::Left(attr))) =
                        collect_attrs(&adt).nth(attr_id.ast_index())
                    {
                        res[keys::DERIVE_MACRO_CALL].insert(attr, (attr_id, call_id, calls.into()));
                    }
                });
            },
        );

        fn add_module_def(
            db: &dyn DefDatabase,
            map: &mut DynMap,
            file_id: HirFileId,
            item: ModuleDefId,
        ) {
            macro_rules! insert {
                ($map:ident[$key:path].$insert:ident($id:ident)) => {{
                    let loc = $id.lookup(db);
                    if loc.id.file_id() == file_id {
                        $map[$key].$insert(loc.source(db).value, $id)
                    }
                }};
            }
            match item {
                ModuleDefId::FunctionId(id) => {
                    insert_assoc_item_loc(db, map, file_id, id, keys::FUNCTION)
                }
                ModuleDefId::ConstId(id) => {
                    insert_assoc_item_loc(db, map, file_id, id, keys::CONST)
                }
                ModuleDefId::TypeAliasId(id) => {
                    insert_assoc_item_loc(db, map, file_id, id, keys::TYPE_ALIAS)
                }
                ModuleDefId::StaticId(id) => {
                    insert_assoc_item_loc(db, map, file_id, id, keys::STATIC)
                }
                ModuleDefId::TraitId(id) => insert_item_loc(db, map, file_id, id, keys::TRAIT),
                ModuleDefId::TraitAliasId(id) => {
                    insert_item_loc(db, map, file_id, id, keys::TRAIT_ALIAS)
                }
                ModuleDefId::AdtId(adt) => match adt {
                    AdtId::StructId(id) => insert_item_loc(db, map, file_id, id, keys::STRUCT),
                    AdtId::UnionId(id) => insert_item_loc(db, map, file_id, id, keys::UNION),
                    AdtId::EnumId(id) => insert_item_loc(db, map, file_id, id, keys::ENUM),
                },
                ModuleDefId::MacroId(id) => match id {
                    MacroId::Macro2Id(id) => insert!(map[keys::MACRO2].insert(id)),
                    MacroId::MacroRulesId(id) => insert!(map[keys::MACRO_RULES].insert(id)),
                    MacroId::ProcMacroId(id) => insert!(map[keys::PROC_MACRO].insert(id)),
                },
                ModuleDefId::ModuleId(_)
                | ModuleDefId::EnumVariantId(_)
                | ModuleDefId::BuiltinType(_) => (),
            }
        }
    }
}

impl ChildBySource for VariantId {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, _: HirFileId) {
        let arena_map = self.child_source(db);
        let arena_map = arena_map.as_ref();
        let parent = *self;
        for (local_id, source) in arena_map.value.iter() {
            let id = FieldId { parent, local_id };
            match source.clone() {
                Either::Left(source) => res[keys::TUPLE_FIELD].insert(source, id),
                Either::Right(source) => res[keys::RECORD_FIELD].insert(source, id),
            }
        }
    }
}

impl ChildBySource for EnumId {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId) {
        let loc = &self.lookup(db);
        if file_id != loc.id.file_id() {
            return;
        }

        let tree = loc.id.item_tree(db);
        let ast_id_map = db.ast_id_map(loc.id.file_id());
        let root = db.parse_or_expand(loc.id.file_id());

        db.enum_data(*self).variants.iter().for_each(|&(variant, _)| {
            res[keys::ENUM_VARIANT].insert(
                ast_id_map.get(tree[variant.lookup(db).id.value].ast_id).to_node(&root),
                variant,
            );
        });
    }
}

impl ChildBySource for DefWithBodyId {
    fn child_by_source_to(&self, db: &dyn DefDatabase, res: &mut DynMap, file_id: HirFileId) {
        let body = db.body(*self);
        if let &DefWithBodyId::VariantId(v) = self {
            VariantId::EnumVariantId(v).child_by_source_to(db, res, file_id)
        }

        for (_, def_map) in body.blocks(db) {
            // All block expressions are merged into the same map, because they logically all add
            // inner items to the containing `DefWithBodyId`.
            def_map[DefMap::ROOT].scope.child_by_source_to(db, res, file_id);
        }
    }
}

fn insert_item_loc<ID, N>(
    db: &dyn DefDatabase,
    res: &mut DynMap,
    file_id: HirFileId,
    id: ID,
    key: Key<N::Source, ID>,
) where
    ID: for<'db> Lookup<Database<'db> = dyn DefDatabase + 'db, Data = ItemLoc<N>> + 'static,
    N: ItemTreeModItemNode,
    N::Source: 'static,
{
    let loc = id.lookup(db);
    if loc.id.file_id() == file_id {
        res[key].insert(loc.source(db).value, id)
    }
}

fn insert_assoc_item_loc<ID, N>(
    db: &dyn DefDatabase,
    res: &mut DynMap,
    file_id: HirFileId,
    id: ID,
    key: Key<N::Source, ID>,
) where
    ID: for<'db> Lookup<Database<'db> = dyn DefDatabase + 'db, Data = AssocItemLoc<N>> + 'static,
    N: ItemTreeModItemNode,
    N::Source: 'static,
{
    let loc = id.lookup(db);
    if loc.id.file_id() == file_id {
        res[key].insert(loc.source(db).value, id)
    }
}
