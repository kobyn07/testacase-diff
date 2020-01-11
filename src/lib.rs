use std::fs::File;
use std::io;
use std::io::Read;

static mut EPS: f64 = 1e-6;

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

pub fn print_simple_diff(ok: &str, ng: &str) {
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let end = "\x1b[0m";
    println!("{}{}{}", green, ok, end);
    println!("{}{}{}", red, ng, end);
}
enum Val {
    Int(i64),
    Float(f64),
    Word(String),
}

impl Val {
    fn to_string(&self) -> String {
        match &*self {
            Val::Int(val) => val.to_string(),
            Val::Float(val) => val.to_string(),
            Val::Word(val) => val.to_string(),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match &*self {
            Val::Int(val) => match other {
                Val::Int(ot) => val == ot,
                Val::Float(ot) => {
                    let f = *val as f64;
                    let diff = f - ot;
                    unsafe { diff.abs() <= EPS }
                }
                Val::Word(_) => false,
            },
            Val::Float(val) => match other {
                Val::Int(ot) => {
                    let f = *ot as f64;
                    let diff = val - f;
                    unsafe { diff.abs() <= EPS }
                }
                Val::Float(ot) => {
                    let diff = val - ot;
                    unsafe { diff.abs() <= EPS }
                }
                Val::Word(_) => false,
            },
            Val::Word(val) => match other {
                Val::Int(_) => false,
                Val::Float(_) => false,
                Val::Word(ot) => val == ot,
            },
        }
    }
}

pub fn diff(original: &str, output: &str) -> Result<(), Box<Vec<Vec<String>>>> {
    let convert_val = |input: &str| -> Box<Vec<Vec<Val>>> {
        let mut ret: Vec<Vec<Val>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<Val> = Vec::new();
            for word in line.split_whitespace() {
                let val = {
                    match word.parse::<i64>() {
                        Ok(x) => Val::Int(x),
                        Err(_) => match word.parse::<f64>() {
                            Ok(x) => Val::Float(x),
                            Err(_) => Val::Word(word.parse::<String>().unwrap()),
                        },
                    }
                };
                row.push(val);
            }
            ret.push(row);
        }
        Box::new(ret)
    };

    let original = *get_file(original).unwrap();
    let original = *convert_val(&original);
    let output = *get_file(output).unwrap();
    let output = *convert_val(&output);

    if original == output {
        Ok(())
    } else {
        let mut ret: Vec<Vec<String>> = Vec::new();
        for (row, line) in original.iter().enumerate() {
            let mut correct: Vec<String> = Vec::new();
            let mut wrong: Vec<String> = Vec::new();
            for (col, val) in line.iter().enumerate() {
                let cmp = output.get(row).expect(" ").get(col).expect(" ");
                if val == cmp {
                    let len = val.to_string().len();
                    wrong.push(" ".to_string().repeat(len));
                } else {
                    wrong.push(val.to_string());
                }
                correct.push(val.to_string());
            }
            ret.push(correct);
            ret.push(wrong);
        }

        Err(Box::new(ret))
    }
}

pub fn print_diff(output: &Vec<Vec<String>>) {
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let end = "\x1b[0m";

    let is_whitespace = |text: &str| -> bool {
        for c in text.chars() {
            if c.is_whitespace() {
                return true;
            }
        }
        false
    };

    for (i, row) in output.iter().enumerate() {
        if i % 2 == 0 {
            for correct in row {
                print!("{}{}{} ", green, correct, end);
            }
            println!();
        } else {
            let mut is_show = false;
            for wrong in row {
                if !is_whitespace(wrong) {
                    is_show = true;
                }
            }
            if is_show{
                for wrong in row{
                    print!("{}{}{} ", red, wrong, end);
                }
                println!();
            }
        }
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
#[test]
fn diff_ok() {
    let x = diff("./testdata/ok.txt", "./testdata/ok_copy.txt");
    assert_eq!(x, Ok(()));
}
#[test]
fn diff_cr_ok() {
    let x = diff("./testdata/ok.txt", "./testdata/ok_cr.txt");
    assert_eq!(x, Ok(()));
}

#[test]
fn diff_ng_typo() {
    let x = *diff("./testdata/ok.txt", "./testdata/ng_typo.txt").unwrap_err();
    let y: Vec<Vec<String>> = vec![
        vec!["hello".to_string(), "world".to_string()],
        vec!["     ".to_string(), "world".to_string()],
        vec!["42".to_string()],
        vec!["  ".to_string()],
        vec!["3.14159265359".to_string()],
        vec!["             ".to_string()],
    ];

    assert_eq!(x == y, true);
}

#[test]
fn val_ok_float() {
    let x = Val::Float(3.141592);
    let y = Val::Float(3.14159264);
    assert_eq!(x == y, true);
}
#[test]
fn val_ok_int_float() {
    let x = Val::Int(1);
    let y = Val::Float(1.00000000001);
    assert_eq!(x == y, true);
}
#[test]
fn val_ok_float_int() {
    let x = Val::Float(1.00000000001);
    let y = Val::Int(1);
    assert_eq!(x == y, true);
}

#[test]
fn val_ng_float() {
    let x = 3.0;
    let y = 3.14;
    assert_eq!(x == y, false);
}
