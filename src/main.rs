mod get_infos;
use crate::get_infos::*;
use dirs::{self, config_dir};
pub use libwmctl::prelude::WmCtl;
use std::fs::File;
use std::io::{prelude::*, BufReader};
fn main() -> std::io::Result<()> {
    std::process::Command::new("clear")
        .spawn()
        .expect("What the fuck?");
    let distroiter = vec!["arch"];
    let mut formatvec1 = vec![]; //The ASCII
    let mut formatvec2 = vec![]; //The Systeminformation

    let moditer = vec![
        "model",
        "cpu",
        "gpu",
        "disk",
        "battery",
        "user",
        "memory",
        "resolution",
        "battery",
        "users",
        "distro",
        "kernel",
        "de",
        "wm",
        "shell",
        "term",
        "font",
        "packages",
        "uptime",
        "cp_usage",
        "public_ip",
        "locale",
    ];

    let mut homepat = config_dir();
    let mut homepath = String::new();
    match homepat.as_mut() {
        Some(v) => homepath.push_str(v.to_str().unwrap()),
        None => {}
    }

    homepath.push_str("/rufetch/rufetch.conf");

    let f = File::open(homepath)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let mut outputline = String::from(line?);
        if outputline.starts_with('-') | outputline.starts_with('>') {
            outputline = outputline.replace('-', "\u{2502}  \u{25bb} ");
            outputline = outputline.replace(">", "");
            for replacement in &moditer {
                match replacement.to_owned() {
                    "kernel" => {
                        outputline = outputline.replace("kernel", kernel_ident().as_str());
                    }
                    "cpu" => {
                        outputline = outputline.replace("cpu", cpucores().as_str());
                    }
                    "uptime" => {
                        outputline = outputline.replace("uptime", lifespan().as_str());
                    }
                    "resolution" => {
                        outputline = outputline.replace("resolution", get_screen_res().as_str());
                    }
                    "wm" => {
                        outputline = outputline.replace("wm", windowmanager().as_str());
                    }
                    "distro" => {
                        outputline = outputline.replace("distro", disto().as_str());
                    }
                    "model" => outputline = outputline.replace("model", dev().as_str()),
                    "gpu" => outputline = outputline.replace("gpu", getgpu().as_str()),
                    "term" => outputline = outputline.replace("term", getterminal().as_str()),
                    "memory" => outputline = outputline.replace("memory", meminfo().as_str()),
                    "de" => outputline = outputline.replace("de", desktopenvironment().as_str()),
                    "packages" => outputline = outputline.replace("packages", pkgmgr().as_str()),
                    "public_ip" => outputline = outputline.replace("public_ip", getip().as_str()),
                    "locale" => outputline = outputline.replace("locale", getlocale().as_str()),
                    "users" => outputline = outputline.replace("users", getusr().as_str()),
                    "font" => outputline = outputline.replace("font", getfont().as_str()),
                    //"disk" => {outputline = outputline.replace("disk", getdisk().as_str())}
                    "battery" => outputline = outputline.replace("battery", getbat().as_str()),
                    _ => (),
                }
            }
            formatvec2.push(outputline);
        }
    }
    let mut logopat = config_dir();
    let mut logopath = String::new();
    match logopat.as_mut() {
        Some(ae) => logopath.push_str(ae.to_str().unwrap()),
        None => {}
    }

    logopath.push_str("/rufetch/distros/");
    for distro in distroiter {
        logopath.push_str(distro.to_lowercase().as_str());
        logopath.push_str(".txt");
        let fle = File::open(&logopath).unwrap();
        let dreader = BufReader::new(fle);
        for dline in dreader.lines() {
            let mut asciiline = String::from(dline?);
            asciiline = asciiline.replace("|", "\\e[2C");
            formatvec1.push(asciiline);
        }
    }
    let mut coun1 = 0;
    let mut coun2 = 0;
    let cap = 25;
    while coun1 <= cap {
        formatvec1.push(" ".to_string());
        coun1 += 1;
    }
    while coun2 <= cap {
        formatvec2.push(" ".to_string());
        coun2 += 1;
    }
    for i in 0..cap {
        let mut final_result = String::new(); //Both combined
        final_result.push_str(formatvec1[i].as_str());

        final_result.push_str(formatvec2[i].as_str());
        final_result.push_str("\n");
        std::process::Command::new("printf")
            .arg(final_result)
            .spawn()
            .expect("Printing Failed!");
    }

    return Ok(());
}
