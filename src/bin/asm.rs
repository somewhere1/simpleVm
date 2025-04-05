use std::{env,path::Path};
use std::io;
use std::fs::File;
use std::io::{BufReader,BufRead,Write};

fn main() -> Result<(),String>{

    // ./asm file.asm
    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        panic!("usages: {} <input>",args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed  to open: {}",e))?;

    let mut output:Vec<u8> = vec![];
    /*for line in io::BufReader::new(file).lines(){
        let line_inner = line.map_err(|_x| "foo")?;
        for t in line_inner.split(" ").filter(|x| x.len()!=0){
            let b = u8::from_str_radix(t,16).map_err(|x| format!("parse int:{}",x))?;
            output.push(b);

        }
    }*/
    // Vec<Result<u8,String>> => Result<Vec<u8>,String>
    let r: Result<Vec<u8>,String> = BufReader::new(file)
        .lines()
        .map(|line|{ 
             line
                .map_err(|e| format!("foo {}",e))?
                .split(" ")
                .filter(|x| x.len()!=0 ) 
                .map(|t| u8::from_str_radix(t,16).map_err(|e| format!("parse int:{}",e)))
                .collect::<Result<Vec<u8>, String>>()

                })
          .collect::<Result<Vec<Vec<u8>>,String>>()
          .map(|vecs| vecs.into_iter().flatten().collect());
    

    let mut stdout = io::stdout().lock();
    stdout.write_all(&(r?)).map_err(|x| format!("{}",x))?;
    Ok(())
}
