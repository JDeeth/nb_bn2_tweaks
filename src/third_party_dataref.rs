use replace_with::replace_with_or_abort;
use xplm::data::{borrowed::DataRef, DataRead, DataReadWrite, ReadWrite};

/// Dataref owned by another aircraft plugin
/// (May not be loaded when our plugin runs)
pub enum ThirdPartyDataref {
    NotFound(String),
    Found(DataRef<u32, ReadWrite>),
}

impl ThirdPartyDataref {
    pub fn new(dataref_name: String) -> Self {
        Self::NotFound(dataref_name)
    }

    fn find(&mut self) {
        replace_with_or_abort(self, |self_| {
            if let ThirdPartyDataref::NotFound(ref dataref_name) = self_ {
                if let Ok(dr) = DataRef::<u32>::find(dataref_name) {
                    if let Ok(writable_dr) = dr.writeable() {
                        return Self::Found(writable_dr);
                    }
                }
            };
            self_
        });
    }

    pub fn get(&mut self) -> Option<u32> {
        self.find();
        match self {
            ThirdPartyDataref::NotFound(_) => None,
            ThirdPartyDataref::Found(data_ref) => Some(data_ref.get()),
        }
    }

    pub fn set(&mut self, pos: u32) {
        self.find();
        match self {
            ThirdPartyDataref::NotFound(_) => (),
            ThirdPartyDataref::Found(ref mut data_ref) => data_ref.set(pos),
        };
    }
}
