// MIT/Apache2 License

#![cfg(feature = "randr")]

use super::auto::{AsByteSequence, randr::{CrtcChange, OutputChange, OutputProperty, ProviderChange, ProviderProperty, ResourceChange, LeaseNotify}};

const NOTIFY_DATA_LEN: usize = 28;

/// Notification data.
#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NotifyData([u8; NOTIFY_DATA_LEN]);

impl AsByteSequence for NotifyData {
  #[inline]
  fn size(&self) -> usize { NOTIFY_DATA_LEN }

  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) -> usize {
    (&mut bytes[..NOTIFY_DATA_LEN]).copy_from_slice(&self.0);
    NOTIFY_DATA_LEN
  }

  #[inline]
  fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
    let mut data = Self([0; NOTIFY_DATA_LEN]);
    data.0.copy_from_slice(&bytes[..NOTIFY_DATA_LEN]);
    Some((data, NOTIFY_DATA_LEN))
  }
}

impl NotifyData {
  #[inline]
  pub fn as_crtc_change(&self) -> Option<CrtcChange> {
    Some(CrtcChange::from_bytes(&self.0)?.0)
  }

  #[inline]
  pub fn into_crtc_change(self) -> Option<CrtcChange> {
    self.as_crtc_change()
  }

  #[inline]
  pub fn as_output_change(&self) -> Option<OutputChange> {
    Some(OutputChange::from_bytes(&self.0)?.0)
  }

  #[inline]
  pub fn into_output_change(self) -> Option<OutputChange> { self.as_output_change() }

  #[inline]
  pub fn as_output_property(&self) -> Option<OutputProperty> { 
Some(OutputProperty::from_bytes(&self.0)?.0) }

  #[inline]
  pub fn into_output_property(self) -> Option<OutputProperty> { self.as_output_property() }

  #[inline]
  pub fn as_provider_change(&self) -> Option<ProviderChange> { 
Some(ProviderChange::from_bytes(&self.0)?.0) }

  #[inline]
  pub fn into_provider_change(self) -> Option<ProviderChange> { self.as_provider_change() }

  #[inline]
  pub fn as_provider_property(&self) -> Option<ProviderProperty> { 
Some(ProviderProperty::from_bytes(&self.0)?.0) }

  #[inline]
  pub fn into_provider_property(self) -> Option<ProviderProperty> { self.as_provider_property() }

  #[inline]
  pub fn as_resource_change(&self) -> Option<ResourceChange> {
Some(ResourceChange::from_bytes(&self.0)?.0) }

  #[inline]
  pub fn into_resource_change(self) -> Option<ResourceChange> { self.as_resource_change() }

  #[inline]
  pub fn as_lease_notify(&self) -> Option<LeaseNotify> {
Some(LeaseNotify::from_bytes(&self.0)?.0) }

  #[inline]
  pub fn into_lease_notify(self) -> Option<LeaseNotify> { self.as_lease_notify() }
}
