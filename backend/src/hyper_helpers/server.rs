use hyper::service::{make_service_fn, service_fn};

pub trait ApplicationContext: Clone + Send + Sync + 'static {}

impl<T> ApplicationContext for T where T: Clone + Send + Sync + 'static {}

pub trait RequestHandlerReturnType:
    std::future::Future<
        Output = Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse>,
    > + Send
    + Sync
    + 'static
{
}

impl<T> RequestHandlerReturnType for T where
    T: std::future::Future<
            Output = Result<hyper::Response<hyper::Body>, crate::hyper_helpers::ErrorResponse>,
        > + Send
        + Sync
        + 'static
{
}

pub trait RequestHandlerType<AC, Fut>:
    FnOnce(hyper::Request<hyper::Body>, AC) -> Fut + Copy + Send + 'static
where
    AC: ApplicationContext,
    Fut: RequestHandlerReturnType,
{
}

impl<AC, Fut, T> RequestHandlerType<AC, Fut> for T
where
    AC: ApplicationContext,
    Fut: RequestHandlerReturnType,
    T: FnOnce(hyper::Request<hyper::Body>, AC) -> Fut + Copy + Send + 'static,
{
}

pub trait RequestHandlerDecoratorType<AC, Fut, RHT>:
    FnOnce(hyper::Request<hyper::Body>, AC, RHT) -> Fut + Copy + Send + 'static
where
    AC: ApplicationContext,
    Fut: RequestHandlerReturnType,
    RHT: RequestHandlerType<AC, Fut>,
{
}

impl<AC, Fut, RHT, T> RequestHandlerDecoratorType<AC, Fut, RHT> for T
where
    AC: ApplicationContext,
    Fut: RequestHandlerReturnType,
    RHT: RequestHandlerType<AC, Fut>,
    T: FnOnce(hyper::Request<hyper::Body>, AC, RHT) -> Fut + Copy + Send + 'static,
{
}

pub struct Server {
    halt_sender: tokio::sync::oneshot::Sender<()>,
    halted_receiver: tokio::sync::oneshot::Receiver<()>,
}

impl Server {
    pub fn block_start<AC, F, RT>(
        listener_address: std::net::SocketAddr,
        application_context: AC,
        router: F,
    ) where
        AC: ApplicationContext,
        F: RequestHandlerType<AC, RT>,
        RT: RequestHandlerReturnType,
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let application =
                Self::spawn_start(listener_address, application_context, router).await;

            tokio::signal::ctrl_c().await.unwrap();

            application.halt().await;
        });
    }

    pub async fn spawn_start<AC, F, RT>(
        listener_address: std::net::SocketAddr,
        application_context: AC,
        router: F,
    ) -> Self
    where
        AC: ApplicationContext,
        F: RequestHandlerType<AC, RT>,
        RT: RequestHandlerReturnType,
    {
        let (halt_sender, halt_receiver) = tokio::sync::oneshot::channel();
        let (halted_sender, halted_receiver) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            let service = make_service_fn(move |_| {
                let application_context = application_context.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |request| {
                        let application_context = application_context.clone();
                        Self::router_helper(router(request, application_context))
                    }))
                }
            });

            let server = hyper::Server::bind(&listener_address).serve(service);
            let graceful = server.with_graceful_shutdown(async {
                halt_receiver.await.unwrap();
                halted_sender.send(()).unwrap();
            });

            graceful.await.unwrap();
        });

        Self {
            halt_sender,
            halted_receiver,
        }
    }

    pub async fn halt(self) {
        self.halt_sender.send(()).unwrap();
        self.halted_receiver.await.unwrap();
    }

    async fn router_helper(
        task: impl RequestHandlerReturnType,
    ) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
        match task.await {
            Ok(ok_response) => Ok(ok_response),
            Err(err_response) => Ok(err_response.0),
        }
    }
}
