use std::collections::HashSet;

// use bsd_kvm::{Access, KernProc, Kvm};
use crate::Listener;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    Err("This OS isn't supported yet".into())
    // let mut kvm = Kvm::open::<&str>(None, None, Access::ReadOnly).unwrap();
    // let procs = kvm.get_process(KernProc::All, 0);
    // for p in procs {
    //     let name = String::from_utf8(
    //         p.info
    //             .comm
    //             .iter()
    //             .filter(|c| **c > 0)
    //             .map(|c| *c as u8)
    //             .collect(),
    //     )
    //     .unwrap();
    //     println!("Name: {name:<25} PID: {:<10}", p.info.pid);
    // }
    //
    // println!();
    //
    // let ctl_list = sysctl::CtlIter::root();
    // for c in ctl_list {
    //     println!("{:?}", c.unwrap().name());
    // }
    //
    // println!();
    //
    // let ctl = sysctl::Ctl::new("net.inet.tcp.pcbcount").unwrap();
    // println!("Value: {:?}", ctl.value());
    //
    // let ctl = sysctl::Ctl::new("net.inet.tcp.pcblist_n").unwrap(); // each is 524 B long (?)
    // let val = ctl.value().unwrap();
    // let val = val.as_struct();
    // println!("Value: {:?}", val);
}
