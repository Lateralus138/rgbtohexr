use colorful::Colorful;
use std::{
    env::args,
    process::exit,
    collections::VecDeque
};
use regex::Regex;
fn main() {
    let mut arg_vector: Vec<String> = args().collect();
    arg_vector.remove(0);
    let mut arg_vector_new: VecDeque<String> = VecDeque::new();
    let mut html_bool: bool = false;
    let mut hex_bool: bool = false;
    let mut skip_bool: bool = false;
    let mut upper_bool: bool = false;
    let html_regex = Regex::new(r"^-([tT]|-[hH][tT][mM][lL])$").unwrap();
    let hex_regex = Regex::new(r"^-([xX]|-[hH][eE][xX])$").unwrap();
    let help_regex = Regex::new(r"^-([hH]|-[hH][eE][lL][pP])$").unwrap();
    let upper_regex = Regex::new(r"^-([uU]|-[uU][pP][pP][eE][rR])$").unwrap();
    let upper_hex_regex = Regex::new(r"^-([uU][xX]|[xX][uU])$").unwrap();
    let upper_html_regex = Regex::new(r"^-([uU][tT]|[tT][uU])$").unwrap();
    let mut mode: u8 = 0;
    if arg_vector.len() > 0 {
        for arg_iter in &arg_vector {
            if help_regex.is_match(&arg_iter) {
                let opts_help = String::from("OPTIONS").green().bold();
                let usage_help = String::from("USAGE").magenta().bold(); 
                let int_help = String::from("INTEGERS").blue().bold();
                let err_help = String::from("ERRORS").red().bold();
                let red_help = String::from("Red").red().bold(); 
                let green_help = String::from("Green").green().bold();
                let blue_help = String::from("Blue").blue().bold();
                print!(
                    "\n 'rgbtohexr'\n \
                    Convert decimal [{}], [{}], and\n \
                    [{}] colors to hexadecimal format.\n \
                    \n \
                    @{}:\n\
                    \trgbtohexr [{}...]\n \
                    \trgbtohexr {}...\n \
                    \trgbtohexr [{}...] {}...\n\n \
                    @{}:\n\
                    \t-h,--help\tThis help screen.\n\
                    \t-u,--upper\tRetrieve output as\n\
                    \t\t\tuppercase.\n\
                    \t-t,--html\tRetrieve output as\n\
                    \t\t\t'HTML':'#FFFFFF'\n \
                    \t-x,--hex\tRetrieve output as\n\
                    \t\t\t'HEX':'0xFFFFFF'\n \
                    \t\t\tDefault format:'FFFFFF'\n\n \
                    @{}:\n \
                    \t3 integers from 0-255 that represent the \n\
                    \tcorresponding [{}],[{}], and [{}]\n \
                    \tcolors.\n\n \
                    @{}:\tExit Codes\n\
                    \t0\tNo errors.\n\
                    \t1\tNot enough arguments passed.\n\
                    \t2\tPassed argument is not an\n\
                    \t\tinteger between 0-255.\n\n",
                    red_help,green_help,blue_help,
                    usage_help,opts_help, int_help, opts_help,
                    int_help,opts_help,int_help,
                    red_help,green_help,blue_help,
                    err_help);
                exit(0);
            }
            if html_regex.is_match(&arg_iter) {
                skip_bool = true;
                if ! html_bool {
                    mode += 1;
                }
                html_bool = true;
            }
            if html_regex.is_match(&arg_iter) {
                skip_bool = true;
                if ! html_bool {
                    mode += 1;
                }
                html_bool = true;
            }
            if hex_regex.is_match(&arg_iter) {
                skip_bool = true;
                if ! hex_bool {
                    mode += 2;
                }
                hex_bool = true;
            }
            if upper_regex.is_match(&arg_iter) {
                skip_bool = true;
                if ! upper_bool {
                    mode += 4;
                }
                upper_bool = true;
            }

            if upper_hex_regex.is_match(&arg_iter) {
                skip_bool = true;
                if ! upper_bool {
                    mode += 4;
                }
                upper_bool = true;
                if ! hex_bool {
                    mode += 2;
                }
                hex_bool = true;
            }
            if upper_html_regex.is_match(&arg_iter) {
                skip_bool = true;
                if ! upper_bool {
                    mode += 4;
                }
                upper_bool = true;
                if ! html_bool {
                    mode += 1;
                }
                html_bool = true;
            }
            if ! skip_bool {
                arg_vector_new.push_back(
                    (&arg_iter as &str).to_string()
                );
            } else {
                skip_bool = false;
            }
        }
    }
    if arg_vector_new.is_empty() {
        for arg_iter in &arg_vector {
            arg_vector_new.push_back(
                (&arg_iter as &str).to_string()
            );            
        }
    }
    let arg_vector_new = match arg_vector_new.len() >= 3 {
        true => {
            for int in 0..3 {
                if arg_vector_new[int].parse::<u8>().is_err() {
                    println!(" {}.","Passed argument is not an integer between 0-255".red().bold());
                    exit(2);            
                }
            }
            arg_vector_new
        },
        _ => {
            println!(" {}.","Not enough arguments passed".red().bold());
            exit(1);
        }
    };
    let rgb_decimal = (u32::pow(256,2)*arg_vector_new[0].parse::<u32>().unwrap())
                    + (u32::pow(256,1)*arg_vector_new[1].parse::<u32>().unwrap())
                    + (u32::pow(256,0)*arg_vector_new[2].parse::<u32>().unwrap());
    let hex = match mode {
        1 => format!("#{:06x}",rgb_decimal),
        2 => format!("0x{:06x}",rgb_decimal),
        3 => format!("#{:06x}",rgb_decimal),
        4 => format!("{:06X}",rgb_decimal),
        5 => format!("#{:06X}",rgb_decimal),
        6 => format!("0x{:06X}",rgb_decimal),
        _ => format!("{:06x}",rgb_decimal)
    };
    println!("{}",hex);
}
