pub mod http_helper;


pub struct StratumContext ();

pub trait Stratum {
    fn build(&self);
    fn inspect(&self);
    fn mount(&self);
    fn umount(&self);
    fn pull(&self);
}

impl Stratum for StratumContext {
    fn build(&self) {
        println!("build");
    }

    fn inspect(&self) {
        println!("inspect");
        http_helper::http_helper::read_manifest();
    }

    fn mount(&self) {
        println!("mount");
    }

    fn umount(&self) {
        println!("umount");
    }

    fn pull(&self) {
        println!("pull");
    }
}
