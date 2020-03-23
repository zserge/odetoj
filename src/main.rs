use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Array {
    boxed: bool,
    rank: i64,
    depth: [i64; 3],
    data: Vec<Element>,
}

#[derive(Debug, Clone)]
enum Element {
    Array(Array),
    Number(i64),
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Variable(String),
    Verb(char),
}

fn tr(r: i64, d: &[i64]) -> i64 {
    let mut z = 1;
    (0..r).for_each(|i| {
        z = z * d[i as usize];
    });
    z
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (0..self.rank).for_each(|i| {
            write!(f, "{} ", self.depth[i as usize]);
        });
        write!(f, "\n")?;
        let n = tr(self.rank, &self.depth);
        (0..n).for_each(|i| {
            match &self.data[i as usize] {
                Element::Array(arr) => write!(f, "< {} ", arr),
                Element::Number(n) => write!(f, "{} ", n),
            };
        });
        write!(f, "\n")?;
        Ok(())
    }
}

fn array_from_i64(n: i64) -> Array {
    Array {
        boxed: false,
        rank: 0,
        depth: [0, 0, 0],
        data: vec![Element::Number(n)],
    }
}

fn id(a: Array) -> Array {
    a
}

fn size(a: Array) -> Array {
    array_from_i64(if a.boxed { a.depth[0] } else { 1 })
}

fn iota(a: Array) -> Array {
    if let Element::Number(n) = a.data[0] {
        Array {
            boxed: false,
            rank: 1,
            depth: [n, 0, 0],
            data: (0..n).map(|i| Element::Number(i)).collect(),
        }
    } else {
        array_from_i64(0)
    }
}

fn boxing(a: Array) -> Array {
    Array {
        boxed: true,
        rank: 0,
        depth: [0, 0, 0],
        data: vec![Element::Array(a)],
    }
}

fn sha(a: Array) -> Array {
    Array {
        boxed: false,
        rank: 1,
        depth: [a.rank, 0, 0],
        data: (0..a.rank)
            .map(|i| Element::Number(a.depth[i as usize]))
            .collect(),
    }
}

fn at(a: &Array, i: i64) -> i64 {
    if (i as usize) < a.data.len() {
        if let Element::Number(n) = a.data[i as usize] {
            return n;
        }
    }
    return 0;
}

fn plus(a: Array, b: Array) -> Array {
    Array {
        boxed: false,
        rank: b.rank,
        depth: b.depth,
        data: (0..tr(b.rank, &b.depth))
            .map(|i| Element::Number(at(&a, i) + at(&b, i)))
            .collect(),
    }
}

fn from(a: Array, b: Array) -> Array {
    let n = tr(b.rank - 1, &[b.depth[1], b.depth[2], 0]);
    Array {
        boxed: b.boxed,
        rank: b.rank - 1,
        depth: [b.depth[1], b.depth[2], 0],
        data: (0..n)
            .map(|i| b.data[(i + n * at(&a, 0)) as usize].clone())
            .collect(),
    }
}

fn rsh(a: Array, b: Array) -> Array {
    let n = if a.rank == 0 {
        at(&a, 0)
    } else {
        let depth: Vec<i64> = (0..a.depth[0]).map(|i| at(&a, i)).collect();
        tr(a.depth[0], &depth)
    };
    Array {
        boxed: b.boxed,
        rank: if a.rank == 0 { 1 } else { a.depth[0] },
        depth: [at(&a, 0), at(&a, 1), at(&a, 2)],
        data: b
            .data
            .iter()
            .cycle()
            .take(n as usize)
            .map(|x| x.clone())
            .collect(),
    }
}

fn cat(a: Array, b: Array) -> Array {
    let an = tr(a.rank, &a.depth);
    let bn = tr(b.rank, &b.depth);
    let n = an + bn;
    Array {
        boxed: b.boxed,
        rank: 1,
        depth: [n as i64, 0, 0],
        data: (0..n)
            .map(|i| {
                if i < an {
                    a.data[i as usize].clone()
                } else {
                    b.data[(i - an) as usize].clone()
                }
            })
            .collect(),
    }
}

