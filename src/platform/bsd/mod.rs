use bsd_kvm::{Access, KernProc};

pub fn get_all() {
    let mut kvm = bsd_kvm::Kvm::open::<&str>(None, None, Access::ReadOnly).unwrap();
    let procs = kvm.get_process(KernProc::All, 0);
    for p in procs {
        let comm = String::from_utf8(p.info.comm.iter().map(|c| *c as u8).collect()).unwrap();
        println!("{comm}");
    }
}
