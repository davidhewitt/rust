//! This pass replaces the unreachable intrinsic with the Unreachable terminator,
//! so that other optimisation passes can eliminate uncreachable code.

use rustc::ty::{self, TyCtxt};
use rustc::mir::*;
use rustc::mir::visit::MutVisitor;
use rustc_target::spec::abi::Abi;
use syntax::symbol::sym;
use crate::transform::{MirPass, MirSource};

pub struct ReplaceUnreachableIntrinsic<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> ReplaceUnreachableIntrinsic<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        ReplaceUnreachableIntrinsic { tcx }
    }
}

impl<'tcx> MirPass<'tcx> for ReplaceUnreachableIntrinsic<'tcx> {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, _: MirSource<'tcx>, body: &mut Body<'tcx>) {
        replace_unreachable_intrinsic(tcx, body)
    }
}

pub fn replace_unreachable_intrinsic<'tcx>(tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
    ReplaceUnreachableIntrinsic::new(tcx).visit_body(body);
}

impl<'tcx> MutVisitor<'tcx> for ReplaceUnreachableIntrinsic<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn visit_terminator_kind(&mut self,
                        kind: &mut TerminatorKind<'tcx>,
                        _location: Location) {

        if let TerminatorKind::Call { func: Operand::Constant(c), .. } = kind {
            if let ty::FnDef(def_id, ..) = c.literal.ty.kind {
                if let Abi::RustIntrinsic = self.tcx.fn_sig(def_id).abi() {
                    if let sym::unreachable = self.tcx.item_name(def_id) {
                        *kind = TerminatorKind::Unreachable;
                    }
                }
            }
        }
    }
}
