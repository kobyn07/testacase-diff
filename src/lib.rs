use std::fs::File;
use std::io;
use std::io::Read;

pub fn simple_diff(path1: &str, path2: &str) -> bool {
    let get_file = |path: &str| -> Result<String, io::Error> {
        let mut ret = String::new();
        File::open(path)?.read_to_string(&mut ret)?;
        ret.retain(|c| c != '\r');
        Ok(ret)
    };

    get_file(path1).unwrap() == get_file(path2).unwrap()
}

#[cfg(test)]
#[test]
fn simple_diff_ok() {
    let b = simple_diff("./testdata/ok.txt", "./testdata/ok.txt");
    assert_eq!(b, true);
}
#[test]
fn simple_diff_cr() {
    let b = simple_diff("./testdata/ok.txt", "./testdata/ok_cr.txt");
    assert_eq!(b, true);
}
#[test]
fn simple_diff_ng_typo() {
    let b = simple_diff("./testdata/ok.txt", "./testdata/ng_typo.txt");
    assert_eq!(b, false);
}
