
pub trait TryAll {
  type Ok;
  type Error;

  fn try_all(self) -> Result<Self::Ok, Self::Error>;
}

impl<T1, T2, E> TryAll for (Result<T1, E>, Result<T2, E>) {
  type Ok = (T1, T2);
  type Error = E;

  fn try_all(self) -> Result<Self::Ok, Self::Error> {
    match self {
      (Ok(v1), Ok(v2)) => Ok((v1, v2)),
      (Err(e), _) | (_, Err(e)) => Err(e),
    }
  }
}

impl<T1, T2, T3, E> TryAll for (Result<T1, E>, Result<T2, E>, Result<T3, E>) {
  type Ok = (T1, T2, T3);
  type Error = E;

  fn try_all(self) -> Result<Self::Ok, Self::Error> {
    match self {
      (Ok(v1), Ok(v2), Ok(v3)) => Ok((v1, v2, v3)),
      (Err(e), ..) | (_, Err(e), _) | (.., Err(e))=> Err(e),
    }
  }
}

