#![deny(unsafe_code)]
#![no_std]
#[allow(unused_imports)]
// use f3::hal::delay::Delay;
// use f3::hal::stm32f30x::i2c1;
use panic_itm;
use f3::hal::{delay::Delay,stm32f30x::i2c1};
use f3::hal::prelude::*;

// Create Struct For LCD 
pub struct LiquidCrystalI2c <'a,'b>{
    add : u8,
    cols : u8,
    rows : u8,
    charsize: u8,
    backlightval:u8,
    // convert to pub for futher use
    pub delay:&'b mut Delay,
    i2c1:&'a i2c1::RegisterBlock
}

impl <'a,'b> LiquidCrystalI2c <'a,'b>{
// Create for initialize LCD Struct
   pub fn new(add:u8,cols:u8,rows:u8,delay:&'b mut Delay, i2c1:&'a i2c1::RegisterBlock )->LiquidCrystalI2c<'a,'b>{
       
        LiquidCrystalI2c{
            add : add,
            cols : cols,
            rows : rows,
            charsize: 0x00,
            backlightval:0x08,
            delay:delay,
            i2c1:i2c1
        }
    }
}


impl <'a,'b> LiquidCrystalI2c <'a,'b>{
// begin Function to start LCD 
   pub fn begin(&mut self){
        let display_function:u8 = 0x00 | 0x08 | 0x00;
        let displaymode:u8 = 0x02 | 0x00;
        self.delay.delay_ms(50u32);
        self.expander_write(0x08);
        self.delay.delay_ms(1000u32);
        self.write4bits(0x03 << 4);
        self.delay.delay_us(4500u32);
        self.write4bits(0x03 << 4);
        self.delay.delay_us(4500u32);
        self.write4bits(0x03 << 4);
        self.delay.delay_us(150u32);
        self.write4bits(0x02 << 4);
        self.command(0x20| display_function);
        self.display();
        self.clear();
        self.command(0x04 | displaymode);
        self.home();
    }
    // Clear Function to Clear Display
   pub fn clear(&mut self){
        self.command(0x01);
        self.delay.delay_us(2000u32);
    }
    // Go To Home
    pub fn home(&mut self){
        self.command(0x02);
        self.delay.delay_us(2000u32)
    }
    // Display On
   pub fn display(&mut self){
        let displaycontrol:u8 = 0x04 | 0x00 | 0x00;
        self.command(0x08 | displaycontrol)

    }
    // Cursor On
   pub fn cursor(&mut self){
        let displaycontrol:u8 = 0x04 | 0x02 | 0x00;
        self.command(0x08 | displaycontrol)
    }
    // Blink On
   pub fn blinkon(&mut self){
        let displaycontrol:u8 = 0x04 | 0x02 | 0x01;
        self.command(0x08 | displaycontrol)
    }
    // Goto Line One and Selected Column 
   pub fn line1(&mut self,col:u8){
        self.command(0x80 | (col + 0x00))
    }
    // Goto Line Two and Seleted Column
   pub fn line2(&mut self,col:u8){
        self.command(0x80 | (col + 0x40))
    }  
    // Write The Data to Display
    pub fn write(&mut self, value:u8)->u8{
        let rs= 0b00000001;
        self.send(value,rs);
        1
    }
    
}


impl <'a,'b> LiquidCrystalI2c <'a,'b>{
    fn command(&mut self,value:u8){
        self.send(value, 0)
    }
    fn send(&mut self,value:u8,mode:u8){
        let highnib:u8 = value&0xf0;
        let lownib:u8 = (value<<4)&0xf0;
        self.write4bits(highnib|mode);
        self.write4bits(lownib|mode);
    }
    fn write4bits(&mut self,value:u8){
        self.expander_write(value);
	    self.pulse_enable(value);
    }
    fn expander_write(&self,data:u8){
        self.i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd1().bits(self.add);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });
        while self.i2c1.isr.read().txis().bit_is_clear() {};
        self.i2c1.txdr.write(|w| w.txdata().bits( data | 0x08));
        while self.i2c1.isr.read().tc().bit_is_clear() {}
    }

    fn pulse_enable(&mut self,data:u8){
        let en :u8 = 0b00000100;
        self.expander_write(data|en);
        self.delay.delay_us(1u32);
        self.expander_write(data &!en);
	    self.delay.delay_us(1u32);	
    }
}