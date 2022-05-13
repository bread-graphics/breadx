// MIT/Apache2 License

use ahash::RandomState;

use super::Prefetch;
use crate::HashMap;

cfg_sync! {
    use crate::mutex::RwLock;
}

use x11rb_protocol::{
    protocol::xproto::QueryExtensionRequest,
    x11_utils::{ExtInfoProvider, ExtensionInformation},
};

/// A map of extension names to extension information.
pub(crate) struct ExtensionMap {
    inner: HashMap<&'static str, Prefetch<QueryExtensionRequest<'static>>>,
}

impl Default for ExtensionMap {
    fn default() -> Self {
        Self {
            inner: HashMap::with_hasher(RandomState::new()),
        }
    }
}

impl ExtensionMap {
    pub(crate) fn get(&self, name: &'static str) -> Option<Option<ExtensionInformation>> {
        self.inner
            .get(&name)
            .and_then(|p| p.get_if_resolved())
            .copied()
    }

    pub(crate) fn insert(
        &mut self,
        name: &'static str,
        pf: Prefetch<QueryExtensionRequest<'static>>,
    ) {
        self.inner.insert(name, pf);
    }

    /// Utility function to find an extension that matches a closure.
    fn find(
        &self,
        mut closure: impl FnMut(&ExtensionInformation) -> bool,
    ) -> Option<(&'static str, ExtensionInformation)> {
        self.inner
            .iter()
            .filter_map(|(name, pf)| {
                pf.get_if_resolved()
                    .copied()
                    .flatten()
                    .map(|ext_info| (*name, ext_info))
            })
            .find(|(_, ext_info)| closure(ext_info))
    }
}

impl ExtInfoProvider for ExtensionMap {
    fn get_from_major_opcode(&self, major_opcode: u8) -> Option<(&str, ExtensionInformation)> {
        self.find(|info| info.major_opcode == major_opcode)
    }

    fn get_from_event_code(&self, event_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.find(|info| info.first_event == event_code)
    }

    fn get_from_error_code(&self, error_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.find(|info| info.first_error == error_code)
    }
}

cfg_sync! {
    impl ExtInfoProvider for RwLock<ExtensionMap> {
        fn get_from_major_opcode(&self, major_opcode: u8) -> Option<(&str, ExtensionInformation)> {
            self.read().find(|info| info.major_opcode == major_opcode)
        }

        fn get_from_event_code(&self, event_code: u8) -> Option<(&str, ExtensionInformation)> {
            self.read().find(|info| info.first_event == event_code)
        }

        fn get_from_error_code(&self, error_code: u8) -> Option<(&str, ExtensionInformation)> {
            self.read().find(|info| info.first_error == error_code)
        }
    }
}
