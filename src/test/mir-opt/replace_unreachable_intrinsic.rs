#![feature(core_intrinsics)]

// Tests that unreachable intrinsic is replaced by unreachable terminator

fn main() {
    let x = Some(1);

    match x {
        Some(_) => {  },
        None => unsafe { std::intrinsics::unreachable() }
    }
}


// END RUST SOURCE
// START rustc.main.ReplaceUnreachableIntrinsic.before.mir
// ...
// bb3: {
//     StorageLive(_3);
//     const std::intrinsics::unreachable();
// }
// ...
// END rustc.main.ReplaceUnreachableIntrinsic.before.mir
// START rustc.main.ReplaceUnreachableIntrinsic.after.mir
// ...
// bb3: {
//     StorageLive(_3);
//     unreachable;
// }
// ...
// END rustc.main.ReplaceUnreachableIntrinsic.after.mir
