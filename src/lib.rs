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

extern crate bencode;
extern crate sha1;

use std::collections::BTreeMap;

use bencode::{Bencode, FromBencode, ToBencode};
use bencode::util::ByteString;
use sha1::{Sha1, Digest};

#[derive(Debug)]
pub enum TorrentError {
    NotADict,
    NotANumber,
    NotAByteString,
}

#[derive(PartialEq, Debug)]
pub struct Torrent {
    pub announce: Option<String>,
    pub announce_list: Option<Vec<String>>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub creation_date: Option<i64>,
    pub encoding: Option<String>,
    pub info: Info,
}

impl FromBencode for Torrent {
    type Err = TorrentError;

    fn from_bencode(bencode: &Bencode) -> Result<Self, <Self as FromBencode>::Err> {
        use TorrentError::*;

        match bencode {
            &Bencode::Dict(ref m) => {
                let mut torrent = Torrent {
                    announce: None,
                    announce_list: None,
                    comment: None,
                    created_by: None,
                    creation_date: None,
                    encoding: None,
                    info: empty_info(),
                };

                torrent.announce = match m.get(&ByteString::from_str("announce")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                torrent.announce_list = match m.get(&ByteString::from_str("announce-list")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                torrent.comment = match m.get(&ByteString::from_str("comment")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                torrent.created_by = match m.get(&ByteString::from_str("created by")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                torrent.creation_date = match m.get(&ByteString::from_str("creation date")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                torrent.encoding = match m.get(&ByteString::from_str("encoding")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                let info = match m.get(&ByteString::from_str("info")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                if let Some(info) = info {
                    torrent.info = info;
                }

                Ok(torrent)
            }
            _ => Err(NotADict),
        }
    }
}

impl ToBencode for Torrent {
    fn to_bencode(&self) -> bencode::Bencode {
        let mut m = BTreeMap::new();

        if let &Some(ref v) = &self.announce {
            m.insert(ByteString::from_str("announce"), v.to_bencode());
        }
        if let &Some(ref v) = &self.announce_list {
            m.insert(ByteString::from_str("announce-list"), v.to_bencode());
        }
        if let &Some(ref v) = &self.comment {
            m.insert(ByteString::from_str("comment"), v.to_bencode());
        }
        if let &Some(ref v) = &self.created_by {
            m.insert(ByteString::from_str("created by"), v.to_bencode());
        }
        if let &Some(ref v) = &self.creation_date{
            m.insert(ByteString::from_str("creation date"), v.to_bencode());
        }
        if let &Some(ref v) = &self.encoding{
            m.insert(ByteString::from_str("encoding"), v.to_bencode());
        }
        m.insert(ByteString::from_str("info"), self.info.to_bencode());
        Bencode::Dict(m)
    }
}

impl Torrent {
    pub fn from_buf(buf: &[u8]) -> Result<Self, TorrentError> {
        let bencode: Bencode = bencode::from_buffer(buf).unwrap();

        let result = FromBencode::from_bencode(&bencode);
        result
    }

    pub fn files(&self) -> Option<&Vec<File>> {
        match self.info().files {
                Some(ref f) => Some(f),
                None => None,
        }
    }

    pub fn num_files(&self) -> usize {
        match self.files() {
            Some(f) => f.len(),
            None => 1,
        }
    }

    pub fn total_size(&self) -> u64 {
        let mut total_size = 0;

        if let Some(files) = self.files() {
            for file in files {
                total_size += file.length;
            }
        }

        total_size
    }

    pub fn info_hash(&self) -> Vec<u8> {
        let bencode_info = self.info.to_bencode().to_bytes().unwrap();

        let info_hash: Vec<u8> = Sha1::digest(&bencode_info).to_vec();
        info_hash
    }

    pub fn info(&self) -> &Info {
        &self.info
    }
}

type Piece = Vec<u8>;

#[derive(PartialEq, Debug)]
pub struct Info {
    pub files: Option<Vec<File>>,
    pub length: Option<u64>,
    pub name: Option<String>,
    pub piece_length: i32,
    pub pieces: Vec<Piece>,
    pub private: u8,
}

impl FromBencode for Info {
    type Err = TorrentError;

    fn from_bencode(bencode: &Bencode) -> Result<Self, <Self as FromBencode>::Err> {
        use TorrentError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let mut info = empty_info();

                info.files = match m.get(&ByteString::from_str("files")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                info.length = match m.get(&ByteString::from_str("length")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                info.name = match m.get(&ByteString::from_str("name")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => None,
                };
                info.piece_length = match m.get(&ByteString::from_str("piece length")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => 0,
                };
                info.pieces = match m.get(&ByteString::from_str("pieces")) {
                    Some(a) => match a {
                        &Bencode::ByteString(ref p) => {
                            let mut pieces: Vec<Piece> = Vec::with_capacity(p.len() / 20);
                            let mut p2 = p.to_owned();

                            while p2.len() > 0 {
                                let piece = p2.drain(..20).collect();
                                pieces.push(piece);
                            }

                            pieces
                        }
                        _ => vec![],
                    },
                    None => vec![],
                };
                info.private = match m.get(&ByteString::from_str("private")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => 0,
                };

                Ok(info)
            }
            _ => Err(NotADict),
        }
    }
}

impl ToBencode for Info {
    fn to_bencode(&self) -> bencode::Bencode {
        let mut m = BTreeMap::new();

        if let &Some(ref v) = &self.files {
            m.insert(ByteString::from_str("files"), v.to_bencode());
        }
        if let &Some(ref v) = &self.length {
            m.insert(ByteString::from_str("length"), v.to_bencode());
        }
        if let &Some(ref v) = &self.name {
            m.insert(ByteString::from_str("name"), v.to_bencode());
        }
        m.insert(ByteString::from_str("piece length"), self.piece_length.to_bencode());
        let mut pieces: Vec<u8> = vec![];
        for piece in self.pieces.clone().iter_mut() {
            pieces.append(piece);
        }
        let pieces = Bencode::ByteString(pieces);
        m.insert(ByteString::from_str("pieces"), pieces);
        m.insert(ByteString::from_str("private"), self.private.to_bencode());
        Bencode::Dict(m)
    }
}

#[derive(PartialEq, Debug)]
pub struct File {
    pub length: u64,
    pub path: Vec<String>,
}

impl FromBencode for File {
    type Err = TorrentError;

    fn from_bencode(bencode: &Bencode) -> Result<Self, <Self as FromBencode>::Err> {
        use TorrentError::*;
        match bencode {
            &Bencode::Dict(ref m) => {
                let mut file = File {
                    length: 0,
                    path: vec![],
                };

                file.length = match m.get(&ByteString::from_str("length")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => 0,
                };
                file.path = match m.get(&ByteString::from_str("path")) {
                    Some(a) => FromBencode::from_bencode(a).unwrap(),
                    None => vec![],
                };

                Ok(file)
            }
            _ => Err(NotADict),
        }
    }
}

impl ToBencode for File {
    fn to_bencode(&self) -> bencode::Bencode {
        let mut m = BTreeMap::new();

        m.insert(ByteString::from_str("length"), self.length.to_bencode());
        m.insert(ByteString::from_str("path"), self.path.to_bencode());
        Bencode::Dict(m)
    }
}

fn empty_info() -> Info {
    Info {
        length: None,
        files: None,
        name: None,
        pieces: vec![],
        private: 0,
        piece_length: 0,
    }
}
