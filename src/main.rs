use std::fs::File;
use std::io::Read;
use std::net::{SocketAddr, ToSocketAddrs};

extern crate clap;
use clap::{Arg, App};

extern crate rayon;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

fn resolve(domain: &str) -> Vec<String> {
    let mut result = Vec::new();

    if let Ok(mut iter) = (domain, 0).to_socket_addrs() {
        while let Some(addr) = iter.next() {
            result.push(
                match addr {
                    SocketAddr::V4(sockv4) => format!("{}", sockv4.ip()),
                    SocketAddr::V6(sockv6) => format!("{}", sockv6.ip())
                }
            );
        }
    }

    result
}

fn expand_stdin(file_name: &str) -> &str {
    if file_name == "-" {
        return "/dev/stdin";
    } else {
        return file_name;
    }
}

fn main() {
    const ARG_HOSTS: &str = "hosts";
    const ARG_THREADS: &str = "threads";
    const ARG_THREADS_DEFAULT: &str = "10";

    let matches = App::new("Resolver.rs")
        .about("Resolve a list of hosts from a file")
        .arg(Arg::with_name(ARG_HOSTS)
            .help("the input file")
            .index(1)
            .required(true)
        )
        .arg(Arg::with_name(ARG_THREADS)
            .help("the amount of threads to use")
            .short("t")
            .long("threads")
            .takes_value(true)
            .default_value(ARG_THREADS_DEFAULT)
            .required(false)
        )
        .get_matches();

    let file_name = expand_stdin(matches.value_of(ARG_HOSTS).unwrap());
    let thread_count: usize = matches.value_of(ARG_THREADS).unwrap().parse().unwrap();

    let result = File::open(file_name);

    if let Err(error) = result {
        println!("{}", error);
        return;
    }

    let mut file = result.ok().unwrap();
    let mut contents = String::new();

    let result = file.read_to_string(&mut contents);
    if let Err(error) = result {
        println!("{}", error);
        return;
    }

    let _ = ThreadPoolBuilder::new().num_threads(thread_count).build_global();

    let lines: Vec<_> = contents.split("\n").collect();
    lines.par_iter().for_each(|&s| {
        let ips = resolve(s);
        if ips.len() > 0 {
            println!("{}={}", s, ips.join(","));
        }
    });
}
