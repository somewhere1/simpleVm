use crate::memory::{LinearMemory,Addressable};
use std::collections::HashMap;

//寄存器元组
#[derive(Debug,Clone,Copy)]
#[repr(u8)]
pub enum Register{
    A,B,C,M,SP,PC,BP,FLAGS,
}

impl Register{

    pub fn from_u8(v: u8) -> Option<Self>{
        match v {
            x if x==Register::A  as u8 => Some(Register::A),
            x if x==Register::B as u8 => Some(Register::B),
            x if x==Register::C as u8 => Some(Register::C),
            x if x==Register::M as u8 => Some(Register::M),
            x if x==Register::SP as u8 => Some(Register::BP),
            x if x==Register::PC as u8 => Some(Register::PC),
            x if x==Register::BP as u8 => Some(Register::BP),
            x if x==Register::FLAGS as u8 => Some(Register::FLAGS),
            _ => None
            
        }
    }
}

#[derive(Debug)]
pub enum Instruction{
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register,Register),
    Signal(u8),

}


//convert Instruction to operation Code
impl Instruction{

    pub fn encode_r1(r:Register) -> u16{
      (r as u16)&0xf << 8
    }
    
    pub fn encode_r2(r:Register) -> u16{
        (r as  u16)&0xf << 12

    }
    pub fn encode_num(u:u8) -> u16 {
        (u as u16) << 8
    }

    pub fn encode_rs(r1:Register,r2:Register) -> u16{

        Self::encode_r1(r1)| Self::encode_r2(r2)
    }
    pub fn encode_u16(&self) -> u16 {
        match self  {

            Self::Nop => OpCode::Nop as u16,
            Self::Push(x) => OpCode::Push as u16  |  Self::encode_num(*x),
            Self::PopRegister(r) => OpCode::PopRegister as u16 | Self::encode_r1(*r),
            Self::AddStack => OpCode::AddStack as u16,
            Self::AddRegister(r1,r2) => OpCode::AddRegister as u16 | Self::encode_rs(*r1,*r2),
            Self::Signal(x) => OpCode::Signal as u16 | Self::encode_num(*x) 
            
        }
    }

}


#[repr(u8)]
#[derive(Debug)]
pub enum OpCode{

    Nop=0x0,
    Push = 0x1,
    PopRegister = 0x2,
    Signal = 0x0f,
    AddStack = 0x10,
    AddRegister = 0x11,


}





impl OpCode{
    pub fn value(&self) -> u8 {
        unsafe{ *<*const _>::from(self).cast::<u8>()}
    }
    
    pub fn from_str(s:&str) -> Option<Self>{

        match s {

            "Nop" => Some(Self::Nop),
            "Push" => Some(Self::Push),
            "PopRegister"=> Some(Self::PopRegister),
            "Signal" => Some(Self::Signal),
            "AddStack" => Some(Self::AddStack),
            "AddRegister" => Some(Self::AddRegister),
            _ => None
        }
    }


    pub fn from_u8(b:u8) -> Option<Self>{
        match b {
        x if x == Self::Nop  as u8 => Some(Self::Nop),
        x if x == Self::Push as u8 => Some(Self::Push),
        x if x == Self::PopRegister as u8  => Some(Self::PopRegister),
        x if x == Self::Signal as u8  => Some(Self::Signal),
        x if x == Self::AddStack as u8  => Some(Self::AddStack),
        x if x == Self::AddRegister as u8 => Some(Self::AddRegister),
        _ => None


        }
    }
}

fn parse_instruction_arg(ins:u16) -> u8{
    ((ins&0xff00) >> 8) as u8
}


fn parse_instruction(ins: u16) -> Result<Instruction,String>{

    let op =(ins & 0xff) as u8;
    match OpCode::from_u8(op).ok_or(format!("unknown op: {:X}",op))? {
        OpCode::Nop => Ok(Instruction::Nop),
        OpCode::Push =>{
            //取出进栈的数据
            let arg = (ins & 0xff00) >> 8;
            Ok(Instruction::Push(arg as u8))

        },
        OpCode::PopRegister => {
            let reg = (ins & 0xff00) >> 8;
            Register::from_u8(reg as u8)
                .ok_or(format!("unknown register 0x{:X}",reg))
                .map(|r| Instruction::PopRegister(r))
                    
        },
        OpCode::AddStack => {
            Ok(Instruction::AddStack)
        },
        OpCode::AddRegister => {
            let reg1_raw = (ins&0xf00) >> 8;
            let reg2_raw = (ins&0xf000) >> 12 ;
            let reg1 = Register::from_u8(reg1_raw as u8).
                ok_or(format!("unknown register 0x{:X}",reg1_raw))?;
            let reg2 = Register::from_u8( reg2_raw as u8).
                ok_or(format!("unknow register 0x{:X}",reg2_raw))?;
            Ok(Instruction::AddRegister(reg1,reg2))
        },

        OpCode::Signal =>{
            let arg = parse_instruction_arg(ins);
            Ok(Instruction::Signal(arg))
        }
        _ => Err(format!("Unknown op 0x{:X}",op))
    }
}
//


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

    pub fn get_register(&self,r:Register) -> u16{
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
        //Instruction  = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 ]
        //                 operation       |ARG(s)
        //                                 |8 bit literal
        //                                 |REG1 | REG2 
        let  op = parse_instruction(instruction)?;
        match op{
            Instruction::Nop => Ok(()),
            Instruction::Push(v) => {
                self.push(v.into())
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
