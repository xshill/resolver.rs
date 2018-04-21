# Resolver.rs
Resolver.rs is a rust tool for resolving a list of domain names.

## Installing
To install this tool, you need to
[install rust](https://www.rust-lang.org/en-US/install.html). Then, follow
these instructions:

```
git clone https://github.com/xshill/resolver.rs
cd resolver.rs
cargo install
```

## Usage
To resolve a list of hosts in a file:

```
resolver /home/user/host_list
```

You can set the thread count using the `-t` flag:

```
resolver /home/usr/host_list -t 100
```

## Output format
The output is formatted in a way that is intended to facilitate scripting. The
format is:

```
<DOMAIN>=<IP_1>,<IP_2>,<...>
```

Domains that do not resolve will not be displayed.

## Dependencies
This tool uses the [clap](https://github.com/kbknapp/clap-rs) and
[rayon](https://github.com/rayon-rs/rayon) libraries.
