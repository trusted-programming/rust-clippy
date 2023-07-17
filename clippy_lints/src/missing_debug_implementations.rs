use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir as hir;
use rustc_hir::def_id::LocalDefIdSet;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_tool_lint, impl_lint_pass};
use rustc_span::symbol::sym;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.73.0"]
    pub MISSING_DEBUG_IMPLEMENTATIONS,
    nursery,
    "default lint description"
}

#[derive(Default)]
pub struct MissingDebugImplementations {
    impling_types: Option<LocalDefIdSet>,
}

impl_lint_pass!(MissingDebugImplementations => [MISSING_DEBUG_IMPLEMENTATIONS]);

impl LateLintPass<'_> for MissingDebugImplementations {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &hir::Item<'_>) {
        if !cx.effective_visibilities.is_reachable(item.owner_id.def_id) {
            return;
        }

        match item.kind {
            hir::ItemKind::Struct(..) | hir::ItemKind::Union(..) | hir::ItemKind::Enum(..) => {},
            _ => return,
        }

        let Some(debug) = cx.tcx.get_diagnostic_item(sym::Debug) else {
            return;
        };

        if self.impling_types.is_none() {
            let mut impls = LocalDefIdSet::default();
            cx.tcx.for_each_impl(debug, |d| {
                if let Some(ty_def) = cx.tcx.type_of(d).subst_identity().ty_adt_def() {
                    if let Some(def_id) = ty_def.did().as_local() {
                        impls.insert(def_id);
                    }
                }
            });

            self.impling_types = Some(impls);
            dbg!("{:?}", &self.impling_types);
        }

        if !self.impling_types.as_ref().unwrap().contains(&item.owner_id.def_id) {
            span_lint_and_help(
                cx,
                MISSING_DEBUG_IMPLEMENTATIONS,
                item.span,
                "DEBUG",
                None,
                "consider using a more meaningful DEBUG",
            );
        }
    }
}
