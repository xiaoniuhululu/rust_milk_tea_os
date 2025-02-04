use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;
use lazy_static::lazy_static;
use crate::gdt;
use pic8259::ChainedPics;
use spin;
use crate::print;
use crate::hlt_loop;

// static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();


// pub fn init_idt() {
//     // let mut idt = InterruptDescriptorTable::new();
//     // idt.breakpoint.set_handler_fn(breakpoint_handler);
//     // idt.load();
//     unsafe {
//         IDT.breakpoint.set_handler_fn(breakpoint_handler);
//         IDT.load();
//     }
    
// }

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard
}


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // idt.double_fault.set_handler_fn(double_fault_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(keyboard_interrupt_handler);

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}



impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    // print!(".");
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// extern "x86-interrupt" fn keyboard_interrupt_handler(
//     _stack_frame: InterruptStackFrame)
// {
//     // print!("k");
//     use x86_64::instructions::port::Port;
//     use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
//     use spin::Mutex;

//     // let mut port = Port::new(0x60);
//     // let scancode: u8 = unsafe { port.read() };
//     // // print!("{}", scancode);

//     // let key = match scancode {
//     //     0x02 => Some('1'),
//     //     0x03 => Some('2'),
//     //     0x04 => Some('3'),
//     //     0x05 => Some('4'),
//     //     0x06 => Some('5'),
//     //     0x07 => Some('6'),
//     //     0x08 => Some('7'),
//     //     0x09 => Some('8'),
//     //     0x0a => Some('9'),
//     //     0x0b => Some('0'),
//     //     _ => None,
//     // };
//     // if let Some(key) = key {
//     //     print!("{}", key);
//     // }

//     lazy_static! {
//         static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
//             Mutex::new(Keyboard::new(ScancodeSet1::new(),
//                 layouts::Us104Key, HandleControl::Ignore)
//             );
//     }

//     let mut keyboard = KEYBOARD.lock();
//     let mut port = Port::new(0x60);

//     let scancode: u8 = unsafe { port.read() };
//     if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//         if let Some(key) = keyboard.process_keyevent(key_event) {
//             match key {
//                 DecodedKey::Unicode(character) => print!("{}", character),
//                 DecodedKey::RawKey(key) => print!("{:?}", key),
//             }
//         }
//     }

//     unsafe {
//         PICS.lock()
//             .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
//     }

// }

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode); // new

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}