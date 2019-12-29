use super::CuneiformArgs;
use hashbrown::HashMap;
use lazy_static::*;

// ----------------------------------------------------
// Ok, here we go with a Sumerian slab.
// That's why this thing is called cuneiform.
// Snow Crash, right here.
// ----------------------------------------------------

//
// 32-bit architecture and things other than netburst microarchitecture are using 64 bytes.
#[cfg(target_arch = "x86")]
const COHERENCE_LINE_SIZE: u8 = (1 << 6);

//
// Intel x86_64:
// L2D streamer from L1:
// Loads data or instructions from memory to the second-level cache. To use the streamer,
// organize the data or instructions in blocks of 128 bytes, aligned on 128 bytes.
// - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
#[cfg(target_arch = "x86_64")]
const COHERENCE_LINE_SIZE: u8 = (1 << 7);

//
// 24Kc:
// Data Line Size
// - https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00346-2B-24K-DTS-04.00.pdf
// - https://gitlab.e.foundation/e/devices/samsung/n7100/stable_android_kernel_samsung_smdk4412/commit/2dbac10263b2f3c561de68b4c369bc679352ccee
// majority uses 32. can be overridden with #[cuneiform(force = 16)]
#[cfg(target_arch = "mips")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);
#[cfg(target_arch = "mips64")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);

//
// Defaults for powerpc
#[cfg(target_arch = "powerpc")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);

//
// Defaults for the ppc 64
#[cfg(target_arch = "powerpc64")]
const COHERENCE_LINE_SIZE: u8 = (1 << 6);

//
// e.g.: sifive
// - https://github.com/torvalds/linux/blob/master/Documentation/devicetree/bindings/riscv/sifive-l2-cache.txt#L41
// in general all of them are the same.
#[cfg(target_arch = "riscv")]
const COHERENCE_LINE_SIZE: u8 = (1 << 6);

//
// This is fixed.
// - https://docs.huihoo.com/doxygen/linux/kernel/3.7/arch_2s390_2include_2asm_2cache_8h.html
#[cfg(target_arch = "s390x")]
const COHERENCE_LINE_SIZE: u8 = (1 << 8);

//
// This is also fixed.
// - https://docs.huihoo.com/doxygen/linux/kernel/3.7/arch_2sparc_2include_2asm_2cache_8h.html#a9400cc2ba37e33279bdbc510a6311fb4
#[cfg(target_arch = "sparc")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);

//
// This is also fixed.
#[cfg(target_arch = "sparc64")]
const COHERENCE_LINE_SIZE: u8 = (1 << 6);

//
// On ARM cache line sizes are fixed. both v6 and v7.
// Need to add board specific or platform specific things to the slabs.
#[cfg(target_arch = "thumbv6")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);
#[cfg(target_arch = "thumbv7")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);

//
// Like what? I mean... This?
#[cfg(target_arch = "wasm32")]
const COHERENCE_LINE_SIZE: u8 = FALLBACK_COHERENCE_LINE_SIZE;

//
// Same as v6 and v7.
// Boards specifics should go to the slabs.
// Tbh, list goes like that:
// Cortex A, M, R, ARM v7, v7-M, Krait and NeoverseN uses this size.
#[cfg(target_arch = "arm")]
const COHERENCE_LINE_SIZE: u8 = (1 << 5);

//
// lil nit: https://youtu.be/IVpOyKCNZYw
// Combined from 4 sectors. Volta says 128.
// Prevent chunk optimizations better to go to the default size.
// If you have smaller data with less padded functionality then use 32 with force option.
// - https://devtalk.nvidia.com/default/topic/803600/variable-cache-line-width-/
#[cfg(target_arch = "nvptx")]
const COHERENCE_LINE_SIZE: u8 = (1 << 7);
#[cfg(target_arch = "nvptx64")]
const COHERENCE_LINE_SIZE: u8 = (1 << 7);

//
// This is fixed.
#[cfg(target_arch = "aarch64")]
const COHERENCE_LINE_SIZE: u8 = (1 << 6);

//
// Look, this should never happen.
// If this happens call me from landline. asap.
// I prefer blue box.
pub(crate) const FALLBACK_COHERENCE_LINE_SIZE: u8 = (1 << 6);

#[inline]
pub fn slabs() -> &'static HashMap<&'static str, u8> {
    lazy_static! {
        static ref SLABS: HashMap<&'static str, u8> = [
            ("powerpc_mpc8xx", 1 << 4),

            // We don't have this in LLVM yet!
            // Combined bridge emulation mode for ppc64.
            // Uses SLB. so that bad boy loads 2 DWORDS at a time.
            ("powerpc64bridge", 1 << 7),

            // E500 signal processors use 64 even they are 32-bit in most cases. But for the sake of working with
            // all of them alignment can be parallelizable. Since it's risc can do pipelining and
            // work optimally with the value below. No false sharing.
            ("powerpc_e500mc", 1 << 6),
            ("power_7", 1 << 7),
            ("power_8", 1 << 7),
            ("power_9", 1 << 7),

            // Other specific archs at the ARM side
            ("exynos_big", 1 << 7),
            ("exynos_LITTLE", 1 << 6),
            ("krait", 1 << 6),
            // Arm server side
            ("neoverse_n1", 1 << 6),
        ].iter().copied().collect();
    }

    &*SLABS
}

pub(crate) fn fetch(args: CuneiformArgs) -> u8 {
    if args.force != 0 && args.force % 2 == 0 {
        return args.force as u8;
    } else {
        if args.force != !0 {
            panic!("Forced value is not acceptable.");
        }
    }

    if args.hermetic {
        super::detection::hermetic_detection(args) as u8
    } else {
        slab_fetch(args)
    }
}

pub(crate) fn slab_fetch(args: CuneiformArgs) -> u8 {
    slabs()
        .get(args.slab.as_str())
        .map_or(COHERENCE_LINE_SIZE, |e| *e)
}
