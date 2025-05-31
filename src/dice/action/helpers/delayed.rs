use std::future::Future;

use bevy_defer::{AccessError, AsyncWorld};

pub async fn delayed<T>(
  delay: f32,
  action: impl Future<Output = Result<T, AccessError> > + Send + 'static,
) -> Result<T, AccessError> {
  AsyncWorld.sleep(delay).await;
  action.await
}
