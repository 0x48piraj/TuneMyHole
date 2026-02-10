use std::path::PathBuf;

pub struct Paths {
    pub ftl_db: PathBuf,

    // INPUTS
    pub reference_dir: PathBuf,

    // OUTPUTS
    pub managed_list: PathBuf,
    pub meta: PathBuf,
    pub state: PathBuf,
    pub config: PathBuf,
}

impl Paths {
    pub fn detect() -> Self {
        Self {
            ftl_db: PathBuf::from("/etc/pihole/pihole-FTL.db"),
            reference_dir: PathBuf::from("/etc/pihole/tune-my-hole.d"),
            managed_list: PathBuf::from("/etc/pihole/tune-my-hole.list"),
            meta: PathBuf::from("/etc/pihole/tune-my-hole.meta.json"),
            state: PathBuf::from("/etc/pihole/tune-my-hole.state.json"),
            config: PathBuf::from("/etc/pihole/tune-my-hole.conf"),
        }
    }
}
