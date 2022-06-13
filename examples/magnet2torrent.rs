// Copyright (c) 2022 Nicolas Chevalier
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use libtorrent_sys::ffi::*;

use std::io::{self, Write};
use std::{thread, time::Duration};
use std::fs::File;

fn main() {
    let uri = std::env::args().nth(1).expect("no pattern given");
    let mut session = lt_create_session();
    let mut torrent_param = lt_parse_magnet_uri(&uri, ".");
    let hdl = lt_session_add_torrent(session.pin_mut(), torrent_param.pin_mut());

    loop {
	if lt_torrent_has_metadata(&hdl) {
	    lt_session_pause(session.pin_mut());
	    let torrent_name = lt_torrent_get_name(&hdl);
	    println!("\ncreate file: {}.torrent", torrent_name);
	    io::stdout().flush().unwrap();

	    let bin = lt_torrent_bencode(&hdl);
	    let mut ofile = File::create(format!("{}.torrent", torrent_name)).expect("unable to create file");
	    ofile.write_all(&bin).expect("unable to write");

	    lt_session_remove_torrent(session.pin_mut(), &hdl);
	    break;
	}

	print!(".");
	io::stdout().flush().unwrap();
	thread::sleep(Duration::from_millis(1000));
    }
}
