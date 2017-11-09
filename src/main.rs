extern crate ftc_http;

use std::iter::Peekable;
use std::env;

static VERISON_STR: &'static str = "v1.0.0";

fn main() {
    let mut args = env::args().skip(1).peekable();

    loop {
        match args.next() {
            Some(arg) => if is_option(&arg) {
                handle_options(&mut args, arg)
            } else {
                help();
                break;
            },
            None => break,
        };
    }
}

fn is_option(arg: &str) -> bool {
    arg.chars().nth(0).unwrap_or(' ') == '-'
}

fn handle_options<I: Iterator<Item = String>>(args: &mut Peekable<I>, options: String) {
    let pwd = env::current_dir().expect("It is not possible to access the current directory");
    let empty_peek = &String::new();
    for option in options.chars().skip(1) {
        match option {
            'd' => {
                {
                    let next_arg = args.peek().unwrap_or(empty_peek);
                    if !is_option(&next_arg) {
                        ftc_http::down(&pwd.join(&next_arg));
                    } else {
                        ftc_http::down(&pwd);
                        continue;
                    }
                }
                args.next();
            }
            'u' => {
                {
                    let next_arg = args.peek().unwrap_or(empty_peek);
                    if !is_option(&next_arg) {
                        ftc_http::up(&pwd.join(&next_arg));
                    } else {
                        ftc_http::up(&pwd);
                        continue;
                    }
                }
                args.next();
            }
            'b' => ftc_http::build(),
            'w' => ftc_http::wipe(),
            'v' => version(),
            _ => {
                help();
                break;
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
