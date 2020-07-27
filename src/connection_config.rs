/// NSQD TLS encryption options
#[derive(Debug, Clone)]
pub struct NSQConfigSharedTLS {
    required: bool
}

impl NSQConfigSharedTLS {
    pub fn new() -> Self {
        return NSQConfigSharedTLS {
            required: true,
        }
    }

    pub fn set_required(mut self, required: bool) -> Self {
        self.required = required;

        return self;
    }
}

/// NSQD TCP compression options
#[derive(Debug, Clone)]
pub enum NSQConfigSharedCompression {
    Deflate(u8)
}

/// Configuration options shared by both produces and consumers
#[derive(Debug, Clone)]
pub struct NSQConfigShared {
    pub(crate) backoff_max_wait:      std::time::Duration,
    pub(crate) backoff_healthy_after: std::time::Duration,
    pub(crate) compression:           Option<NSQConfigSharedCompression>,
    pub(crate) tls:                   Option<NSQConfigSharedTLS>,
    pub(crate) credentials:           Option<Vec<u8>>,
    pub(crate) client_id:             Option<String>,
}

impl NSQConfigShared {
    pub fn new() -> Self {
        return NSQConfigShared {
            backoff_max_wait:      std::time::Duration::new(60, 0),
            backoff_healthy_after: std::time::Duration::new(45, 0),
            compression:           None,
            tls:                   None,
            credentials:           None,
            client_id:             None,
        }
    }

    pub fn set_backoff_max_wait(mut self, duration: std::time::Duration) -> Self {
        self.backoff_max_wait = duration;

        return self;
    }

    pub fn set_backoff_healthy_after(mut self, duration: std::time::Duration) -> Self {
        self.backoff_healthy_after = duration;

        return self;
    }

    pub fn set_compression(mut self, compression: NSQConfigSharedCompression) -> Self {
        self.compression = Some(compression);

        return self;
    }

    pub fn set_credentials(mut self, credentials: Vec<u8>) -> Self {
        self.credentials = Some(credentials);

        return self;
    }

    pub fn set_tls(mut self, tls: NSQConfigSharedTLS) -> Self {
        self.tls = Some(tls);

        return self;
    }

    pub fn set_client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.client_id = Some(client_id.into());

        return self;
    }
}
