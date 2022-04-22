pub use libwmctl::prelude::WmCtl;
use std::{process::Command, fs};
use systemstat::Duration;
use libmacchina::{GeneralReadout, MemoryReadout};
//

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
pub fn get_shell() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let mut shell = general_readout.shell(libmacchina::traits::ShellFormat::Relative, libmacchina::traits::ShellKind::Current).unwrap();
    shell.pop();
    return shell;
}
pub fn get_screen_res() -> String { 
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let mut resolution = String::new();
    resolution.push(' ');
    resolution.push_str(general_readout.resolution().unwrap().replace(",", " x").as_str());
    return resolution;
}

pub fn cpucores() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let mut modl = general_readout.cpu_model_name().unwrap().to_string();
    modl.push_str(" (");
    modl.push_str(general_readout.cpu_usage().unwrap().to_string().as_str());
    modl.push_str("%% Usage)");
    return modl;
}
pub fn lifespan() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let mut gg = general_readout.uptime().unwrap() as f64;
    gg = gg / 60.0 / 60.0;
    let upt: Vec<char> = gg.to_string().chars().collect();
    let mut ut = String::new();
    
    if upt[1] == '.' {
        ut.push(upt[0]);
        ut.push_str(" Hours and ");
        ut.push(upt[2]);
        ut.push(upt[3]);
        ut.push_str(" Minutes");
    } else if upt[2] == '.' {
        ut.push(upt[0]);
        ut.push(upt[1]);
        ut.push_str(" Hours and ");
        ut.push(upt[3]);
        ut.push(upt[4]);
        ut.push_str(" Minutes");
    }else {
        ut.push(upt[0]);
        ut.push(upt[1]);
        ut.push_str(" Hours and ");
        ut.push(upt[3]);
        ut.push(upt[4]);
        ut.push_str(" Minutes");

    }
    return ut;
}
pub fn get_current_distro() -> String {
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
    use libmacchina::traits::MemoryReadout as _;
    let memread = MemoryReadout::new();
    let mused = (memread.used().unwrap() / 1024) as f64;
    let mtot = (memread.total().unwrap() / 1024) as f64;
    let perc = (mused as f64/ mtot as f64) * 100.0;
    let mut mems = String::from(mused.to_string());
    mems.push_str(" MiB | ");
    mems.push_str(mtot.to_string().as_str());
    mems.push_str(" MiB (");
    mems.push_str(perc.round().to_string().as_str());
    mems.push_str("%% Used)");

    return mems;
}
pub fn desktopenvironment() -> String {
    return nixinfo::environment().unwrap();
}
pub fn pkgmgr() -> String {
    return nixinfo::packages("pacman").unwrap();
}
pub fn getip() -> String {
    let cat = Command::new("curl")
    .arg("ifconfig.me")
    .output()
    .expect("NoPe (=");
let mut ooc = String::new();
for ins in cat.stdout {
    ooc.push(ins as char)
}
ooc = String::from(ooc.trim());
return ooc;
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
pub fn getdisk() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let used = general_readout.disk_space().unwrap().0 / 1024 / 1024 / 1024;
    let avail = general_readout.disk_space().unwrap().1 / 1024 / 1024 / 1024;
    let perc = ((used as f64/ avail as f64) * 100.0).round() as i64;
    let mut space = String::new();
    space.push_str(used.to_string().as_str());
    space.push_str(" GB  | ");
    space.push_str(avail.to_string().as_str());
    space.push_str(" GB (");
    space.push_str(perc.to_string().as_str());
    //space.push_str("%");
    space.push_str("%%)");
    //space.push_str(general_readout.disk_space().unwrap().0.to_string().as_str());
    //space.push_str(general_readout.disk_space().unwrap().1.to_string().as_str());
    return space;
}
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
pub fn create_missing_dir(path: String) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
} 
pub fn create_config_file(mut path: String) -> std::io::Result<()> {
    //let mut defpath = String::from(path);
    path.push_str("/rufetch.conf");
    fs::write(path, 
">┌─────────Hardware Information───────────
-  model
-  cpu
-﬙  gpu
-  disk
-塞 memory
- resolution
#  battery
>├─────────Software Information───────────
-  users
-  distro
# Just get your distro's logo of nerdfonts.com
-  kernel
-  de
-  shell
-  term
-  font
-  packages
-  uptime
-  cp_usage
-  public_ip
-🕐 locale  
# This only works on glibc systems.
>└─────────────────────────────────────────")?;
    Ok(())
}
pub fn create_gen_ascii(mut path: String) -> std::io::Result<()> {
    //let mut defpath = String::from(path);
    path.push_str("/distros/default.ascii");
    fs::write(path,
         "                 R RR RR                  |
              R RRRRRRRR R          R     |
 R RR       R RRRRRRRRRRRRR R      RR     |
rR RRR    R RRRRRRRRRRRRRRRRR R   RRR R   |
RRR RR   RRRRRRRRRRRRRRRRRRRRRRR  RRRRR   |
 RRRRR  RRRRRRRRRRRRRRRRRRRRRRRR  RRRR    |
  RRR RRRRRRRRRRRRRRRRRRRRRRRRRRRR RR     |
    R  RRRRRRRRRR  RR  RRRRRRRRRRR        |
     RRRRRRRRRRRR   RR  RRRRRRRRRR        |
      RRRRRRRRRRR   RR   RRRRRRRRRR       |
     RR==RRRRRRRRRRRRRRRRRRRRRR===RR      |
     RR =  ==RRRRRRR  RRRRRR==  = RR      |
      RR =     ===========     = RR       |
       RR                        R        |
        R                       R         |
         R                                |
                                          |
                                          |
                                          |
                                          |
                                          |
                                          |
                                          |
                                          |")?;
    Ok(())
}
pub fn does_this_exist(filepath: &String) -> bool{
    let x: bool = std::path::Path::new(filepath).exists();
    return x;
}