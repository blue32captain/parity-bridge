macro_rules! try_bridge {
	($e: expr) => (match $e {
		Err(err) => return Err(From::from(err)),
		Ok($crate::futures::Async::NotReady) => None,
		Ok($crate::futures::Async::Ready(None)) => return Ok($crate::futures::Async::Ready(None)),
		Ok($crate::futures::Async::Ready(Some(value))) => Some(value),
	})
}

macro_rules! try_stream {
	($e: expr) => (match $e {
		Err(err) => return Err(From::from(err)),
		Ok($crate::futures::Async::NotReady) => return Ok($crate::futures::Async::NotReady),
		Ok($crate::futures::Async::Ready(None)) => return Ok($crate::futures::Async::Ready(None)),
		Ok($crate::futures::Async::Ready(Some(value))) => value,
	})
}
