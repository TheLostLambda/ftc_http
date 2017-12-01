extern crate ftc_http;

use std::process;
use std::env;

static VERISON_STR: &'static str = "v1.3.0";
static STARTUP_OPTS: &'static str = "hv";

fn main() {
    let pwd = env::current_dir().expect("It is not possible to access the current directory");

    fn is_option(arg: &str) -> bool {
        arg.chars().nth(0).unwrap_or(' ') == '-'
    }

    let mut args = env::args().skip(1).filter(|arg| !is_option(arg));

    let mut opts: String = env::args()
        .filter(|arg| is_option(arg))
        .map(|arg| arg[1..].to_string())
        .collect();

    if opts.chars().any(|c| STARTUP_OPTS.contains(c)) {
        opts = opts.chars().filter(|&c| STARTUP_OPTS.contains(c)).collect();
    }

    for opt in opts.chars() {
        match opt {
            'd' => {
                let next_arg = args.next().unwrap_or(String::new());
                ftc_http::down(&pwd.join(&next_arg)).unwrap_or_else(|_| {
                    println!("Failed to download files from the robot controller");
                    process::exit(0);
                });
            }
            'u' => {
                let next_arg = args.next().unwrap_or(String::new());
                ftc_http::up(&pwd.join(&next_arg)).unwrap_or_else(|_| {
                    println!("Failed to upload files to the robot controller");
                    process::exit(0);
                });
            }
            'b' => ftc_http::build().unwrap_or_else(|_| {
                    println!("Failed to start build on the robot controller");
                    process::exit(0);
                }),
            'w' => ftc_http::wipe().unwrap_or_else(|_| {
                    println!("Failed to wipe files on the robot controller");
                    process::exit(0);
                }),
            'v' => {
                version();
                process::exit(0);
            }
            _ => {
                help();
                process::exit(0);
            }
        };
    }
}

fn version() {
    println!("ftc_http {}", VERISON_STR);
}

fn help() {
    print!(
        "\
Usage: ftc_http [OPTION]... [FILE]
Provides an interface to FTC OnBotJava without being constrained to a browser.
Actions will be executed in the same order the OPTIONs are given.

Startup:
  -v          Display this version of ftc_http
  -h          Display this help text

Actions:
  -d [DEST]   Downloads the source tree from the robot controller and saves it
                to the location specified by DEST. If no DEST is given, it
                defaults to the current directory.
  -u [SRC]    Uploads .java files to the robot controller. SRC specifies the
                directory in which to search for the Java files. If no SRC is
                given, it defaults to the current directory.
  -b          Initiates a build on the robot controller and reports the build
                status and any errors back the the user.
  -w          Wipes all files from the robot controller. Be sure to run
                ftc_http with the -d option first if you wish to keep any of
                the files on the robot controller.

Please report any bugs here: https://github.com/TheLostLambda/ftc_http
"
    );
}
