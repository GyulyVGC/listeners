use bsd_kvm::{Access, KernProc};

pub fn get_all() {
    let mut kvm = bsd_kvm::Kvm::open(None, None, Access::ReadOnly).unwrap();
    let procs = kvm.get_process(KernProc::All, 0);
    for proc in procs {
        println!("{:?}", proc);
    }
}
