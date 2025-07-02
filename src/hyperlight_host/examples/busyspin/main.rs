use anyhow::anyhow;
use hyperlight_host::func::call_ctx::MultiUseGuestCallContext;
use hyperlight_host::sandbox::{Callable, MultiUseSandbox, UninitializedSandbox};
use hyperlight_host::sandbox_state::sandbox::EvolvableSandbox;
use hyperlight_host::sandbox_state::transition::Noop;
use hyperlight_host::{GuestBinary, Result};
use hyperlight_testing::rust_guest_as_pathbuf;

fn main() {
    let buf = rust_guest_as_pathbuf("busyspinguest");
    let spin_path = buf
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("couldn't convert simple guest PathBuf to string"))
        .unwrap();

    let sbox1: MultiUseSandbox = {
        let path = spin_path;
        let u_sbox = UninitializedSandbox::new(GuestBinary::FilePath(path), None).unwrap();
        u_sbox.evolve(Noop::default())
    }
    .unwrap();
    let ctx1 = sbox1.new_call_context();
    do_calls(ctx1).unwrap();
}

fn do_calls(mut ctx: MultiUseGuestCallContext) -> Result<MultiUseSandbox> {
    let res: i32 = ctx.call("BusySpin", (1_u32, 1_u32))?;
    println!("got BusySpin res: {res}");
    ctx.finish()
}
