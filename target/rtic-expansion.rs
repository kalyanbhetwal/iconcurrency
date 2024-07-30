#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    stm32f3xx_hal_v2 :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 1 <<
    stm32f3xx_hal_v2 :: pac :: NVIC_PRIO_BITS; use core :: arch :: asm; use
    core :: mem; use core :: ptr; use cortex_m_semihosting ::
    { debug, hprintln }; use cortex_m :: asm :: { nop, self }; use crate ::
    checkpoint :: { self, delete_all_pg, restore, checkpoint, erase_all }; use
    crate :: checkpoint :: my_flash ::
    { unlock, wait_ready, clear_error_flags, erase_page, write_to_flash }; use
    stm32f3xx_hal_v2 ::
    {
        pac :: { self, NVIC }, pac :: Peripherals, pac :: FLASH, pac ::
        Interrupt, gpio :: { gpioa :: PA0, Input, PullUp }
    }; use volatile :: Volatile; #[no_mangle] fn c_checkpoint(c_type : bool)
    {
        unsafe { asm! ("add sp, #76"); } unsafe
        { asm! ("pop	{{r8, r9, sl, fp}}") }; unsafe
        { asm! ("pop	{{r4, r5, r6, r7, lr}}") }; unsafe
        { asm! ("push	{{r4, r5, r6, r7, lr}}") }; unsafe
        { asm! ("push	{{r8, r9, sl, fp}}") }; unsafe { asm! ("sub sp, #76"); }
        let r0_value : u32; let r1_value : u32; let r2_value : u32; let
        r3_value : u32; let r4_value : u32; let r5_value : u32; let r6_value :
        u32; let r7_value : u32; let r8_value : u32; let r9_value : u32; let
        r10_value : u32; let r11_value : u32; let r12_value : u32; let r13_sp
        : u32; let r14_lr : u32; let r15_pc : u32; unsafe
        { asm! ("MOV {0}, r0", out(reg) r0_value); } unsafe
        { asm! ("MOV {0}, r1", out(reg) r1_value); } unsafe
        { asm! ("MOV {0}, r2", out(reg) r2_value); } unsafe
        { asm! ("MOV {0}, r3", out(reg) r3_value); } unsafe
        { asm! ("MOV {0}, r4", out(reg) r4_value); } unsafe
        { asm! ("MOV {0}, r5", out(reg) r5_value); } unsafe
        { asm! ("MOV {0}, r14", out(reg) r14_lr); } unsafe
        { asm! ("MOV {0}, r7", out(reg) r7_value); } unsafe
        { asm! ("MOV {0}, r8", out(reg) r8_value); } unsafe
        { asm! ("MOV {0}, r9", out(reg) r9_value); } unsafe
        { asm! ("MOV {0}, r10", out(reg) r10_value); } unsafe
        { asm! ("MOV {0}, r11", out(reg) r11_value); } unsafe
        { asm! ("MOV {0}, r12", out(reg) r12_value); } unsafe
        { asm! ("MOV {0}, r6", out(reg) r6_value); } unsafe
        { asm! ("MOV {0}, r15", out(reg) r15_pc); } unsafe
        { asm! ("MOV r0, sp"); } unsafe { asm! ("add r0, #112"); } unsafe
        { asm! ("MOV {0}, r0", out(reg) r13_sp); } unsafe
        {
            let dp = Peripherals :: steal(); let mut flash = dp.FLASH;
            unlock(& mut flash); wait_ready(& flash); let mut start_address :
            u32; let end_address = r13_sp; asm!
            ("movw r0, 0xFFF8
             movt r0, 0x2000"); asm!
            ("MOV {0}, r0", out(reg) start_address); let stack_size =
            (start_address - end_address) + 4; let mut flash_start_address =
            Volatile :: new(0x0803_0000); let mut flash_end_address = Volatile
            :: new(0x0807_FFFF); let mut checkpoint_size = Volatile ::
            new(0u32); checkpoint_size.write(stack_size + 4 + 16 * 4 + 4 + 4);
            asm :: dmb(); loop
            {
                let mut offset = ptr ::
                read_volatile(flash_start_address.read() as * const u32); if
                offset == 0xffff_ffff { break; }
                flash_start_address.write(flash_start_address.read() +
                offset); if flash_start_address.read() +
                checkpoint_size.read() >= flash_end_address.read()
                {
                    erase_all(& mut flash); flash_start_address = Volatile ::
                    new(0x0803_0000); break;
                }
            } asm :: dmb();
            write_to_flash(& mut flash, (flash_start_address.read()) as u32,
            checkpoint_size.read() as u32);
            flash_start_address.write(flash_start_address.read() + 4); asm ::
            dmb(); asm :: dmb(); if c_type
            {
                write_to_flash(& mut flash, flash_start_address.read() as u32,
                0xDEADBEEF as u32);
            } else
            {
                write_to_flash(& mut flash, flash_start_address.read() as u32,
                0x0000_0001 as u32);
            } while start_address >= end_address
            {
                let mut data = Volatile :: new(0u32);
                data.write(core :: ptr ::
                read_volatile(start_address as * const u32));
                write_to_flash(& mut flash, flash_start_address.read() as u32,
                data.read() as u32);
                flash_start_address.write(flash_start_address.read() + 1 * 4);
                start_address = start_address - 4;
            } asm :: dmb(); asm :: dmb();
            write_to_flash(& mut flash, (flash_start_address.read()) as u32,
            0xf1f1_f1f1 as u32);
            flash_start_address.write(flash_start_address.read() + 4); asm ::
            dmb();
            write_to_flash(& mut flash, flash_start_address.read() as u32,
            r0_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 4 as u32,
            r1_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 8 as u32,
            r2_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 12 as
            u32, r3_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 16 as
            u32, r4_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 20 as
            u32, r5_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 24 as
            u32, r6_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 28 as
            u32, r7_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 32 as
            u32, r8_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 36 as
            u32, r9_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 40 as
            u32, r10_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 44 as
            u32, r11_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 48 as
            u32, r12_value as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 52 as
            u32, r13_sp as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 56 as
            u32, r14_lr as u32);
            write_to_flash(& mut flash, flash_start_address.read() + 60 as
            u32, r15_pc as u32);
        }
    } #[doc = r" User code end"] #[doc = r"Shared resources"] struct Shared
    { a : u32, b : u32, flash : pac :: FLASH, } #[doc = r"Local resources"]
    struct Local {} #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Core peripherals"] pub core : rtic :: export ::
        Peripherals,
        #[doc = r" The space used to allocate async executors in bytes."] pub
        executors_size : usize, #[doc = r" Device peripherals (PAC)"] pub
        device : stm32f3xx_hal_v2 :: pac :: Peripherals,
        #[doc = r" Critical section token for init"] pub cs : rtic :: export
        :: CriticalSection < 'a > ,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals, executors_size : usize) ->
        Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, device :
                stm32f3xx_hal_v2 :: pac :: Peripherals :: steal(), cs : rtic
                :: export :: CriticalSection :: new(), core, executors_size,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_init_Context as
        Context;
    } #[inline(always)] #[allow(non_snake_case)] fn
    init(ctx : init :: Context) -> (Shared, Local)
    {
        let mut dp = ctx.device; restore(); hprintln! ("init"); unsafe
        {
            asm! ("mov r1, #6"); asm! ("mov r2, #2"); asm! ("mov r3, #4");
            asm! ("mov r4, #6"); asm! ("mov r6, #12"); asm! ("mov r8, #9");
            asm! ("mov r9, #6"); asm! ("mov r12, #2");
        } c_checkpoint(false);
        (Shared { flash : dp.FLASH, a : 0, b : 0 }, Local {})
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > ,
    } impl < 'a > __rtic_internal_idle_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idle_Context
            { __rtic_internal_p : :: core :: marker :: PhantomData, }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idle_Context as
        Context;
    } #[allow(non_snake_case)] fn idle(_ : idle :: Context) -> !
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; loop
        { hprintln! ("idle"); debug :: exit(debug :: EXIT_SUCCESS); }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_a : rtic :: RacyCell < core :: mem ::
    MaybeUninit < u32 >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: a_that_needs_to_be_locked < 'a >
    {
        type T = u32; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u32) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 0u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_a.get_mut() as * mut _,
                CEILING, stm32f3xx_hal_v2 :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_b : rtic :: RacyCell < core :: mem ::
    MaybeUninit < u32 >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: b_that_needs_to_be_locked < 'a >
    {
        type T = u32; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u32) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 0u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_b.get_mut() as * mut _,
                CEILING, stm32f3xx_hal_v2 :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_shared_resource_flash : rtic :: RacyCell < core :: mem ::
    MaybeUninit < pac :: FLASH >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: flash_that_needs_to_be_locked < 'a >
    {
        type T = pac :: FLASH; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut pac :: FLASH) -> RTIC_INTERNAL_R)
        -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 0u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_flash.get_mut() as * mut
                _, CEILING, stm32f3xx_hal_v2 :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } mod shared_resources
    {
        #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        a_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > a_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                a_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        b_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > b_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                b_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        flash_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > flash_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                flash_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        }
    } #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() -> !
    {
        rtic :: export :: interrupt :: disable(); let mut core : rtic ::
        export :: Peripherals = rtic :: export :: Peripherals ::
        steal().into(); let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: TIM2; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: TIM3; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: TIM4; #[inline(never)] fn __rtic_init_resources < F >
        (f : F) where F : FnOnce() { f(); } let mut executors_size = 0; extern
        "C" { pub static _stack_start : u32; pub static __ebss : u32; } let
        stack_start = & _stack_start as * const _ as u32; let ebss = & __ebss
        as * const _ as u32; if stack_start > ebss
        {
            if rtic :: export :: msp :: read() <= ebss
            { panic! ("Stack overflow after allocating executors"); }
        }
        __rtic_init_resources(||
        {
            let (shared_resources, local_resources) =
            init(init :: Context :: new(core.into(), executors_size)); rtic ::
            export :: interrupt :: enable();
        }); idle(idle :: Context :: new())
    }
}