use std::{fmt::Debug, future::Future, pin::Pin, sync::{Arc, Mutex}};

use bevy::prelude::*;
use bevy_defer::{AccessError, AsyncAccess, AsyncWorld};

pub mod tooltip;

pub type DynAsyncFunction<E> =
  Arc<dyn Fn(Arc<Mutex<E>>) -> Pin<Box<dyn Future<Output = Result<(), AccessError>>>> + Send + Sync>;

#[derive(Resource, Clone)]
pub struct AsyncListeners<E: Event> {
  listeners: Vec<(ListenerID, DynAsyncFunction<E>)>,
  new_id: ListenerID,
}

impl<E: Event> Default for AsyncListeners<E> {
  fn default() -> Self {
    Self { listeners: default(), new_id: 0 }
  }
}

pub type ListenerID = usize;

impl<E: Event + Clone + Copy + Debug> AsyncListeners<E> {
  pub fn add_listener(&mut self, listener: DynAsyncFunction<E>) -> ListenerID {
    let id = self.new_id;
    self.new_id += 1;
    self.listeners.push((id, listener));
    id
  }

  #[allow(dead_code)]
  pub fn remove_listener(&mut self, id: ListenerID) {
    if let Ok(pos) = self.listeners.binary_search_by_key(&id, |(listener_id, _)| *listener_id) {
      self.listeners.remove(pos);
    }
  }

  pub async fn trigger_event(&self, event: Arc<Mutex<E>>) -> Result<(), AccessError> {
    info!("{:?}", event.get());
    for (_, listener) in &self.listeners {
      listener(event.clone()).await?;
    }
    Ok(())
  }
}

pub trait SyncEvents {
  async fn trigger_event<E: Event + Clone + Copy + Debug>(&self, event: Arc<Mutex<E>>) -> Result<(), AccessError>;
}

impl SyncEvents for AsyncWorld {
  async fn trigger_event<E: Event + Clone + Copy + Debug>(&self, event: Arc<Mutex<E>>) -> Result<(), AccessError> {
    let listeners = self
      .resource::<AsyncListeners<E>>()
      .cloned()?;
    listeners.trigger_event(event).await?;
    Ok(())
  }
}

pub trait ListenTo {
  fn listen_to<E: Event>(&mut self) -> &mut Self;
}

impl ListenTo for App {
  fn listen_to<E: Event>(&mut self) -> &mut Self {
    self.init_resource::<AsyncListeners<E>>()
  }
}

pub trait AddEventAndListen {
  fn add_event_and_listen<E: Event>(&mut self) -> &mut Self;
}

impl AddEventAndListen for App {
  fn add_event_and_listen<E: Event>(&mut self) -> &mut Self {
    self
      .add_event::<E>()
      .listen_to::<E>()
  }
}

pub trait RegisterListener {
  fn register_listener<E: Event + Clone + Copy + Debug, F, FOut>(&mut self, listener: F) -> &mut Self
  where 
    F: Fn(Arc<Mutex<E>>) -> FOut + Send + Sync + 'static,
    FOut: Future<Output = Result<(), AccessError>> + Send + 'static;

  fn register_dyn_listener<E: Event + Clone + Copy + Debug>(&mut self, listener: DynAsyncFunction<E>) -> &mut Self;
}

impl RegisterListener for App {
  fn register_listener<E: Event + Clone + Copy + Debug, F, FOut>(&mut self, listener: F) -> &mut Self
  where 
    F: Fn(Arc<Mutex<E>>) -> FOut + Send + Sync + 'static,
    FOut: Future<Output = Result<(), AccessError>> + Send + 'static,
  {
    self.register_dyn_listener(Arc::new(move |event| Box::pin(listener(event))))
  }

  fn register_dyn_listener<E: Event + Clone + Copy + Debug>(&mut self, listener: DynAsyncFunction<E>) -> &mut Self {
    let _ = self
      .world_mut()
      .resource_mut::<AsyncListeners<E>>()
      .add_listener(listener);
    self
  }
}

pub trait ArcMutex: Sized {
  fn wrap(self) -> Arc<Mutex<Self>> {
    Arc::new(Mutex::new(self))
  }
}

impl<E: Event> ArcMutex for E {}

pub trait ArcMutexMutable<E> {
  fn mutate<F>(&self, function: F)
  where
    F: FnOnce(E) -> E;

  fn get(&self) -> E;
}

impl<E: Event + Clone + Copy> ArcMutexMutable<E> for Arc<Mutex<E>> {
  fn mutate<F>(&self, function: F)
  where
    F: FnOnce(E) -> E
  {
    let mut current = self.lock().unwrap();
    *current = function(*current);
  }

  fn get(&self) -> E {
    *self.lock().unwrap()
  }
}
