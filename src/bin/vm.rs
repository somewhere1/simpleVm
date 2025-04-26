use simpleVM::{Machine,Register,SignalFunction};
use std::env;
use std::io::{BufReader,Read};
use std::fs::File;
use std::path::Path;

fn signal_halt(vm:&mut Machine)-> Result<(),String>{

    vm.halt = true;
    Ok(())
}

pub fn main() ->Result<(),String>{

    let args:Vec<_> = env::args().collect();
    if args.len()!=2{

        panic!("usage: {} <input>",args[0]);

    }
    
    let file = File::open(Path::new(&args[1])).map_err(|x|format!("failed to open {}",x))?;

    let mut reader = BufReader::new(file);
    let mut program:Vec<u8> = Vec::new();
    reader.read_to_end(&mut program).map_err(|e| format!("read {}",e))?;
    
    let mut vm = Machine::new();
<<<<<<< HEAD
    vm.set_sp(Register::SP,0x1000);
=======
>>>>>>> 30465c277c223d488f17568f5accd4c2f8bf0edd
    vm.define_handler(0xf0,signal_halt);
    vm.memory.load_from_vec(&program,0);
    while !vm.halt{
        vm.step()?;
<<<<<<< HEAD
        println!("{}",vm.state());
    }
    println!("A  = {}",vm.get_registers(Register::A));
=======
    }
    println!("A  = {}",vm.get_register(Register::A));
>>>>>>> 30465c277c223d488f17568f5accd4c2f8bf0edd
    Ok(())

}
