use crate::memory::{LinearMemory,Addressable};
//寄存器元组
#[derive(Debug)]
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
#[repr(u8)]
#[derive(Debug)]
pub enum Op{
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register,Register),

}
impl Op{
    pub fn value(&self) -> u8 {
        unsafe{ *<*const _>::from(self).cast::<u8>()}
    }
    // pub fn equal(u8,other:Self) -> bool{
    //     x==other.value
    // }
}

fn parse_instruction(ins: u16) -> Result<Op,String>{

    let op =(ins & 0xff) as u8;
    match op{
        x if x==Op::Nop.value() => Ok(Op::Nop),
        x if x==Op::Push(0).value() =>{
            //取出进栈的数据
            let arg = (ins & 0xff00) >> 8;
            Ok(Op::Push(arg as u8))

        },
        x if  x== Op::PopRegister(Register::A).value() => {
            let reg = (ins & 0xff00) >> 8;
            if let Some(r) = Register::from_u8(reg as u8){
                Ok(Op::PopRegister(r))
            }
            else{
                Err(format!("unknown register 0x{:X}",reg))
            }
        },
        x if  x==Op::AddStack.value() => {
            Ok(Op::AddStack)
        }
        _ => Err(format!("Unknown op 0x{:X}",op))
    }
}
//

//16bit虚拟机 结构体
pub struct Machine{

    registers:[u16;8],
    pub memory:Box<dyn Addressable>,

}

//
impl Machine{
    //初始化虚拟机
    pub fn new() -> Self{
        Machine{
            registers:[0;8],
            memory: Box::new(LinearMemory::new(8*1024)),
        }
    }

    pub fn get_register(&self,r:Register) -> u16{
        self.registers[r as usize]
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
        let instruction = self.memory.read2(pc).unwrap();
        self.registers[Register::PC as usize ] = pc + 2;
        //Instruction  = [ 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 ]
        //                 operation       |ARG(s)
        //                                 |8 bit literal
        //                                 |REG1 | REG2 
        let  op = parse_instruction(instruction)?;
        match op{
            Op::Nop => Ok(()),
            Op::Push(v) => {
                self.push(v.into())
            },
            Op::PopRegister(r) => {
                //返回栈顶的值
                let value = self.pop()?;
                //将对应的值放入寄存器
                self.registers[r as usize] =  value;
                
                Ok(())
            },
            Op::AddStack =>{
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a+b)?;
                Ok(())

            },
            Op::AddRegister(r1,r2) =>{
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
        }
        //println!("{} & {}",instruction,pc);



    }
}
