use crate::memory::{LinearMemory,Addressable};
use std::collections::HashMap;
use crate::op::{OpCode,Instruction};
use crate::register::Register;


pub type SignalFunction = fn(&mut Machine) -> Result<(),String>;
//16bit虚拟机 结构体
pub struct Machine{

    registers:[u16;8],
    signal_handlers:HashMap<u8,SignalFunction>,
    pub halt:bool,
    pub memory:Box<dyn Addressable>,

}

//
impl Machine{
    //初始化虚拟机
    pub fn new() -> Self{
        Machine{
            registers:[0;8],
            halt:false,
            signal_handlers:HashMap::new(),
            memory: Box::new(LinearMemory::new(8*1024)),
        }
    }
 
    pub fn state(&self) -> String{

    format!("A: {} | B:{} | C:{} | M:{} | SP:{} | PC:{} | BP:{} | FLAGS:{:X}",
            self.get_registers(Register::A),
            self.get_registers(Register::B),
            self.get_registers(Register::C),
            self.get_registers(Register::M),
            self.get_registers(Register::SP),
            self.get_registers(Register::PC),
            self.get_registers(Register::BP),
            self.get_registers(Register::FLAGS))


    }
    
    pub fn set_sp(&mut self,r:Register,u:u16){
        self.registers[r as usize] = u;
    }

    pub fn get_registers(&self,r:Register) -> u16{
        self.registers[r as usize]
    }

    pub fn define_handler(&mut self,index:u8,f:SignalFunction){
        self.signal_handlers.insert(index,f);
    }

    pub fn pop(&mut self,) -> Result<u16,String>{
        //Pop.we have to read the stack pointer first
        let sp = self.registers[Register::SP as usize] -2;
        if let Some(v) = self.memory.read2(sp){
            //出栈成功 栈顶指针-2
            self.registers[Register::SP as usize] -= 2;
            Ok(v)
        }else{
            Err(format!("memory read fault @  0x{:X}",sp))
        }

    }
    pub fn push(&mut self,v:u16) -> Result<(),String>{
        let  sp = self.registers[Register::SP as usize];
                //data push into the stack,
        if !self.memory.write2(sp,v){
                    return Err(format!("memory write fault @  0x{:X}",sp));
                }                   
                //when data push in the stack,the stack pointer increment 2
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }
    

    pub fn step(&mut self) -> Result<(),String>{
        //枚举索引的隐式转换，会让人疑惑，取出程序计数器
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).ok_or(format!("pc read file @ {:X}",pc))?;
        self.registers[Register::PC as usize ] = pc + 2;
        let  op = Instruction::try_from(instruction)?;
        match op{
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => {
                self.push(v.into())
            },
            Instruction::PushRegister(r) => {
                self.push(self.registers[r as usize])?;
                Ok(())
            },
            Instruction::PopRegister(r) => {
                //返回栈顶的值
                let value = self.pop()?;
                //将对应的值放入寄存器
                self.registers[r as usize] =  value;
                
                Ok(())
            },
            Instruction::AddStack =>{
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a+b)?;
                Ok(())

            },
            Instruction::AddRegister(r1,r2) =>{
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            },
            Instruction::Signal(signal) => {
                let sig_fn = self.signal_handlers
                    .get(&signal).
                    ok_or(format!("unkown signal {:X}",signal))?;
                sig_fn(self)
            },
        }
        //println!("{} & {}",instruction,pc);



    }
}
