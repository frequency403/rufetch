pub use libwmctl::prelude::WmCtl;
use std::process::Command;
use systemstat::Duration;
pub fn kernel_ident() -> String {
    let cat = Command::new("cat")
        .arg("/proc/sys/kernel/osrelease")
        .output()
        .expect("NoPe (=");
    let mut ooc = String::new();
    for ins in cat.stdout {
        ooc.push(ins as char)
    }
    ooc = String::from(ooc.trim());
    return ooc;
}
pub fn get_screen_res() -> String {
    let wmctl = WmCtl::connect().unwrap();
    let mut screenres = String::new();
    screenres.push_str(" ");
    screenres.push_str(wmctl.width().to_string().as_str());
    screenres.push_str(" x ");
    screenres.push_str(wmctl.height().to_string().as_str());

    return screenres;
}

pub fn cpucores() -> String {
    let ce = rin_sys::get_cpu_info();
    return ce.model_name;
}
pub fn lifespan() -> String {
    let upt = nixinfo::uptime();
    return upt.unwrap();
}
pub fn windowmanager() -> String {
    let wmctl = WmCtl::connect().unwrap();
    let (_, wm_name) = wmctl.winmgr().unwrap();
    let wwm = wm_name;
    return wwm;
}
pub fn disto() -> String {
    let mut ret = String::new();
    let dis = nixinfo::distro();
    ret.push_str(dis.unwrap().replace("\"", "").as_str());
    return ret;
}
pub fn dev() -> String {
    let modl = nixinfo::device();
    return modl.unwrap();
}
pub fn getgpu() -> String {
    let ggpu = nixinfo::gpu().unwrap();
    return ggpu;
}
pub fn getterminal() -> String {
    return nixinfo::terminal().unwrap();
}
pub fn meminfo() -> String {
    return nixinfo::memory().unwrap();
}
pub fn desktopenvironment() -> String {
    return nixinfo::environment().unwrap();
}
pub fn pkgmgr() -> String {
    return nixinfo::packages("pacman").unwrap();
}
pub fn getip() -> String {
    return local_ip::get().unwrap().to_string();
}
pub fn getlocale() -> String {
    let cat = Command::new("cat")
        .arg("/etc/timezone")
        .output()
        .expect("NoPe (=");
    let mut ooc = String::new();
    for ins in cat.stdout {
        ooc.push(ins as char)
    }
    ooc = String::from(ooc.trim());
    return ooc;
}
/*pub fn getdisk() -> String {
   let sys = systemstat::System::new();
   let mut res = String::new();
   let ava: u64 = 0;
   let tot: u64 = 0;

   match sys.mount_at("/") {
            Ok(mounts) => {res.push_str("/root: ");
                                     ava = mounts.avail.as_u64(); tot = mounts.total.as_u64();
                                     ava = ava.try_into().unwrap() * 0.000000001; tot = tot.try_into().unwrap() * 0.000000001 ;
                                     res.push_str(ava.to_string().as_str());
                                     res.push_str("GB used | ");
                                     res.push_str(tot.to_string().as_str());
                                     res.push_str("GB total")}
            Err(x) => {res.push_str("Error detecting Filesystems")}
   }
   return res;
}*/
pub fn getbat() -> String {
    let mut there = String::new();
    let cap: f32 = 0.00;
    let tim = Duration::new(0, 0);
    let time;
    let _h = systemstat::data::BatteryLife {
        remaining_capacity: cap,
        remaining_time: tim,
    };
    time = tim.as_secs() / 60;
    there.push_str(cap.to_string().as_str());
    there.push_str("% | ");
    there.push_str(time.to_string().as_str());
    there.push_str(" min remaining");

    return there;
}
pub fn getusr() -> String {
    let cas = Command::new("whoami").output().expect("NoPe (=");
    let mut oo_c = String::new();
    for inc in cas.stdout {
        oo_c.push(inc as char)
    }
    oo_c = String::from(oo_c.trim());
    let cat = Command::new("cat")
        .arg("/etc/hostname")
        .output()
        .expect("NoPe (=");
    let mut ooc = String::new();
    for ins in cat.stdout {
        ooc.push(ins as char)
    }
    ooc = String::from(ooc.trim());
    let mut res = String::new();
    res.push_str(oo_c.as_str());
    res.push(' ');
    res.push('@');
    res.push(' ');
    res.push_str(ooc.as_str());
    return res;
}
pub fn getfont() -> String {
    let cat = Command::new("fc-match").output().expect("NoPe (=");
    let mut ooc = String::new();
    for ins in cat.stdout {
        ooc.push(ins as char)
    }

    ooc = String::from(ooc.split(':').take(1).collect::<Vec<_>>()[0].to_string());
    ooc = ooc.replace(".ttf", "");
    return ooc;
}
