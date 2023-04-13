fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }
    slice_from_raw_parts();
    inline_assembly::nop();
    inline_assembly::mov();
    inline_assembly::mov_add();
    inline_assembly::inout_add();
    inline_assembly::inlateout_add();
    // inline_assembly::explicit_register();    // segmentation fault
    inline_assembly::explicit_registers_mul();   
    inline_assembly::clobbered_registers();      
    inline_assembly::mul_shift_add();            
    inline_assembly::call_foo_clobber_abi(1);
    inline_assembly::register_template_modifier();
    inline_assembly::load_fpu_control_word(1);
    inline_assembly::labels();
    inline_assembly::options();
}



fn slice_from_raw_parts() {
    use std::slice;
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}

mod inline_assembly {
    use std::arch::asm;

    pub fn nop(){
        unsafe {
            asm!("nop");
        }
    }

    pub fn mov(){
        let x: u64;
        unsafe {
            asm!("mov {}, 5", out(reg) x);
        }
        assert_eq!(x, 5);
    }

    pub fn mov_add(){
        let i: u64 = 3;
        let o: u64;

        unsafe {
            asm!(
                "mov {0}, {1}",
                "add {0}, 5",
                out(reg) o,
                in(reg) i,
            );
        }
        assert_eq!(o, 8);
    }

    pub fn inout_add(){
        let mut x: u64 = 3;
        unsafe {
            asm!("add {0}, 5", inout(reg) x);
        }
        assert_eq!(x, 8);
    } 

    pub fn inlateout_add(){
        let mut a: u64 = 4;
        let b: u64 = 4;
        unsafe {
            asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
        }
        assert_eq!(a, 8);
    }

    #[allow(dead_code)]
    pub fn explicit_register(){
        let cmd = 0xd1;
        unsafe {
            asm!("out 0x64, eax", in("eax") cmd);
        }
    }

    
    pub fn explicit_registers_mul(){
         mul(2u64,3u64);
    }
    fn mul(a: u64, b: u64) -> u128 {
        let lo: u64;
        let hi: u64;
    
        unsafe {
            asm!(
                // The x86 mul instruction takes rax as an implicit input and writes
                // the 128-bit result of the multiplication to rax:rdx.
                "mul {}",
                in(reg) a,
                inlateout("rax") b => lo,
                lateout("rdx") hi
            );
        }
    
        ((hi as u128) << 64) + lo as u128
    
    }

    pub fn clobbered_registers() {
        // three entries of four bytes each
        let mut name_buf = [0_u8; 12];
        // String is stored as ascii in ebx, edx, ecx in order
        // Because ebx is reserved, the asm needs to preserve the value of it.
        // So we push and pop it around the main asm.
        // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)
    
        unsafe {
            asm!(
                "push rbx",
                "cpuid",
                "mov [rdi], ebx",
                "mov [rdi + 4], edx",
                "mov [rdi + 8], ecx",
                "pop rbx",
                // We use a pointer to an array for storing the values to simplify
                // the Rust code at the cost of a couple more asm instructions
                // This is more explicit with how the asm works however, as opposed
                // to explicit register outputs such as `out("ecx") val`
                // The *pointer itself* is only an input even though it's written behind
                in("rdi") name_buf.as_mut_ptr(),
                // select cpuid 0, also specify eax as clobbered
                inout("eax") 0 => _,
                // cpuid clobbers these registers too
                out("ecx") _,
                out("edx") _,
            );
        }
    
        let name = core::str::from_utf8(&name_buf).unwrap();
        println!("CPU Manufacturer ID: {}", name);
    }

    pub fn mul_shift_add(){
        // Multiply x by 6 using shifts and adds
        let mut x: u64 = 4;
        unsafe {
            asm!(
                "mov {tmp}, {x}",
                "shl {tmp}, 1",
                "shl {x}, 2",
                "add {x}, {tmp}",
                x = inout(reg) x,
                tmp = out(reg) _,
            );
        }
        assert_eq!(x, 4 * 6);
    }

    extern "C" fn foo(arg: i32) -> i32 {
        println!("arg = {}", arg);
        arg * 2
    }
    
    pub fn call_foo_clobber_abi (arg: i32) -> i32 {
        unsafe {
            let result;
            asm!(
                "call {}",
                // Function pointer to call
                in(reg) foo,
                // 1st argument in rdi
                in("rdi") arg,
                // Return value in rax
                out("rax") result,
                // Mark all registers which are not preserved by the "C" calling
                // convention as clobbered.
                clobber_abi("C"),
            );
            result
        }
    }

    pub fn register_template_modifier(){
        let mut x: u16 = 0xab;

        unsafe {
            asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
        }

        assert_eq!(x, 0xabab);
    }

    pub fn load_fpu_control_word(control: u16) {
        unsafe {
            asm!("fldcw [{}]", in(reg) &control, options(nostack));
        }
    }

    #[allow(asm_sub_register)]
    pub fn labels(){
        #[allow(unused_assignments)]
        let mut a = 0;
        unsafe {
            asm!(
                "mov {0}, 10",
                "2:",
                "sub {0}, 1",
                "cmp {0}, 3",
                "jle 2f",
                "jmp 2b",
                "2:",
                "add {0}, 2",
                out(reg) a
            );
        }
        assert_eq!(a, 5);
    }

    pub fn options(){
        let mut a: u64 = 4;
        let b: u64 = 4;
        unsafe {
            asm!(
                "add {0}, {1}",
                inlateout(reg) a, in(reg) b,
                options(pure, nomem, nostack),
            );
        }
        assert_eq!(a, 8);
    }
}