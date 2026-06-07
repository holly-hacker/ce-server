use super::handle_allocator::HandleAllocator;

/// Fills `size` bytes with a deterministic xorshift stream, so seeded modules
/// hold reproducible "data" for a client to read or scan.
fn seeded_memory(size: usize, seed: u64) -> Vec<u8> {
    let mut state = if seed == 0 { 1 } else { seed };
    (0..size)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state as u8
        })
        .collect()
}

/// The single source of truth for the dummy backend.
pub struct World {
    pub processes: Vec<DummyProcess>,
    pub handles: HandleAllocator<HandleObject>,
}

pub enum HandleObject {
    Snapshot(Snapshot),
    Process { pid: i32 },
}

/// Entries are references into the world (a pid, or a pid + module index),
/// resolved live on each Process32/Module32 call rather than copied.
#[derive(Default)]
pub struct Snapshot {
    pub process_idx: usize,
    pub processes: Vec<i32>,
    pub module_idx: usize,
    pub modules: Vec<(i32, usize)>,
}

pub struct DummyProcess {
    pub pid: i32,
    pub name: &'static str,
    pub modules: Vec<DummyModule>,
}

pub struct DummyModule {
    pub name: &'static str,
    pub base: i64,
    /// The module's bytes; its length is the module size.
    pub memory: Vec<u8>,
}

impl World {
    pub fn new() -> Self {
        Self {
            handles: HandleAllocator::new(0x1000_0000),
            processes: vec![
                DummyProcess {
                    pid: 1,
                    name: "Dummy 1",
                    modules: vec![
                        DummyModule {
                            name: "dummy1.exe",
                            base: 0x0000_0000_0040_0000,
                            memory: seeded_memory(0x1000, 1),
                        },
                        DummyModule {
                            name: "kernel32.dll",
                            base: 0x0000_7fff_0000_0000,
                            memory: vec![0; 0x2000],
                        },
                    ],
                },
                DummyProcess {
                    pid: 2,
                    name: "Dummy 2",
                    modules: vec![
                        DummyModule {
                            name: "dummy2.exe",
                            base: 0x0000_0000_0140_0000,
                            memory: seeded_memory(0x1000, 2),
                        },
                        DummyModule {
                            name: "kernel32.dll",
                            base: 0x0000_7fff_0000_0000,
                            memory: vec![0; 0x2000],
                        },
                        DummyModule {
                            name: "user32.dll",
                            base: 0x0000_7fff_0010_0000,
                            memory: vec![0; 0x2000],
                        },
                    ],
                },
                DummyProcess {
                    pid: 3,
                    name: "Dummy 3",
                    modules: vec![DummyModule {
                        name: "dummy3.exe",
                        base: 0x0000_0000_0040_0000,
                        memory: seeded_memory(0x1000, 3),
                    }],
                },
            ],
        }
    }

    pub fn process(&self, pid: i32) -> Option<&DummyProcess> {
        self.processes.iter().find(|process| process.pid == pid)
    }

    pub fn process_mut(&mut self, pid: i32) -> Option<&mut DummyProcess> {
        self.processes.iter_mut().find(|process| process.pid == pid)
    }
}
