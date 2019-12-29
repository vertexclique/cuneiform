use super::CuneiformArgs;

#[cfg(target_os = "linux")]
use super::slabs::FALLBACK_COHERENCE_LINE_SIZE;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use walkdir::WalkDir;

#[cfg(target_os = "linux")]
const LINUX_CPU_PROC_PROBE: &'static str = "/sys/devices/system/cpu/";
#[cfg(target_os = "linux")]
const CACHE_LINE_PROBE: &'static str = "coherency_line_size";

#[cfg(target_os = "linux")]
pub(crate) fn hermetic_detection(_args: CuneiformArgs) -> usize {
    let mut ways = Vec::<usize>::new();
    for entry in WalkDir::new(LINUX_CPU_PROC_PROBE)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().ends_with(CACHE_LINE_PROBE))
    {
        let clps =
            fs::read_to_string(entry.path()).expect("Unexpected failure reading cache line probe");
        let clps = clps.trim();
        let clp = clps.parse::<usize>().unwrap();
        ways.push(clp);
    }

    ways.iter()
        .min()
        .map_or(FALLBACK_COHERENCE_LINE_SIZE as usize, |e| *e)
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn hermetic_detection(args: CuneiformArgs) -> usize {
    super::slabs::slab_fetch(args) as usize
}

#[test]
fn hermetic_test() {
    let mut cargs = CuneiformArgs::new();
    cargs.with_hermetic(true);

    hermetic_detection(cargs);
}
