pub struct ExecutionContext ();

pub trait Stratum {
    fn build(&self);
    fn inspect(&self);
    fn mount(&self);
    fn umount(&self);
    fn pull(&self);
}

impl Stratum for ExecutionContext {
    fn build(&self) {
        println!("build");
    }

    fn inspect(&self) {
        println!("inspect");
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
