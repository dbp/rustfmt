
use std(vers = "0.3");
use syntax(vers = "0.3");

import core::*;

import result::{ok, err};
import option::{none, some};
import std::getopts;
import getopts::{opt_present, optflag};

import syntax::{parse, codemap};
import syntax::parse::{new_parse_sess, parse_sess};
import syntax::diagnostic::{mk_handler, mk_span_handler};
import syntax::print::pprust;

// copied from rustc/driver/driver.rs
fn anon_src() -> ~str { ~"<anon>" }
fn source_name(input: input) -> ~str {
    match input {
      file_input(ifile) => copy ifile,
      str_input(_) => anon_src()
    }
}
enum input {
    /// Load source from file
    file_input(~str),
    /// The string is the source
    str_input(~str)
}

fn usage() {
    io::println(~"Usage: rustfmt filename\n" +
                ~"       rustfmt - (reads from standard in)\n" +
                ~"       rustfmt [-h|--help] (this message)");
}

fn main(args: ~[~str]) {
    let mut args = copy args;
    vec::shift(args); // get rid of binary

    let matches =
        match getopts::getopts(args, ~[optflag(~"h"), optflag(~"help")]) {
          ok(m) => copy m,
          err(f) => {
            fail getopts::fail_str(f);
          }
        };

    if opt_present(matches, ~"h") || opt_present(matches, ~"help") {
        usage();
        return;
    }

    let input = match vec::len(matches.free) {
      0u => { io::println(~"no input filename given"); return; }
      1u => {
        let ifile = copy matches.free[0];
        if ifile == ~"-" {
            let src = str::from_bytes(io::stdin().read_whole_stream());
            str_input(src)
        } else {
            file_input(ifile)
        }
      }
      _ => { io::println(~"multiple input filenames provided"); return; }
    };


    // run pretty printer
    let codemap = codemap::new_codemap();
    let span_diagnostic = mk_span_handler(mk_handler(none), codemap);
    let parse_sess = parse::new_parse_sess_special_handler(span_diagnostic,
                                                           codemap);
    let crate = match input {
      file_input(file) => {
        parse::parse_crate_from_file(file, ~[], parse_sess)
      }
      str_input(src) => {
        // FIXME (#2319 on rust tracker): Don't really want to box the source string
        parse::parse_crate_from_source_str(
            anon_src(), @(copy src), ~[], parse_sess)
      }
    };

    let src = codemap::get_filemap(codemap, source_name(input)).src;

    do io::with_str_reader(*src) |rdr| {
        pprust::print_crate(parse_sess.cm, parse_sess.interner,
                            parse_sess.span_diagnostic, crate, 
                            source_name(input), rdr, io::stdout(),
                            pprust::no_ann(), false);
    }
    return;
}