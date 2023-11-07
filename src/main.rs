use std::env::args;
use std::fmt::format;
use std::process::{Command, CommandArgs, Stdio};
use std::io;
use std::path::Prefix::DeviceNS;

struct SystemConfig {
    kernel: String,
    password_root: String,
    username: String,
    choose_de: String,
    user_admin: bool,
    password: String,
    hostname: String,
    locale: String,
}



fn input() -> String {
    let mut answer= String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed");
    if answer.len() == 0 {
        panic!("Shutdown");
    } else {
    return answer;
        }
}

fn passwd() -> String {
    let mut pass1 = String::new();
    let mut pass2= String::new();
    loop {
        pass1 = input();
        pass2 = input();
        if pass1 == pass2 {
            break;
        } else {
            println!("Password doesn't match");
            pass1.clear();
            pass2.clear();
            continue;
        }
    }
    return pass1;
}

fn main() {
    let locales = ["English", "Russian", "German", "French", "Spanish"];
    let DE = ["GNOME", "KDE Plasma", "Xfce", "Cinnamon", "Mate", "i3", "dwm", "sway"];
    let mut system_config = SystemConfig {
        kernel: String::new(),
        password_root: String::new(),
        username: String::new(),
        choose_de: String::new(),
        password: String::new(),
        user_admin: false,
        hostname: String::new(),
        locale: String::new(),
    };
    println!("Arch Installer by karui");
    println!("Do you mount all partitions?");
    let mut answer = String::new();
    println!("Enter [Y/N] to continue");
    loop {
        answer = input();
        match answer.trim_end() {
            "Y" | "y" => break,
            "N" | "n" => panic!("Mount partitions."),
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Choose a kernel:\n1. Linux\n2. Linux LTS\n3. Linux Zen\n4. Linux Hardened");
    loop {
        answer = input();
        match answer.trim_end() {
            "1" => {
                system_config.kernel = String::from("linux");
                break;
            },
            "2" => {
                system_config.kernel = String::from("linux-lts");
                break;
            },
            "3" => {
                system_config.kernel = String::from("linux-zen");
                break;
            },
            "4" => {
                system_config.kernel = String::from("linux-hardened");
                break;
            },
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Choose  a language:");
    for i in 1..6 {
        println!("{}. {} language", i, locales[i - 1]);
    };
    loop {
        answer = input();
        match answer.trim_end() {
            "1" => {
                system_config.locale = String::from("en_US.UTF-8");
                break;
            },
            "2" => {
                system_config.locale = String::from("ru_RU.UTF-8");
                break;
            },
            "3" => {
                system_config.locale = String::from("de_DE.UTF-8");
                break;
            },
            "4" => {
                system_config.locale = String::from("fr_FR.UTF-8");
                break;
            },
            "5" => {
                system_config.locale = String::from("fr_FR.UTF-8");
                break;
            },
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Enter root password:");
    system_config.password_root = passwd();
    println!("Enter username:");
    system_config.username = input();
    println!("Enter username password:");
    system_config.password = passwd();
    println!("Add user {} to the \"wheel\" group?", system_config.username);
    println!("Enter [Y/N] to add");
    loop {
        answer = input();
        match answer.trim_end() {
            "Y" | "y" => {
                system_config.user_admin = true;
                break;
            },
            "N" | "n" => {
                system_config.user_admin = false;
                break;
            }
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Choose Desktop environment:");
    for i in 1..9 {
        println!("{}. {}", i, DE[i - 1]);
    };
    loop {
        answer = input();
        match answer.trim_end() {
            "1" => {
                system_config.choose_de = String::from("gnome");
                break;
            },
            "2" => {
                system_config.choose_de = String::from("plasma");
                break;
            },
            "3" => {
                system_config.choose_de = String::from("xfce4");
                break;
            },
            "4" => {
                system_config.choose_de = String::from("cinnamon");
                break;
            },
            "5" => {
                system_config.choose_de = String::from("mate");
                break;
            },
            "6" => {
                system_config.choose_de = String::from("i3");
                break;
            },
            "7" => {
                system_config.choose_de = String::from("dwm");
                break;
            },
            "8" => {
                system_config.choose_de = String::from("sway");
            },
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    install(system_config);
}

fn mount(directory: &String, flag: &String) {
    if flag == "--make-rslave" {
        let mount_check = Command::new("mount")
            .arg(flag.trim_end())
            .arg(format!("/mnt/{}", directory).trim_end())
            .spawn();
        match mount_check {
            Ok(..) => ..,
            Err(..) => panic!("Failed mount {} with flag {}", directory, flag)
        };
    } else {
        let mount_check = Command::new("mount")
            .arg(flag.trim_end())
            .arg(format!("/{}", directory).trim_end())
            .arg(format!("/mnt/{}", directory).trim_end())
            .spawn();
        match mount_check {
            Ok(..) => ..,
            Err(..) => panic!("Failed mount {} with flag {}", directory, flag)
        };
    }
}

fn install(sc: SystemConfig) {
    let sys = String::from("sys");
    let proc = String::from("proc");
    let dev = String::from("dev");
    let flag1 = String::from("--rbind");
    let flag2 = String::from("--make-rslave");
    let flag3 = String::from("--types");
    let pacstrap = Command::new("pacstrap -K /mnt")
        .arg("base base-devel linux-firmware")
        .arg(sc.kernel.trim_end())
        .spawn();
    match pacstrap {
        Ok(..) => ..,
        Err(..) => panic!("Failed start pacstrap")
    };
    let genfstab = Command::new("genfstab /mnt >> /mnt/etc/fstab")
        .spawn();
    match genfstab {
        Ok(..) => ..,
        Err(..) => panic!("Failed start genfstab")
    };
    mount(&sys, &flag1);
    mount(&sys, &flag2);
    mount(&dev, &flag1);
    mount(&dev, &flag2);
    mount(&proc, &flag3);
    let hwclock = Command::new("hwclock --systohc")
        .spawn();
    match hwclock {
        Ok(..) => ..,
        Err(..) => panic!("Error set hardware clock")
    };
    let set_clock = Command::new("ln -sf /usr/share/zoneinfo/Europe/Moscow /etc/localtime")
        .spawn();
    match set_clock {
        Ok(..) => ..,
        Err(..) => panic!("Error set clock")
    };

}
//# mount --rbind /sys/firmware/efi/efivars sys/firmware/efi/efivars/

fn locale(locale_config: String) {
    if locale_config == "en_US.UTF-8" {
        let locale = Command::new("echo")
            .arg(locale_config.trim_end())
            .arg("UTF-8 > /etc/locale.gen")
            .spawn();
        match locale {
            Ok(..) => ..,
            Err(..) => panic!("Error adding locale")
        };

    } else {
        let locale = Command::new("echo")
            .arg(locale_config.trim_end())
            .arg("UT")
            .arg("> /etc/locale.gen")
            .spawn();
        match locale {
            Ok(..) => ..,
            Err(..) => panic!("Error adding locale")
        };

    }
}