fn eval(tokens: &[Token], env: &mut HashMap<String, Array>) -> Result<Array, String> {
    if let Some((head, tail)) = tokens.split_first() {
        let a: Array = if let Token::Variable(var) = head {
            if let Some((Token::Verb('='), expr)) = tail.split_first() {
                let val = eval(expr, env)?;
                env.insert(var.to_string(), val.clone());
                return Ok(val);
            }
            env.entry(var.to_string())
                .or_insert(array_from_i64(0))
                .clone()
        } else if let Token::Number(num) = head {
            array_from_i64(*num)
        } else {
            array_from_i64(0)
        };

        if let Token::Verb(verb) = head {
            let x = eval(tail, env)?;
            match verb {
                '+' => Ok(id(x)),
                '{' => Ok(size(x)),
                '~' => Ok(iota(x)),
                '<' => Ok(boxing(x)),
                '#' => Ok(sha(x)),
                _ => return Err(format!("unknown monadic verb: {}", verb)),
            }
        } else if let Some((Token::Verb(verb), expr)) = tail.split_first() {
            let b = eval(expr, env)?;
            match verb {
                '+' => Ok(plus(a, b)),
                '{' => Ok(from(a, b)),
                '#' => Ok(rsh(a, b)),
                ',' => Ok(cat(a, b)),
                _ => return Err(format!("unknown dyadic verb: {}", verb)),
            }
        } else {
            Ok(a)
        }
    } else {
        Ok(array_from_i64(0))
    }
}

fn parse(s: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut it = s.chars().peekable();
    while let Some(&c) = it.peek() {
        let mut lex = |f: fn(char) -> bool| {
            let mut s = String::from("");
            while let Some(&x) = it.peek() {
                if !f(x) {
                    break;
                }
                s.push(it.by_ref().next().unwrap())
            }
            return s;
        };
        match c {
            '0'...'9' => {
                result.push(Token::Number(
                    lex(|c| c >= '0' && c <= '9').parse::<i64>().unwrap(),
                ));
            }
            'a'...'z' => result.push(Token::Variable(lex(|c| c >= 'a' && c <= 'z'))),
            '+' | '{' | '~' | '<' | '#' | ',' | '=' => result.push(Token::Verb(it.next().unwrap())),
            _ => return Err(format!("unexpected {}", &c)),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mut env: HashMap<String, Array> = HashMap::new();
        // Atoms
        println!("{}", eval(&parse("").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("1").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("123").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("abc").unwrap(), &mut env).unwrap());
        // Monads
        println!("{}", eval(&parse("+10").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("{10").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("<10").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("~10").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("#10").unwrap(), &mut env).unwrap());
        // Dyads
        println!("{}", eval(&parse("1+2").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("1,2,3").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("1{5,7,9").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("5#3,4").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("shp=2,3").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("shp#~10").unwrap(), &mut env).unwrap());
        // Variables
        println!("{}", eval(&parse("a=3").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("b=4").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("d=1+c=a+b").unwrap(), &mut env).unwrap());
        println!("{}", eval(&parse("d+c").unwrap(), &mut env).unwrap());
    }

    #[test]
    fn test_parser() {
        assert_eq!(parse(""), Ok(vec![]));
        assert_eq!(parse("a"), Ok(vec![Token::Variable("a".to_string())]));
        assert_eq!(parse("abc"), Ok(vec![Token::Variable("abc".to_string())]));
        assert_eq!(parse("1"), Ok(vec![Token::Number(1)]));
        assert_eq!(parse("123"), Ok(vec![Token::Number(123)]));
        assert_eq!(
            parse("1+2"),
            Ok(vec![Token::Number(1), Token::Verb('+'), Token::Number(2)])
        );
        assert!(parse("1.2").is_err());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut env: HashMap<String, Array> = HashMap::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", eval(&parse(line?.as_str())?, &mut env)?);
    }
    Ok(())
}
