branch-d is a tiny little cli use to clean up branches which have been merged in github

### Installation

For anyone familiar with Rust:

`cargo install branch-d`

If you are not, the easiest way is [https://www.rustup.rs/](https://www.rustup.rs/)

### Usage

```
USAGE:
    branch-d [FLAGS] [OPTIONS]

FLAGS:
    -f               fetch all branches before delete
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <BRANCH>        Delete branches which have been merged into input branch
```

### Example

```curl
branch-d -b develop -f
```
