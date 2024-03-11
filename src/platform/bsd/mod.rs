use bsd_kvm::{Access, KernProc};

pub fn get_all() {
    let mut kvm = bsd_kvm::Kvm::open::<&str>(None, None, Access::ReadOnly).unwrap();
    let procs = kvm.get_process(KernProc::All, 0);
    for p in procs {
        let name = String::from_utf8(
            p.info
                .comm
                .iter()
                .filter(|c| **c > 0)
                .map(|c| *c as u8)
                .collect(),
        )
        .unwrap();
        println!("Name: {name:<25} PID: {:<10}", p.info.pid);
    }
}
