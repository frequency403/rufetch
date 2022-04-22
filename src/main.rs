mod get_infos;
use crate::get_infos::*;
use dirs::{self, config_dir};
pub use libwmctl::prelude::WmCtl;
use std::fs::{File};
use std::io::{prelude::*, BufReader};
use std::path::Path;
fn main() -> std::io::Result<()> {
// ##################################################################################### Clear screen for better printing
    std::process::Command::new("clear")
        //.arg("\\e2J\\e[H")
        .spawn()
        .expect("What the fuck?"); // Seriously, why in hell should the command fail?

// ##################################################################################### Distroiter is for all supported Distros with logo. More to be added.

    let distroiter = vec!["arch"];
    let mut formatvec1 = vec![]; //The ASCII
    let mut formatvec2 = vec![]; //The Systeminformation
// ##################################################################################### Declare Moduleiterator for replacing later
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
        "public_ip",
        "locale",
    ];
// #################################################################################### Make Configdir a String
    let mut homepat = config_dir();
    let mut config_directory = String::new();
    let mut homepath = String::new();
    match homepat.as_mut() {
        Some(v) => {homepath.push_str(v.to_str().unwrap()); config_directory.push_str(v.to_str().unwrap())},
        None => {}
    }
    config_directory.push_str("/rufetch");
    homepath.push_str("/rufetch/rufetch.conf");
// #################################################################################### Dir Checker & Create if not exist
    let mut asciidir = String::from(&config_directory);
    asciidir.push_str("/distros");
    let cfgfdir2 = config_directory.clone();
    let cfgfdir1 = config_directory.clone();
    let cfgdirex = Path::new(&config_directory).is_dir();
    if cfgdirex == false {
        let f = create_missing_dir(config_directory);
        let g = create_missing_dir(asciidir);
        let h = create_config_file(cfgfdir1);
        let j = create_gen_ascii(cfgfdir2);
        // ############################################################################ Errorhandling
        match f {
            Ok(file) => file,
            Err(error) => panic!("{}", error),
        }
        match g {
            Ok(fil) => fil,
            Err(er) => panic!("{}", er),
        }
        match h {
            Ok(fi) => fi,
            Err(el) => panic!("{}", el),
        }
        match j {
            Ok(fe) => fe,
            Err(ecs) => panic!("{}", ecs),
        }
    }
// #################################################################################### Open configfile
    let f = File::open(homepath)?;
    let reader = BufReader::new(f);
// #################################################################################### Iterate over file line by line, replacing keywords with information.
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
                    "distro" => {
                        outputline = outputline.replace("distro", get_current_distro().as_str());
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
                    "disk" => outputline = outputline.replace("disk", getdisk().as_str()),
                    "battery" => outputline = outputline.replace("battery", getbat().as_str()),
                    "shell" => outputline = outputline.replace("shell", get_shell().as_str()),
                    _ => (),
                }
            }
            formatvec2.push(outputline);
            //formatvec2.push('\n'.to_string());
        }
    }
// ################################################################################################# Logopath to file as String
    let mut logopat = config_dir();
    let mut logopath = String::new();
    match logopat.as_mut() {
        Some(ae) => logopath.push_str(ae.to_str().unwrap()),
        None => {}
    }
// ################################################################################################# Check for the current Distro and choose appropriate Logo
            logopath.push_str("/rufetch/distros/");
            let currdist = get_current_distro().to_lowercase();
            let dchk = currdist.as_str().split_whitespace().collect::<Vec<&str>>();
            let mut logodircheck = logopath.clone();
            logodircheck.push_str(dchk[0]);
            logodircheck.push_str(".ascii");
            // #################################################################################### Why 2 Strings with the logodir? Because reasons i cant explain - maybe i just dont know better (= 
            match distroiter.contains(&dchk[0]){ // ############################################### Fallback to default if logo is not present.
                    true  => {if does_this_exist(&logodircheck) == true {logopath.push_str(dchk[0]);logopath.push_str(".ascii");} else {logopath.push_str("default.ascii")}},
                    false => logopath.push_str("default.ascii"),
            }     
        let fle = File::open(&logopath);
        let dreader = BufReader::new(fle.unwrap());
        let ascii_counter = 42;
        for dline in dreader.lines() {
            let mut asciiline = String::from(dline?);
                while asciiline.len() < ascii_counter {
                    asciiline.push(' ');
                };
            formatvec1.push(asciiline);//43 Chars in width
        }
    while formatvec1.len() < formatvec2.len() {
        let mut filler = String::new();
            while filler.len() < ascii_counter {
                filler.push(' ');
            }
        formatvec1.push(filler); 
    }  
    std::process::Command::new("printf")
    .arg("\\e[?25l")
        .spawn()
            .expect("Printing Failed!");

    for item in formatvec1 {
        std::process::Command::new("printf")
            .arg(item)
            .spawn()
            .expect("Printing Failed!");
            std::process::Command::new("printf")
            .arg("\\e[1E")
            .spawn()
            .expect("Printing Failed!");
        
    }
    std::process::Command::new("printf")
    .arg("\\e[H")
    .spawn()
    .expect("What the fuck?");

    for item in formatvec2 {
        let mut cursor_mover_escapecode = String::new();
        cursor_mover_escapecode.push_str("\\e[");
        cursor_mover_escapecode.push_str(ascii_counter.to_string().as_str());
        cursor_mover_escapecode.push('C');
        std::process::Command::new("printf")
            .arg(cursor_mover_escapecode)
                .spawn()
                    .expect("Printing Failed!");
        std::process::Command::new("printf")
            .arg(item)
                .spawn()
                    .expect("Printing Failed!");
        std::process::Command::new("printf")
            .arg("\\e[1E")
                .spawn()
                    .expect("Printing Failed!");
    }
    //println! ("{}", get_shell());
    std::process::Command::new("printf")
    .arg("\\e[?25h")
        .spawn()
            .expect("Printing Failed!");
    return Ok(());
}