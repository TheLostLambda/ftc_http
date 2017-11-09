```
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
```
