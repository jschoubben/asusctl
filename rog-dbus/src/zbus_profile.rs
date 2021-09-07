//! # DBus interface proxy for: `org.asuslinux.Daemon`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/asuslinux/Profile' from service 'org.asuslinux.Daemon' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://zeenix.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use std::sync::mpsc::Sender;

use rog_profiles::{Profile, fan_curves::FanCurveSet};
use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.asuslinux.Daemon",
    default_path = "/org/asuslinux/Profile"
)]
trait Daemon {
    /// Profiles method
    fn profiles(&self) -> zbus::Result<Vec<Profile>>;

    /// NextProfile method
    fn next_profile(&self) -> zbus::Result<()>;

    /// Profile, get the active profile
    fn active_profile(&self) -> zbus::Result<Profile>;

    /// Set the specific profile as active
    fn set_active_profile(&self, profile: Profile) -> zbus::Result<()>;

    /// Get enabled fan curves
    fn enabled_fan_profiles(&self) -> zbus::Result<Vec<Profile>>;

    /// Get the active `Profile` data
    fn active_fan_data(&self) -> zbus::Result<FanCurveSet>;

    /// Get all fan curve data
    fn fan_curves(&self) -> zbus::Result<Vec<FanCurveSet>>;

    /// Set a fan curve. If a field is empty then the exisiting saved curve is used
    fn set_fan_curve(&self, curve: FanCurveSet) -> zbus::Result<()>;

    /// NotifyProfile signal
    #[dbus_proxy(signal)]
    fn notify_profile(&self, profile: Profile) -> zbus::Result<()>;
}

pub struct ProfileProxy<'a>(DaemonProxy<'a>);

impl<'a> ProfileProxy<'a> {
    #[inline]
    pub fn new(conn: &Connection) -> Result<Self> {
        Ok(ProfileProxy(DaemonProxy::new(conn)?))
    }

    #[inline]
    pub fn proxy(&self) -> &DaemonProxy<'a> {
        &self.0
    }

    #[inline]
    pub fn profiles(&self) -> Result<Vec<Profile>> {
        self.0.profiles()
    }

    #[inline]
    pub fn next_profile(&self) -> Result<()> {
        self.0.next_profile()
    }

    #[inline]
    pub fn connect_notify_profile(&self, send: Sender<Profile>) -> zbus::fdo::Result<()> {
        self.0.connect_notify_profile(move |data| {
            send.send(data)
                .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?;
            Ok(())
        })
    }
}
