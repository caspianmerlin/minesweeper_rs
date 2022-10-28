use rand::{rngs::ThreadRng, Rng};

pub trait RandomNumberGenerator {
    fn random_u32(&mut self, max: u32) -> usize;
}

pub struct LegacyRandomNumberGenerator;
impl LegacyRandomNumberGenerator {
    #[cfg(target_os="windows")]
    pub fn new() -> LegacyRandomNumberGenerator {
        unsafe { srand(lo_word(GetTickCount()) as u32) }
        LegacyRandomNumberGenerator {}
    }
    #[cfg(target_os="linux")]
    pub fn new() -> LegacyRandomNumberGenerator {
        unsafe { srand(lo_word(get_tick_count()) as u32) }
        LegacyRandomNumberGenerator {}
    }
}
impl RandomNumberGenerator for LegacyRandomNumberGenerator {
    fn random_u32(&mut self, max: u32) -> usize {
        unsafe { (rand() as u32 % max) as usize }
    }
}

pub struct ModernRandomNumberGenerator {
    inner: ThreadRng,
}
impl ModernRandomNumberGenerator {
    pub fn new() -> ModernRandomNumberGenerator {
        ModernRandomNumberGenerator {
            inner: rand::thread_rng()
        }
    }
}
impl RandomNumberGenerator for ModernRandomNumberGenerator {
    fn random_u32(&mut self, max: u32) -> usize {
        self.inner.gen_range(0..=max) as usize
    }
}




extern "C" {
    fn rand() -> i32;
    fn srand(seed: u32);
}

#[cfg(target_os="windows")]
#[link(name = "Kernel32")]
extern "C" {
    fn GetTickCount() -> u32;
}

#[cfg(target_os="linux")]
fn get_tick_count() -> u32 {
    let time_spec = nix::time::clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC).expect("Could not retrieve TickCount from Linux system");
    let elapsed_millis = (time_spec.tv_nsec() / 1000) + (time_spec.tv_sec() * 1000);
    elapsed_millis as u32
}


#[inline]
const fn lo_word(v: u32) -> u16 {
	(v & 0xffff) as _
}