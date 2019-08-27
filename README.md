# rust-time
Intratime rust CLI

```
USAGE:
    rust-time [OPTIONS] <email> <password>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    --in <HH:MM:SS | false>         [default: false]
    --break <HH:MM:SS | false>      [default: false]
    --return <HH:MM:SS | false>     [default: false]
    --out <HH:MM:SS | false>        [default: false]
    --date <YYYY-MM-DD>             [default: 2019-08-27]

ARGS:
    <email>
    <password>

EXAMPLE:
    ./rust-time somebody@someemail.com 9876 --in 09:02:13
```
