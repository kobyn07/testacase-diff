use std::fs::File;
use std::io;
use std::io::Read;

fn get_file(path: &str) -> Result<Box<String>, io::Error> {
    let mut ret = String::new();
    File::open(path)?.read_to_string(&mut ret)?;
    ret.retain(|c| c != '\r');
    Ok(Box::new(ret))
}

pub fn simple_diff(path1: &str, path2: &str) -> Result<(), Box<(String, String)>> {
    let s1 = get_file(path1).unwrap();
    let s2 = get_file(path2).unwrap();
    if *s1 == *s2 {
        Ok(())
    } else {
        Err(Box::new((*s1, *s2)))
    }
}



#[cfg(test)]
#[test]
fn simple_diff_ok() {
    let b = simple_diff("./testdata/ok.txt", "./testdata/ok.txt");
    assert_eq!(b, Ok(()));
}
#[test]
fn simple_diff_cr() {
    let b = simple_diff("./testdata/ok.txt", "./testdata/ok_cr.txt");
    assert_eq!(b, Ok(()));
}
#[test]
fn simple_diff_ng_typo() {
    let b = simple_diff("./testdata/ok.txt", "./testdata/ng_typo.txt");
    assert_eq!(
        b,
        Err(Box::new((
            String::from(
                "hello world
42
3.14159265359
"
            ),
            String::from(
                "hello wordl
42
3.14159265359
"
            )
        )))
    );
}
