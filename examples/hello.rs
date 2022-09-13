use fused::fuse_loop_cfg_create;

fn main() {
    println!("{}", unsafe { fuse_loop_cfg_create() } as usize);
}
