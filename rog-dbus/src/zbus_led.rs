//! # DBus interface proxy for: `org.asuslinux.Daemon`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/asuslinux/Aura' from service 'org.asuslinux.Daemon' on system bus`.
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
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use std::collections::BTreeMap;

use zbus::{blocking::Connection, Result};
use zbus_macros::dbus_proxy;

use rog_aura::{usb::AuraPowerDev, AuraEffect, AuraModeNum, LedBrightness, PerKeyRaw};

const BLOCKING_TIME: u64 = 40; // 100ms = 10 FPS, max 50ms = 20 FPS, 40ms = 25 FPS

#[dbus_proxy(
    interface = "org.asuslinux.Daemon",
    default_path = "/org/asuslinux/Aura"
)]
trait Led {
    /// NextLedMode method
    fn next_led_mode(&self) -> zbus::Result<()>;

    /// PrevLedMode method
    fn prev_led_mode(&self) -> zbus::Result<()>;

    /// Toggle to next led brightness
    fn next_led_brightness(&self) -> zbus::Result<()>;

    /// Toggle to previous led brightness
    fn prev_led_brightness(&self) -> zbus::Result<()>;

    /// SetBrightness method
    fn set_brightness(&self, brightness: LedBrightness) -> zbus::Result<()>;

    /// SetLedMode method
    fn set_led_mode(&self, effect: &AuraEffect) -> zbus::Result<()>;

    fn set_leds_power(&self, options: AuraPowerDev, enabled: bool) -> zbus::Result<()>;

    fn per_key_raw(&self, data: PerKeyRaw) -> zbus::fdo::Result<()>;

    /// NotifyLed signal
    #[dbus_proxy(signal)]
    fn notify_led(&self, data: AuraEffect) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn notify_power_states(&self, data: AuraPowerDev) -> zbus::Result<()>;

    /// LedBrightness property
    #[dbus_proxy(property)]
    fn led_brightness(&self) -> zbus::Result<i16>;

    /// LedMode property
    fn led_mode(&self) -> zbus::Result<AuraModeNum>;

    /// LedModes property
    fn led_modes(&self) -> zbus::Result<BTreeMap<AuraModeNum, AuraEffect>>;

    // As property doesn't work for AuraPowerDev (complexity of serialization?)
    // #[dbus_proxy(property)]
    fn leds_enabled(&self) -> zbus::Result<AuraPowerDev>;
}

pub struct LedProxyPerkey<'a>(LedProxyBlocking<'a>);

impl<'a> LedProxyPerkey<'a> {
    #[inline]
    pub fn new(conn: &Connection) -> Result<Self> {
        Ok(LedProxyPerkey(LedProxyBlocking::new(conn)?))
    }

    #[inline]
    pub fn proxy(&self) -> &LedProxyBlocking<'a> {
        &self.0
    }

    /// Write a single colour block.
    ///
    /// Intentionally blocks for 10ms after sending to allow the block to
    /// be written to the keyboard EC. This should not be async.
    #[inline]
    pub fn set_per_key(&self, per_key_raw: PerKeyRaw) -> Result<()> {
        self.0.per_key_raw(per_key_raw)?;
        std::thread::sleep(std::time::Duration::from_millis(BLOCKING_TIME));
        Ok(())
    }
}
