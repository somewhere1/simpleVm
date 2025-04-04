
pub trait Addressable{

    fn read(&self,addr:u16) ->Option<u8>;
    fn write(&mut self,addr:u16,value:u8) -> bool;
    fn read2(&self,addr:u16) -> Option<u16>{
        if let Some(x0) = self.read(addr){
            if let Some(x1) =self.read(addr + 1){
                //小段方式：低位数据放在低地址为 高位数据放在高位置位
                return Some((x0 as u16) |((x1 as u16) << 8))
            }
        };
        None
    }

    fn write2(&mut self,addr:u16,value:u16) -> bool{
        let lower = (value & 0xff) as u8;
        let upper = ((value & 0xff00) >> 8) as u8;

        self.write(addr,lower) && self.write(addr+1,upper)

    }
    
    fn copy(&mut self,from:u16,to:u16,n:usize) -> bool{

        for i in 0..(n as u16){
            if let Some(x) = self.read(from+i){
                if !self.write(to+i,x){
                    return false;
                }
            }
            else{
                    return false;
            }
        }

        true
    }
   
    fn load_from_vec(&mut self,from:&[u8],addr:u16) -> bool {
        for (i,b) in from.iter().enumerate(){
            
            if !self.write(addr+(i as u16),*b){

                return false;
            }   
        };
        true

    }
}

pub struct LinearMemory{
    bytes:Vec<u8>,
    size:usize

}

impl LinearMemory{
    pub fn new(n:usize)->Self{

        Self{
            bytes:vec![0;n],
            size:n,
        }
    }
}
impl Addressable for LinearMemory{
    fn read(&self,addr:u16) ->Option<u8>{
        if (addr as usize) < self.size{
            Some(self.bytes[addr as usize])
        }
        else{
            None
        }
    }
    fn write(&mut self,addr:u16,value:u8) -> bool{
        if (addr as usize) < self.size{
            self.bytes[addr as usize] = value;
            true
        }
        else{
            false
        }
    }
}

