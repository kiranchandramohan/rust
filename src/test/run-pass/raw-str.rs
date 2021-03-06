// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-tidy-tab
// xfail-tidy-linelength

pub fn main() {
    assert_eq!(r"abc", "abc");

    assert_eq!(r#"abc"#, "abc");

    assert_eq!(r"###", "###");

    assert_eq!(r"\", "\\");

    assert_eq!(r#"\""#, "\\\"");

    assert_eq!(r#"#"\n""#, "#\"\\n\"");

    assert_eq!(r##"a"#"b"##, "a\"#\"b");

    // from rust.vim
    assert_eq!(r#""%\(\d\+\$\)\=[-+' #0*]*\(\d*\|\*\|\*\d\+\$\)\(\.\(\d*\|\*\|\*\d\+\$\)\)\=\([hlLjzt]\|ll\|hh\)\=\([aAbdiuoxXDOUfFeEgGcCsSpn?]\|\[\^\=.[^]]*\]\)""#,
               "\"%\\(\\d\\+\\$\\)\\=[-+' #0*]*\\(\\d*\\|\\*\\|\\*\\d\\+\\$\\)\\(\\.\\(\\d*\\|\\*\\|\\*\\d\\+\\$\\)\\)\\=\\([hlLjzt]\\|ll\\|hh\\)\\=\\([aAbdiuoxXDOUfFeEgGcCsSpn?]\\|\\[\\^\\=.[^]]*\\]\\)\"");

    assert_eq!(r"newline:'
', tab:'	', unicode:'●', null:' '",
        "newline:'\n', tab:'\t', unicode:'\u25cf', null:'\0'");

    assert_eq!(format!(r"\\\{\}"), format!("\\\\\\{\\}"));
}
