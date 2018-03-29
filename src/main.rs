/*
 * torrentinfo, A torrent file parser
 * Copyright (C) 2018  Daniel Müller
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>
 */

#[macro_use] extern crate clap;
extern crate chrono;
extern crate number_prefix;
extern crate torrentinfo;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use clap::{App, AppSettings, Arg};
use chrono::prelude::*;
use number_prefix::{binary_prefix, Standalone, Prefixed};

use torrentinfo::Torrent;

const VERSION: &'static str = crate_version!();

fn main() {
    let app = App::new("torrentinfo")
        .version(VERSION)
        .about("A torrent file parser")
        .author("Daniel Müller <perlfuchsi@gmail.com>")
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .help("Show files within the torrent")
                .required(false)
                .takes_value(false)
                .conflicts_with_all(&["details", "everything"]),
        )
        .arg(
            Arg::with_name("details")
                .short("d")
                .long("details")
                .help("Show detailed information about the torrent")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("everything")
                .short("e")
                .long("everything")
                .help("Print everything about the torrent")
                .required(false)
                .takes_value(false),
        )
        .arg(Arg::with_name("filename").required(true).takes_value(true));

    let matches = app.get_matches();

    let show_files = matches.is_present("files");
    let show_details = matches.is_present("details");
    let show_everything = matches.is_present("everything");
    let filename = matches.value_of("filename").unwrap();

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Application Error: {}", e);
            process::exit(1);
        }
    };

    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();

    let torrent = Torrent::from_buf(&buf).unwrap();
    let indent = " ".repeat(4);
    let col_width: u32 = 19;

    println!(
        "{}",
        Path::new(filename).file_name().unwrap().to_str().unwrap()
    );
    let info = torrent.info();
    if !show_details || show_everything {
        if let Some(ref v) = info.name {
            print_line("name", &v, &indent, &col_width);
        }
        if let &Some(ref v) = &torrent.comment {
            print_line("comment", &v, &indent, &col_width);
        }
        if let &Some(ref v) = &torrent.announce {
            print_line("announce url", &v, &indent, &col_width);
        }
        if let &Some(ref v) = &torrent.created_by {
            print_line("created by", &v, &indent, &col_width);
        }
        if let &Some(ref v) = &torrent.creation_date {
            let date = Utc.timestamp(*v, 0);
            print_line("created on", &date, &indent, &col_width);
        }
        if let &Some(ref v) = &torrent.encoding {
            print_line("encoding", &v, &indent, &col_width);
        }

        let files = torrent.num_files();
        print_line("num files", &files, &indent, &col_width);

        ;
        let size = match binary_prefix(torrent.total_size() as f64) {
            Standalone(bytes) => format!("{} bytes", bytes),
            Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix)
        };
        print_line("total size", &size, &indent, &col_width);
        let info_hash = torrent.info_hash();
        let mut info_hash_str = String::new();
        for byte in info_hash {
            info_hash_str.push_str(&format!("{:X}", byte));
        }
        print_line("info hash", &info_hash_str, &indent, &col_width);

    }

    if show_files || show_details || show_everything {
        println!("{}{}", indent, "files");
        if let Some(files) = torrent.files() {
            for (index, file) in files.iter().enumerate() {
                println!("{}{}", indent.repeat(2), index);
                println!("{}{}", indent.repeat(3), "path");
                println!("{}{}", indent.repeat(4), file.path.join("/"));
                println!("{}{}", indent.repeat(3), "length");
                println!("{}{}", indent.repeat(4), file.length);
            }
        }
    }

    if show_details || show_everything {
        print_line("piece length", &info.piece_length, &indent, &col_width);
        println!("{}{}", indent, "pieces");
        println!("{}[{} UTF-8 Bytes]", indent.repeat(2), info.pieces.len() * 20);
        print_line("private", &info.private, &indent, &col_width);
    }
}

fn print_line<T: std::fmt::Display>(name: &str, value: &T, indent: &str, col_width: &u32) {
    let n = *col_width as usize - name.len();
    println!("{}{} {}{}", indent, name, " ".repeat(n), value);
}
