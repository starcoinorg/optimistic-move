use crate::data::Data;
use anyhow::Result;

use unicorn_engine::unicorn_const::{uc_error, MemRegion, Permission};
use unicorn_engine::Unicorn;

#[derive(Debug)]
struct MapInfo {
    info: MemRegion,
    label: String,
}

#[derive(Default, Debug)]
pub struct MemoryManager {
    map_info: Vec<MapInfo>,
}

impl MemoryManager {
    pub(crate) fn add_mapinfo(&mut self, mem_info: MemRegion, label: String) {
        self.map_info.push(MapInfo {
            info: mem_info,
            label,
        });
        self.map_info.sort_by_key(|info| info.info.begin);
    }
}

pub trait Memory {
    fn mem_map(&mut self, region: MemRegion, info: Option<String>) -> Result<(), uc_error>;
    fn write(&mut self, address: u64, bytes: impl AsRef<[u8]>) -> Result<(), uc_error>;
}

impl<'a> Memory for Unicorn<'a, Data> {
    fn mem_map(
        &mut self,
        MemRegion { begin, end, perms }: MemRegion,
        info: Option<String>,
    ) -> Result<(), uc_error> {
        debug_assert!(
            perms & (!Permission::ALL) == Permission::NONE,
            "unexcepted permissions mask {:?}",
            perms
        );

        self.mem_map(begin, (end - begin) as usize, perms)?;
        self.get_data_mut().memories.add_mapinfo(
            MemRegion { begin, end, perms },
            info.unwrap_or("[mapped]".to_string()),
        );
        Ok(())
    }
    fn write(&mut self, address: u64, bytes: impl AsRef<[u8]>) -> Result<(), uc_error> {
        self.mem_write(address, bytes.as_ref())
    }
}
