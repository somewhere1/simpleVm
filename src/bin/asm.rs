use std::{env,path::Path};
use std::io;
use std::fs::File;
use std::io::{BufReader,BufRead,Write};

use simpleVM::{Instruction,OpCode,Register};

fn parse_numeric(s:&str) -> Result<u8,String>{
        if s.len() == 0{
            return Err("string has no length".to_string());
        }

        let fst = s.chars().nth(0).unwrap();
        let (num,radix) = match fst {
            '$' => (&s[1..],16),
            '%' => (&s[1..],2),
            _ => (s,10)

        };
        u8::from_str_radix(num,radix).map_err(|x| format!("{}",x))

}


fn parse_register(s:&str) -> Result<Register,String>{

        match s {
            "A" => Ok(Register::A),
            _ => Err(format!("unknown register {}",s))

        }

}

fn  assert_length(parts: &Vec<&str>,n:usize) -> Result<(),String>{
    
    if parts.len() == n{
        Ok(())
    }
    else{
        Err(format!("Expected {} got {}",parts.len(),n))
    }
}

fn handle_line(parts:&Vec<&str>) -> Result<Instruction,String>{
    
        let opcode = OpCode::from_str(parts[0]).ok_or(format!("unknown opcode: {}",parts[0]))?;
        match opcode {
            OpCode::Push => {
                assert_length(parts,2)?;
                Ok(Instruction::Push(parse_numeric(parts[1])?))
            },

            OpCode::AddStack => {
                assert_length(parts,1)?;
                Ok(Instruction::AddStack)
            },
<<<<<<< HEAD
            OpCode::PushRegister => {
                assert_length(parts,2)?;
                Ok(Instruction::PushRegister(parse_register(parts[1])?))
            },
=======
>>>>>>> 30465c277c223d488f17568f5accd4c2f8bf0edd
            OpCode::PopRegister => {
                assert_length(parts,2)?;
                Ok(Instruction::PopRegister(parse_register(parts[1])?))
            },

<<<<<<< HEAD

            OpCode::AddRegister=> {
                assert_length(parts,3)?;
                Ok(Instruction::AddRegister(parse_register(parts[1])?,parse_register(parts[2])?))
            },

            OpCode::Nop => {
=======
           /* OpCode::AddRegister=> {
                assert_length(parts,3)?;
                Ok(Instruction::AddRegister(parse_register(parts[1])?,parse_register(parts[2])?))
            },*/

            OpCode::Nop => {
                assert_length(parts,1)?;
>>>>>>> 30465c277c223d488f17568f5accd4c2f8bf0edd
                Ok(Instruction::Nop)
            },

            OpCode::Signal => {
                assert_length(parts,2)?;
                Ok(Instruction::Signal(parse_numeric(parts[1])?))
            },


            _ => Err(format!("unimplemented opcode: {:?}",opcode))
        }


}


fn main() -> Result<(),String>{

    // ./asm file.asm
    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        panic!("usages: {} <input>",args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed  to open: {}",e))?;

    let mut output:Vec<u8> = vec![];
    for line in io::BufReader::new(file).lines(){
        let line_inner = line.map_err(|_x| "foo")?;
        if line_inner.len() == 0 {
            continue;
        }
        if line_inner.chars().nth(0).unwrap() == ';'{
            continue;
            }   
        let parts: Vec<_> =  line_inner.split(" ").filter(|x| x.len() > 0).collect();
        if parts.len()==0{
            continue;
        }
      
        let instruction = handle_line(&parts)?;
        let raw_instruction:u16 = instruction.encode_u16();
        
        // assumption: >>8 nedds mask for u16
        // low 8bit
        output.push((raw_instruction&0xff) as u8);
        //high 8bit
        output.push((raw_instruction>>8) as u8);

    }
    // Vec<Result<u8,String>> => Result<Vec<u8>,String>
   /* let r: Result<Vec<u8>,String> = BufReader::new(file)
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
    */

    let mut stdout = io::stdout().lock();
   // stdout.write_all(&(r?)).map_err(|x| format!("{}",x))?;
    stdout.write_all(&output).map_err(|x| format!("{}",x))?;
    Ok(())
}
