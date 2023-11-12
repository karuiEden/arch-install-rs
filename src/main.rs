use std::io;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::process::{Command, exit, Stdio};
struct SystemConfig {
    kernel: String,
    password_root: String,
    user: UserConfig,
    choose_de: String,
    hostname: String,
    locale: String,
    login_manager: String,
}
struct UserConfig {
    username: String,
    password: String,
    shell: String,
    user_admin: bool,
}
fn input() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed");
    if answer.len() == 0 {
        panic!("Shutdown");
    } else {
        return answer;
    }
}

fn passwd() -> String {
    let mut pass1 = String::new();
    let mut pass2 = String::new();
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
    let de = [
        "GNOME",
        "KDE Plasma",
        "Xfce",
        "Cinnamon",
        "Mate",
        "i3",
        "dwm",
        "sway",
    ];
    let mut system_config = SystemConfig {
        kernel: String::new(),
        password_root: String::new(),
        user: UserConfig {
            username :String::new(),
            user_admin: false,
            shell: String::new(),
            password: String::new(),
        },
        choose_de: String::new(),
        hostname: String::new(),
        locale: String::new(),
        login_manager: String::new(),
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
            }
            "2" => {
                system_config.kernel = String::from("linux-lts");
                break;
            }
            "3" => {
                system_config.kernel = String::from("linux-zen");
                break;
            }
            "4" => {
                system_config.kernel = String::from("linux-hardened");
                break;
            }
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Choose  a language:");
    for i in 1..6 {
        println!("{}. {} language", i, locales[i - 1]);
    }
    loop {
        answer = input();
        match answer.trim_end() {
            "1" => {
                system_config.locale = String::from("en_US.UTF-8");
                break;
            }
            "2" => {
                system_config.locale = String::from("ru_RU.UTF-8");
                break;
            }
            "3" => {
                system_config.locale = String::from("de_DE.UTF-8");
                break;
            }
            "4" => {
                system_config.locale = String::from("fr_FR.UTF-8");
                break;
            }
            "5" => {
                system_config.locale = String::from("fr_FR.UTF-8");
                break;
            }
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Enter root password:");
    system_config.password_root = passwd();
    println!("Enter username:");
    system_config.user.username = input();
    println!("Enter username password:");
    system_config.user.password = passwd();
    println!(
        "Add user {} to the \"wheel\" group?",
        system_config.user.username
    );
    println!("Enter [Y/N] to add");
    loop {
        answer = input();
        match answer.trim_end() {
            "Y" | "y" => {
                system_config.user.user_admin = true;
                break;
            }
            "N" | "n" => {
                system_config.user.user_admin = false;
                break;
            }
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    println!("Choose shell:");
    println!("1. bash");
    println!("2. zsh");
    println!("3. fish");
    loop {
        answer = input();
        match answer.trim_end() {
            "1" => {
                system_config.user.shell = String::from("bash");
                break;
            },
            "2" => {
                system_config.user.shell = String::from("zsh");
                break;
            },
            "3" => {
                system_config.user.shell = String::from("fish")
            },
            _ => continue,
        }
    }
    println!("Choose Desktop environment:");
    for i in 1..9 {
        println!("{}. {}", i, de[i - 1]);
    }
    loop {
        answer = input();
        match answer.trim_end() {
            "1" => {
                system_config.choose_de = String::from("gnome");
                system_config.login_manager = String::from("gdm");
                break;
            }
            "2" => {
                system_config.choose_de = String::from("plasma");
                system_config.login_manager = String::from("sddm");
                break;
            }
            "3" => {
                system_config.choose_de = String::from("xfce4");
                system_config.login_manager = String::from("lightdm");
                break;
            }
            "4" => {
                system_config.choose_de = String::from("cinnamon");
                system_config.login_manager = String::from("lightdm");
                break;
            }
            "5" => {
                system_config.choose_de = String::from("mate");
                system_config.login_manager = String::from("lightdm");
                break;
            }
            "6" => {
                system_config.choose_de = String::from("i3");
                system_config.login_manager = String::from("lightdm");
                break;
            }
            "7" => {
                system_config.choose_de = String::from("dwm");
                system_config.login_manager = String::from("lightdm");
                break;
            }
            "8" => {
                system_config.choose_de = String::from("sway");
                system_config.login_manager = String::from("lightdm");
            }
            _ => {
                answer.clear();
                continue;
            }
        };
    }
    install(system_config);
}

fn install(sc: SystemConfig) {
    let pacstrap = Command::new("pacstrap")
        .args(["-K", "/mnt", "base", "base-devel", "linux-firmware", sc.choose_de.as_str(), "networkmanager", "xorg",
            "pipewire", "firefox", "unzip", "unrar", "grub", "intel-ucode", "amd-ucode", "xdg-utils", "xdg-user-dirs"])
        .arg(sc.kernel.trim_end())
        .status()
        .unwrap();
    if !pacstrap.success() {
        exit(0);
    }

    let genfstab = Command::new("/bin/bash")
        .args(["-c", "genfstab -U /mnt >> /mnt/etc/fstab"])
        .status()
        .unwrap();
    if !genfstab.success() {
        exit(0);
    }
    let hwclock = Command::new("arch-chroot")
        .arg("/mnt")
        .args(["hwclock", "--systohc"])
        .status()
        .expect("Fail sync hard clock");
    if !hwclock.success() {
        exit(0);
    }
    let set_clock = Command::new("arch-chroot")
        .arg("/mnt")
        .args(["ln", "-sf", "/usr/share/zoneinfo/Europe/Moscow" ,"/etc/localtime"])
        .status()
        .unwrap();
    if !set_clock.success() {
        exit(0);
    }
    hostname(&sc.hostname);
    locale(sc.locale);
    hosts(&sc.hostname);
    passwd_root(sc.password_root);
    create_user(sc.user);
    let nm = String::from("NetworkManager");
    systemd(nm);
    systemd(sc.login_manager);
    boot_loader();
    println!("Installation complete");
}

fn boot_loader() {
    let install = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("grub-install")
        .status()
        .unwrap();
    if !install.success() {
        exit(0);
    }
    let config = Command::new("arch-chroot")
        .arg("/mnt")
        .args(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
        .status()
        .unwrap();
    if !config.success() {
        exit(0)
    }
}
enum Result<T,E> {
    Ok(T),
    Err(E),
}

fn create_user(user: UserConfig) {
    let mut groups = String::from("-g users");
    if user.user_admin == true {
        groups.push_str("-G wheel");
    }
    let create = Command::new("arch-chroot")
        .arg("/mnt")
        .args(["useradd", "-m", groups.trim_end(), "-s", format!("/bin/{}", user.shell).as_str().trim_end(), "-p", user.password.trim_end(), user.username.trim_end()])
        .status()
        .unwrap();
    if !create.success() {
        exit(0);
    }
}
fn passwd_root(pass: String) {
    let mut echo = Command::new("arch-chroot")
        .arg("/mnt")
        .args(["echo", format!("root:{}", pass).as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(echo_output) = echo.stdout.take() {
        Command::new("arch-chroot")
            .arg("/mnt")
            .arg("chpasswd")
            .stdin(echo_output)
            .spawn()
            .unwrap();
        echo.wait().unwrap();
    }
}
fn systemd(service: String) {
    let systemctl = Command::new("arch-chroot")
        .arg("/mnt")
        .args(["systemctl", "enable", service.as_str()])
        .status()
        .unwrap();
    if !systemctl.success() {
        exit(0);
    }
}
fn hostname(host_name: &String) {
    let mut host = File::open("/mnt/etc/hostname").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("/mnt/etc/hostname").unwrap_or_else(|error|{
                panic!("Error create file {:?}", error);
            })
        } else {
            panic!("Error open file {:?}", error);
        }
    });
    host.write_all(host_name.as_ref()).unwrap();
}


fn locale(locale_config: String) {
    if locale_config == "en_US.UTF-8" {
        let mut locale_gen = File::open("/mnt/etc/locale.gen").unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound {
                File::create("/mnt/etc/locale.gen").unwrap_or_else(|error|{
                    panic!("Error create file {:?}", error);
                })
            } else {
                panic!("Error open file {:?}", error);
            }
        });
        locale_gen.write_all(format!("{} UTF-8", locale_config).as_bytes()).unwrap();
        let locale_gen = Command::new("arch-chroot")
            .arg("/mnt")
            .args(["locale-gen"])
            .status()
            .unwrap();
        if !locale_gen.success() {
            exit(0);
        }
        let mut locale_conf = File::open("/mnt/etc/locale.conf").unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound {
                File::create("/mnt/etc/locale.conf").unwrap_or_else(|error|{
                    panic!("Error create file {:?}", error);
                })
            } else {
                panic!("Error open file {:?}", error);
            }
        });
        locale_conf.write_all(format!("LANG={}\nLC_TIME={}\nLC_COLLATE=C", locale_config, locale_config).as_ref()).unwrap();
        } else {
        let locale_gen = Command::new("arch-chroot")
            .arg("/mnt")
            .args(["locale-gen"])
            .status()
            .unwrap();
        if !locale_gen.success() {
            exit(0);
        }
        let mut locale_conf = File::open("/mnt/etc/locale.conf").unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound {
                File::create("/mnt/etc/locale.conf").unwrap_or_else(|error|{
                    panic!("Error create file {:?}", error);
                })
            } else {
                panic!("Error open file {:?}", error);
            }
        });
        locale_conf.write_all(format!("LANG={}\nLC_TIME={}\nLC_COLLATE=C", locale_config, locale_config).as_ref()).unwrap();
    }
}

fn hosts(hostname: &String) {
    let mut host = File::open("/mnt/etc/hosts").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("/mnt/etc/hosts").unwrap_or_else(|error|{
                panic!("Error create file {:?}", error);
            })
        } else {
            panic!("Error open file {:?}", error);
        }
    });
    host.write_all(format!("127.0.0.1        localhost\n::1              localhost\n127.0.1.1        {}", hostname).as_ref()).unwrap();
}