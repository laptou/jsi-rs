use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use anyhow::bail;
use tokio::sync::Mutex;

use crate::*;

// using Rc and RefCell insted of Arc and Mutex b/c JS is single-threaded
pub type Subscriptions<'rt, K> = Rc<RefCell<BTreeMap<K, BTreeMap<usize, JsiFn<'rt>>>>>;

pub trait HostEventKey: FromStr + Ord + Clone + std::fmt::Debug + 'static {}

pub trait HostEvent<'rt> {
    type Key: HostEventKey;

    fn key(&self) -> Self::Key;
    fn args(self, rt: &mut RuntimeHandle<'rt>) -> Vec<JsiValue<'rt>>;
}

pub struct HostEventEmitter<'rt, E: HostEvent<'rt>> {
    rt: Arc<Mutex<RuntimeHandle<'rt>>>,
    ci: CallInvoker<'rt>,
    // probably doesn't need to be atomic b/c JS is single-threaded, but the
    // `self` reference in the functions is a shared reference, so we roll with
    // it
    next_id: AtomicUsize,
    subscriptions: Subscriptions<'rt, E::Key>,
}

impl<'rt, E: HostEvent<'rt>> HostEventEmitter<'rt, E> {
    pub fn new(rt: Arc<Mutex<RuntimeHandle<'rt>>>, ci: CallInvoker<'rt>) -> Self {
        Self {
            rt,
            ci,
            next_id: AtomicUsize::new(0),
            subscriptions: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn emit(&self, event: E) {
        let rt = &mut *self.rt.blocking_lock();

        let key = event.key();
        let args = event.args(rt);

        let subscriptions = self.subscriptions.borrow();

        #[cfg(feature = "host-object-trace")]
        log::trace!("emitting event {:?}", key);

        let key_format = format!("{:?}", key);

        if let Some(event_subscriptions) = subscriptions.get(&key) {
            for (_, callback) in event_subscriptions {
                let callback: JsiValue = callback.as_value(rt);
                let callback = rt.clone(&callback);
                let callback: JsiFn = FromValue::from_value(&callback, rt).unwrap();
                let args = rt.clone(&args);

                let key_format = key_format.clone();

                let _rt = self.rt.clone();
                self.ci.invoke_async(Box::new(move || {
                    #[cfg(feature = "host-object-trace")]
                    log::trace!("running callback for {:?}", key_format);

                    let rt = &mut *self.rt.blocking_lock();
                    callback.call(args, rt)?;

                    Ok(())
                }));
            }
        }
    }
}

#[host_object]
impl<'rt, E: HostEvent<'rt>> HostEventEmitter<'rt, E> {
    pub fn add_event_listener(
        &self,
        rt: &mut RuntimeHandle<'rt>,
        event: JsiString<'rt>,
        callback: JsiFn<'rt>,
    ) -> anyhow::Result<EventSubscription<'rt, E::Key>> {
        let event = rt.display(&event).to_string();
        let event: E::Key = if let Ok(event) = event.parse() {
            event
        } else {
            bail!("event name {} is not supported", event)
        };

        let mut subscriptions = self.subscriptions.borrow_mut();

        let event_subscriptions = subscriptions
            .entry(event.clone())
            .or_insert_with(BTreeMap::new);

        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let subscription = EventSubscription {
            id,
            event,
            subscriptions: self.subscriptions.clone(),
        };

        event_subscriptions.insert(id, callback);

        Ok(subscription)
    }

    fn to_string(&self, _rt: &mut RuntimeHandle<'rt>) -> anyhow::Result<&str> {
        Ok("[HostEventEmitter]")
    }
}

pub struct EventSubscription<'rt, K: HostEventKey> {
    event: K,
    id: usize,
    subscriptions: Subscriptions<'rt, K>,
}

#[host_object]
impl<'rt, K: HostEventKey> EventSubscription<'rt, K> {
    pub fn remove(&self, _rt: &mut RuntimeHandle<'rt>) -> anyhow::Result<()> {
        self.subscriptions
            .borrow_mut()
            .entry(self.event.clone())
            .and_modify(|s| {
                s.remove(&self.id);
            });

        Ok(())
    }

    fn to_string(&self, _rt: &mut RuntimeHandle<'rt>) -> anyhow::Result<&str> {
        Ok("[HostEventSubscription]")
    }
}
