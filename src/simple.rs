use super::get_file;

pub fn diff(path1: &str, path2: &str) -> Result<(), Box<(String, String)>> {
    let s1 = get_file(path1).unwrap();
    let s2 = get_file(path2).unwrap();
    if *s1 == *s2 {
        Ok(())
    } else {
        Err(Box::new((*s1, *s2)))
    }
}

pub fn print(ok: &str, ng: &str) {
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let end = "\x1b[0m";
    println!("{}{}{}", green, ok, end);
    println!("{}{}{}", red, ng, end);
}


#[cfg(test)]
#[test]
fn simple_diff_ok() {
    let b = diff("./testdata/ok.txt", "./testdata/ok.txt");
    assert_eq!(b, Ok(()));
}
#[test]
fn simple_diff_cr() {
    let b = diff("./testdata/ok.txt", "./testdata/ok_cr.txt");
    assert_eq!(b, Ok(()));
}
#[test]
fn simple_diff_ng_typo() {
    let b = diff("./testdata/ok.txt", "./testdata/ng_typo.txt");
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
