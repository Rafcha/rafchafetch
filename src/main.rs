use colored::Colorize;
use sys_info::{os_type, os_release};
mod modules;
use modules::uptime::get_uptime;
use modules::uptime::format_uptime;
use modules::shell::get_shell;
use modules::memandswap::get_memory_info;
use modules::memandswap::get_swap_info;
use modules::hostname::get_hostname;
use modules::ip::get_ip_address;
use std::env;

fn main() {
    let os_type = os_type().unwrap();
    let os_release = os_release().unwrap();
    let hostname = get_hostname();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-a" {

        print!("{}", "⡏⣭⠭⣭⣛⡛⠿⣿⣿⣿⡿⠿⠿⠋⠭⠭⢈⣩⣴⠾⣿⡌⣷⡌⢿⣿⣿         ".cyan());
        println!("{}{}{}{}{}{}{}{}{}{}{}", "r".purple(), "a".blue(), "f".green(), "c".purple(), "h".blue(), "a".yellow(), "f".white(), "e".cyan(), "t".yellow(), "c".cyan(), "h".red());
        print!("{}", " ⣿⣷⣦⢉⡛⢿⡖⢨⣴⡒⢿⣿⣿⣷⣚⡿⢛⣃⣾⣶⣥⡘⣿⡄⢻⣿         ".cyan());
        println!("{}", "------------------------".cyan());
        print!("{}", "⡆⣯⠰⣶⣾⠟⣣⡴⢦⡍⡻⢷⣮⣍⣛⠟⣛⠲⢬⣙⠻⠿⠇⢹⣿⠈⣿         ".cyan());
        println!("{}      {}", "Hostname:".cyan(), hostname.cyan());
        print!("{}", "⠷⠈⣌⢋⣴⣿⡿⣰⣿⡇⢿⡇⢪⣍⣋⠐⢋ ⣳⠁⣤⣄⡛⢘⡁⢶⡎         ".magenta());
        println!("{}       {}", "OS Type:".magenta(), os_type.magenta());
        print!("{}", "⠖⡶⢠⣿⣿⡟⣀⢻⣿⣿⣌⢿⠘⣿⣿⣷⡔⢶⡄⣀⡙⣿⠖⡈⠁⠍⣓         ".magenta());
        println!("{}        {}", "Kernel:".magenta(), os_release.magenta());
        print!("{}", "⠸⠃⡱⣿⣿⢲⡯⢈⠻⣿⣿⣷⣥⡙⠳⣬⣿⡌⣅⠊⢁⡄⠸⣿⠠⣆         ".magenta());
        println!("{}         {}"," Shell:".magenta(), get_shell().magenta());
        print!("{}", "⣶⣿⢡⣿⣿⡘⡡⠛⠓⢮⣙⣈⣙⣓⡸⠒⠨⠅⣹⣿⣮⡻⢠⡶⠘⢿⣷         ".blue());
        match get_uptime() {
            Ok(uptime) => {
                println!("{}        {}","Uptime:".blue(), format_uptime(uptime).blue());
            }
            Err(e) => {
                eprintln!("{}        {}","Uptime:".blue(), e.blue());
            }
        }
        print!("{}", "⣲⣿⢸⣿⣝⠳⠄⠣⡰⢆⣿⣿⣿⣿⣧⣓⣘⠃⣛⣽⣿⢇⣼⡟⠳⠢⣤         ".blue());
        if let Some(mem_stats) = get_memory_info() {
            println!("{}  {}","Memory Usage:".blue(), mem_stats);
        }
        print!("{}", "⢉⣭⠢⠉⠙⠛⣡⣬⡙⢛⡻⠦⠾⠟⢛⣋⡉⠔⣛⠉⠄⣯⡻⢧⡰⣿⢨         ".blue());
        if let Some(swap_stats) = get_swap_info() {
            println!("{}    {}","Swap Usage:".blue(), swap_stats);
        }
        print!("{}", "⣿⣿⡄ ⠾⢡⣛⣏⡹⢶⠖⣤⣾⡿ ⣛⣥⣬⣭⣥⣤⠙⠿⢷⣶⣦⡭         ".green());
        println!("{}    {}","IP Address:".green(), get_ip_address().green());
        print!("{}", "⠎⣉⠛⡈⣴⣾⣟⠻⣿⡶⢿⣿⡾⢃⠮⢍⣉⠉⣒⠲⢶⣾⠇⣴⡆⢩⡻         ".green())

    } else if args.len() > 1 && args[1] == "-b" {

        print!("{}", "⣿⣿⠿⠛⠋⣀⡒⠒⢄⣭⠭⠍⠿⠿⠿⠿⠿⡏⣭⡛⣭⣉⠝⠻⢿         ".cyan());
        println!("{}{}{}{}{}{}{}{}{}{}{}", "r".purple(), "a".blue(), "f".green(), "c".purple(), "h".blue(), "a".yellow(), "f".white(), "e".cyan(), "t".yellow(), "c".cyan(), "h".red());
        print!("{}", "⣿⡇⢰⢸⠘⣨⣵⡆⠠⢀⢤⡶⢒⣶⡲⣦⢄⡀⢉⢉⣐⠐⣮⠴⠌         ".cyan());
        println!("{}", "------------------------".cyan());
        print!("{}", "⣿⠟⣂⣤⣬⠭⠁⢀⡞⢡⠟⣵⣿⣿⣿⢸⣧⡻⡌⣨⠛⠻⠂⠠⠧         ".cyan());
        println!("{}      {}", "Hostname:".cyan(), hostname.cyan());
        print!("{}", "⠿⠋⣡⠶⣫⣵⢠⣟⠬⠴⢦⡸⣿⠟⢏⣸⣧⣷⣿⠿⢸⢸⣄⢿⣷         ".magenta());
        println!("{}       {}", "OS Type:".magenta(), os_type.magenta());
        print!("{}", "⠏⢈⡵⢟⣫⡅⢾⡏⣎⣿⣣⣿⣶⣾⣏⢶⡶⢈⠝ ⡱⡀⠙⠷⣍         ".magenta());
        println!("{}        {}", "Kernel:".magenta(), os_release.magenta());
        print!("{}", "⢃⡀⠘⠿⠿⢳⡈⠳⣝⠻⠿⣯⣉⣽⡿⠿⣛⡅⠘⣷⡝⣢⣤⣤⣿         ".magenta());
        println!("{}         {}","Shell:".magenta(), get_shell().magenta());
        print!("{}", "⣘⣛⣓⣶⠄⢹⣿⠟⣳⠖⣒⣦⣶⣾⡇⡩⠉⣁⣐⠟⠧⡋⠆⠆⣬         ".blue());
        match get_uptime() {
            Ok(uptime) => {
                println!("{}        {}","Uptime:".blue(), format_uptime(uptime).blue());
            }
            Err(e) => {
                eprintln!("{}        {}","Uptime:".blue(), e.blue());
            }
        }
        print!("{}", "⣿⣟⣉⣿⣿⣿⠫⠈⣁⣈⣉⣉⣉⣉⠈⠚⠻⠿⠻⠂⢱⣂⣠⣿⣛         ".blue());
        if let Some(mem_stats) = get_memory_info() {
            println!("{}  {}","Memory Usage:".blue(), mem_stats);
        }
        print!("{}", "⣿⠛⢙⣿⣿⢏⡜⠼⣿⣿⠻⣿⣿⠿⣿⡀⡈⢿⣿⣝⢌⠻⣿⣿⡟         ".blue());
        if let Some(swap_stats) = get_swap_info() {
            println!("{}    {}","Swap Usage:".blue(), swap_stats);
        }
        print!("{}", "⣿⣿⣿⠟⣡⣾⠘⡀⣉⡉⣀⣉⠁⠐⢛⣛⡀⢱⡝⣿⣷⣥⡘⢿⣿         ".green());
        println!("{}    {}","IP Address:".green(), get_ip_address().green());
        print!("{}", "⢿⢟⣡⣾⡿⣱⣿⣼⡿⣼⣿⣿⢜⣿⣎⢷⣴⡆⣿⣮⡻⣿⣿⣷⣌         ".green())

    } else if args.len() > 1 && args[1] == "-c" {

        print!("{}", "⣿⣿⣿⣿⠿⢛⢋⣡⢄⡶⣲⣖⣶⣶⠆⡤⣔⢶⣶⣶⣌⣍⣛⠻⢿⣿⣿         ".cyan());
        println!("{}{}{}{}{}{}{}{}{}{}{}", "r".purple(), "a".blue(), "f".green(), "c".purple(), "h".blue(), "a".yellow(), "f".white(), "e".cyan(), "t".yellow(), "c".cyan(), "h".red());
        print!("{}", "⣿⣿⠟⣡⢃⣷⣿⣵⣿⣾⠟⠸⢻⣿⡜⣿⣜⣗⢝⠻⣟⢝⢿⡳⢕⡝⣿         ".cyan());
        println!("{}", "------------------------".cyan());
        print!("{}", "⣿⢯⡸⢳⡿⣿⣿⡟⡟⠄⣼⣆⠫⣛⢾⢞⢮⡛⠿⡭⡊⠳⡃⣻⡜⣞⡜         ".cyan());
        println!("{}      {}", "Hostname:".cyan(), hostname.cyan());
        print!("{}", "⡟⢀⠋⣿⡇⡟⢸⢱⡇⢰⣿⣿⣥⡈⠡⠑⠳⢉⠊⠈⠌ ⡇⠳⢧⢹⠃         ".magenta());
        println!("{}       {}", "OS Type:".magenta(), os_type.magenta());
        print!("{}", "⣧⠸⠰⢿⣧⢇⠸ ⡁⣌⠋⠋⢽⣿⣶⣴⣶⣦⢔⡈⠁⠢⡀ ⡆⢸⡀         ".magenta());
        println!("{}        {}", "Kernel:".magenta(), os_release.magenta());
        print!("{}", "⣿  ⣏⣿⣾ ⢘⡄⠩⣤⡤⢠⣿⣿⣿⣿⣿⣄⣶⣖⣀⣧⠐⢰⡆⡇         ".magenta());
        println!("{}         {}","Shell:".magenta(), get_shell().magenta());
        print!("{}", "⣿⢀⡏⣭⢊⢿⢧⡈⢿⣯⣿⣿⣿⣯⣉⣉⣽⣿⣿⣿⣾⠟⠁⣾⠇⠇⢱         ".blue());
        match get_uptime() {
            Ok(uptime) => {
                println!("{}        {}","Uptime:".blue(), format_uptime(uptime).blue());
            }
            Err(e) => {
                eprintln!("{}        {}","Uptime:".blue(), e.blue());
            }
        }
        print!("{}", "⠏⣾⣇⡿⣀⢣⠆⢃⢀⡠⢈⣭⣷⣿⣭⠉⣽⣶⣶⡄ ⢀⣾⣿⢀⢸⣇         ".blue());
        if let Some(mem_stats) = get_memory_info() {
            println!("{}  {}","Memory Usage:".blue(), mem_stats);
        }
        print!("{}", "⣐⣿⣻⡿⣣⢠ ⠈⢠⠏⡆⣿⣿⣿⣿⢸⣿⣿⣿⡟ ⢸⡿⠁⠈⣿⣟         ".blue());
        if let Some(swap_stats) = get_swap_info() {
            println!("{}    {}","Swap Usage:".blue(), swap_stats);
        }
        print!("{}", "⠑⢇⠿⣿⠿⠋  ⠓⢠⡸⠿⢿⣿⠃⠈⣿⣿⣿⠇⡆⠘⢧⢤⣰⡇⡿         ".green());
        println!("{}    {}","IP Address:".green(), get_ip_address().green());
        print!("{}", "⠈⠷⡀⣄⢲⡀  ⢰⣼⠔⡋⠤⠔⠂⠐⠡⠭⢙⢰⡄ ⠈⠣⠛⠁⠁          ".green())

    } else {

        print!("{}", "⣿⣿⢀⣯⢹⣷⣶⣬⡉⠥⣂⠤⣒⣀⠲⠆⡒⢬⠍⣩⣴⣾⣿⡿⣩⣆⢻         ".cyan());
        println!("{}{}{}{}{}{}{}{}{}{}{}", "r".purple(), "a".blue(), "f".green(), "c".purple(), "h".blue(), "a".yellow(), "f".white(), "e".cyan(), "t".yellow(), "c".cyan(), "h".red());
        print!("{}", "⣿⡇⠼⠿⣦⠹⣿⣿⠏⣼⢃⠚⢛⣉⣁⣈⣐⣥⣾⣿⣿⣿⠟⢱⡿⢿⢸         ".cyan());
        println!("{}", "------------------------".cyan());
        print!("{}", "⣿⠰⣾⣷⣴⡶⢘⣫⣤⠙⠂⠩⢽⣿⣿⢻⣿⢿⣿⣿⣭⡋⠾⢷⣶⡶⢸         ".cyan());
        println!("{}      {}", "Hostname:".cyan(), hostname.cyan());
        print!("{}", "⠅⠺⢿⡥⢊⣴⣿⣿⢃⣾⣿⣾⣿⣿⡟⢸⣿⣧⡙⢿⣿⣿⣷⣄⠺⣷⢼         ".magenta());
        println!("{}       {}", "OS Type:".magenta(), os_type.magenta());
        print!("{}", "⣼⣶⠌⣴⣿⣿⣿⠃⣸⣿⣿⣿⣿⣿⣦⢸⣿⣿⣿⣦⢻⣿⣿⣿⣷⡐⢰         ".magenta());
        println!("{}        {}", "Kernel:".magenta(), os_release.magenta());
        print!("{}", "⠿⠁⣿⣿⣿⡇⢈⣴⣌⠻⡈⠻⣿⣿⣿⡙⠢⠛⢿⣿⣟⠈⣿⡿⣿⣿⡄         ".magenta());
        println!("{}         {}","Shell:".magenta(), get_shell().magenta());
        print!("{}", "⣾⡄⣿⣿⣿⡇⣿⠿⠓⣀⡀⠈⣦⣭⣍⣉⡀⡀⡀⠈⢭⠁⣤⣰⣿⣿⠃         ".blue());
        match get_uptime() {
            Ok(uptime) => {
                println!("{}        {}","Uptime:".blue(), format_uptime(uptime).blue());
            }
            Err(e) => {
                eprintln!("{}        {}","Uptime:".blue(), e.blue());
            }
        }
        print!("{}", "⣿⣷⡘ ⠻⣧⠨⣰⡘⢗⡚⢸⣿⡿⣿⣿⡅⢃⡸⢰⡤⢠⣿⡟⠻⠋⣼         ".blue());
        if let Some(mem_stats) = get_memory_info() {
            println!("{}  {}","Memory Usage:".blue(), mem_stats);
        }
        print!("{}", "⣿⣟⢿⢠⡤⢦⡥⠸⠷⣶⣲⣿⡿⠿⢿⣿⣷⣶⣶⡏⢐⡛⣋⡄⠶⣹⣿         ".blue());
        if let Some(swap_stats) = get_swap_info() {
            println!("{}    {}","Swap Usage:".blue(), swap_stats);
        }
        print!("{}", "⣬⡙⠆⠢⠁⠶⠖⣿⡆⠭⠝⠛⠛⠒⠛⠛⢛⡩⠁ ⢺⠿⠿⠇⣼⣿⡷         ".green());
        println!("{}    {}","IP Address:".green(), get_ip_address().green());
    }

}
