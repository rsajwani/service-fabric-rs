// ------------------------------------------------------------
// Copyright 2022 Youyuan Wu
// Licensed under the MIT License (MIT). See License.txt in the repo root for
// license information.
// ------------------------------------------------------------

// this contains some experiments for async
#![allow(non_snake_case)]

use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use fabric_base::FabricCommon::{
    FabricClient::FabricCreateLocalClient, IFabricAsyncOperationCallback,
    IFabricAsyncOperationCallback_Impl, IFabricAsyncOperationContext,
};
use tokio::sync::oneshot::Receiver;
use windows::core::implement;
use windows_core::ComInterface;

// fabric code begins here

// Creates the local client
pub fn CreateLocalClient<T: ComInterface>() -> T {
    unsafe { T::from_raw(FabricCreateLocalClient(&T::IID).expect("cannot get localclient")) }
}

pub trait Callback: FnOnce(::core::option::Option<&IFabricAsyncOperationContext>) {}
impl<T: FnOnce(::core::option::Option<&IFabricAsyncOperationContext>)> Callback for T {}

// TODO: rename.
// Fabric Callback that wraps an arbitrary Fn closure.
// Used primarily for bridging Begin and End fabric functions.
#[implement(IFabricAsyncOperationCallback)]
pub struct AwaitableCallback2<F>
where
    F: Callback,
{
    callback: Cell<Option<F>>,
}

impl<F: Callback> IFabricAsyncOperationCallback_Impl for AwaitableCallback2<F> {
    // notify the function has been invoked.
    fn Invoke(&self, context: ::core::option::Option<&IFabricAsyncOperationContext>) {
        let cb_opt = self.callback.take();
        match cb_opt {
            Some(cb) => {
                cb(context);
            }
            None => {
                unreachable!("Invoke has been run already");
            }
        }
    }
}

impl<F: Callback> AwaitableCallback2<F> {
    pub fn i_new(callback: F) -> IFabricAsyncOperationCallback {
        let a = AwaitableCallback2 {
            callback: Cell::new(Some(callback)),
        };
        a.into()
    }
}

// Token that wraps oneshot receiver.
// The future recieve does not have error.
pub struct FabricReceiver<T> {
    rx: tokio::sync::oneshot::Receiver<T>,
}

impl<T> FabricReceiver<T> {
    pub fn new(rx: tokio::sync::oneshot::Receiver<T>) -> FabricReceiver<T> {
        FabricReceiver { rx }
    }

    pub fn blocking_recv(self) -> T {
        // sender must send stuff so that there is not error.
        self.rx.blocking_recv().unwrap()
    }
}

// The future differs from tokio oneshot that it will not error when awaited.
impl<T> Future for FabricReceiver<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try to receive the value from the sender
        let innner = <Receiver<T> as Future>::poll(Pin::new(&mut self.rx), _cx);
        match innner {
            Poll::Ready(x) => {
                // error only happens when sender is dropped without sending.
                // we ignore this error since in sf-rs use this will never happen.
                Poll::Ready(x.unwrap())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// Send Box. Wrap a type and implement send.
// c pointers are not send in rust, so this forces it.
#[derive(Debug)]
pub struct SBox<T> {
    pub b: Box<T>,
}

// We know that T is send. This requires programmer's check of the internals.
unsafe impl<T> Send for SBox<T> {}

impl<T> SBox<T> {
    pub fn new(x: T) -> SBox<T> {
        SBox { b: Box::new(x) }
    }
}

#[cfg(test)]
mod tests {

    use std::cell::Cell;

    use fabric_base::{
        FabricCommon::{
            FabricClient::{
                IFabricClusterManagementClient3, IFabricGetNodeListResult, IFabricQueryClient,
            },
            IFabricAsyncOperationCallback, IFabricAsyncOperationCallback_Impl,
            IFabricAsyncOperationContext,
        },
        FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION, FABRIC_CLUSTER_HEALTH_POLICY,
        FABRIC_NODE_QUERY_DESCRIPTION, FABRIC_NODE_QUERY_RESULT_ITEM,
    };

    use tokio::sync::oneshot::{self, Sender};
    use windows::core::implement;
    use windows_core::{ComInterface, HSTRING};

    use super::{CreateLocalClient, FabricReceiver, SBox};

    use super::AwaitableCallback2;

    // This is implement a call back the supports rust .await syntax
    #[implement(IFabricAsyncOperationCallback)]
    pub struct AwaitableCallback {
        tx: Cell<Option<Sender<()>>>,
    }

    impl AwaitableCallback {
        pub fn channel() -> (AwaitableToken, IFabricAsyncOperationCallback) {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            let callback: AwaitableCallback = AwaitableCallback::new(tx);
            let token = AwaitableToken::new(rx);
            let i_callback: IFabricAsyncOperationCallback = callback.into();
            (token, i_callback)
        }

        fn new(tx: tokio::sync::oneshot::Sender<()>) -> AwaitableCallback {
            AwaitableCallback {
                tx: Cell::new(Some(tx)),
            }
        }
    }

    impl IFabricAsyncOperationCallback_Impl for AwaitableCallback {
        // notify the function has been invoked.
        fn Invoke(&self, _context: ::core::option::Option<&IFabricAsyncOperationContext>) {
            let tx = self.tx.take();
            let txx = tx.expect("tx is empty"); // This means invoke is called twice.
            txx.send(()).expect("fail to send");
        }
    }

    type AwaitableToken = super::FabricReceiver<()>;

    macro_rules! beginmyclient {
    ($name: ident) => {
        paste::item! {
        pub struct $name {
            com: fabric_base::FabricCommon::FabricClient::[<I $name>],
        }
        }

        // both are needed. But should be safe because COM ptr always lives on heap.
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> $name {
                return $name {
                    com: paste::item! {
                            crate::sync::CreateLocalClient::<fabric_base::FabricCommon::FabricClient::[<I $name>]>()
                        }
                };
            }
        } // impl
    };
}

    // macros for impl async fn
    macro_rules! myasyncfunc {
    ($fn_name: ident, $inner_name: ident, $param: ty, $res : ty, $( $param_opt:ty ),*) => {
    paste::item! {
        #[allow(unused)] // in test pkg the function might not be used by only for compile check
        pub async fn $fn_name(&self, p: SBox<$param>
        // optional params
        $(
            , [<$param_opt _name>]: SBox<$param_opt>
        )*

        ) -> ::windows::core::Result<$res> {
            let ctx: SBox<IFabricAsyncOperationContext>;
            let token: AwaitableToken;

            {
                let (token_inner,callback) = AwaitableCallback::channel();
                // make token accessible outside
                token = token_inner;

                {
                    let callback_arg: IFabricAsyncOperationCallback =
                        callback.cast().expect("castfailed");

                    paste::item! {
                    ctx = SBox::new(unsafe {
                        self.com
                        .[<Begin $inner_name>](
                            $(
                                [<$param_opt _name>].b.as_ref(),
                            )*
                            p.b.as_ref(), 1000, &callback_arg)?
                    });
                    }
                }
            }

            // await for async operation.
            token.await;

            paste::item! {
            unsafe { self.com.[<End $inner_name>](&(*ctx.b)) }
            }
        }
    }
    };
}

    beginmyclient!(FabricHealthClient);

    impl FabricHealthClient {
        myasyncfunc!(
            get_cluster_health,
            GetClusterHealth,
            fabric_base::FABRIC_CLUSTER_HEALTH_POLICY,
            fabric_base::FabricCommon::FabricClient::IFabricClusterHealthResult,
        );
        // get node health does not work because it requires node id as additional argument
        myasyncfunc!(
            get_node_health,
            GetNodeHealth,
            fabric_base::FABRIC_CLUSTER_HEALTH_POLICY,
            fabric_base::FabricCommon::FabricClient::IFabricNodeHealthResult,
            HSTRING
        );
        // the u16 is likely wrong. Maybe need to write a url type and convert to const ptr.
        myasyncfunc!(
            get_application_health,
            GetApplicationHealth,
            fabric_base::FABRIC_APPLICATION_HEALTH_POLICY,
            fabric_base::FabricCommon::FabricClient::IFabricApplicationHealthResult,
            u16 // applicationName
        );
        myasyncfunc!(
            get_service_health,
            GetServiceHealth,
            fabric_base::FABRIC_APPLICATION_HEALTH_POLICY,
            fabric_base::FabricCommon::FabricClient::IFabricServiceHealthResult,
            u16 // serviceName
        );
    }

    pub struct FabricQueryClient {
        com: IFabricQueryClient,
    }

    impl Default for FabricQueryClient {
        fn default() -> Self {
            Self::new()
        }
    }

    impl FabricQueryClient {
        pub fn new() -> FabricQueryClient {
            FabricQueryClient {
                com: CreateLocalClient::<IFabricQueryClient>(),
            }
        }

        myasyncfunc!(
            get_node_list,
            GetNodeList,
            FABRIC_NODE_QUERY_DESCRIPTION,
            IFabricGetNodeListResult,
        );

        myasyncfunc!(
            get_application_type_list,
            GetApplicationTypeList,
            fabric_base::FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION,
            fabric_base::FabricCommon::FabricClient::IFabricGetApplicationTypeListResult,
        );

        myasyncfunc!(
            get_service_type_list,
            GetServiceTypeList,
            fabric_base::FABRIC_SERVICE_TYPE_QUERY_DESCRIPTION,
            fabric_base::FabricCommon::FabricClient::IFabricGetServiceTypeListResult,
        );
        // example of not using macro.
        // param is SBox because it crosses await boundary.
        pub async fn get_node_list_example(
            &self,
            p: SBox<FABRIC_NODE_QUERY_DESCRIPTION>,
        ) -> ::windows::core::Result<IFabricGetNodeListResult> {
            let ctx: SBox<IFabricAsyncOperationContext>;
            let token: AwaitableToken;

            {
                let (token_inner, callback) = AwaitableCallback::channel();
                // make token accessible outside
                token = token_inner;

                {
                    ctx = SBox::new(unsafe {
                        self.com.BeginGetNodeList(p.b.as_ref(), 1000, &callback)?
                    });
                }
            }
            // await for async operation.
            token.await;
            unsafe { self.com.EndGetNodeList(&(*ctx.b)) }
        }

        pub fn get_node_list_example2(
            &self,
            querydescription: &FABRIC_NODE_QUERY_DESCRIPTION,
        ) -> FabricReceiver<::windows::core::Result<IFabricGetNodeListResult>> {
            let (tx, rx) = oneshot::channel();

            let callback = AwaitableCallback2::i_new(move |ctx| {
                let res = unsafe { self.com.EndGetNodeList(ctx) };
                if tx.send(res).is_err() {
                    // This can happen if user on the receiver end use cancel or select.
                    // Ideally user should always wait for result.
                    debug_assert!(false, "Receiver is dropped.");
                }
            });
            let ctx = unsafe { self.com.BeginGetNodeList(querydescription, 1000, &callback) };
            if ctx.is_err() {
                let (tx2, rx2) = oneshot::channel();
                tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2"); // This should never fail since rx2 is available
                FabricReceiver::new(rx2)
            } else {
                FabricReceiver::new(rx)
            }
        }

        pub fn get_node_list_sync_example(
            &self,
            querydescription: &FABRIC_NODE_QUERY_DESCRIPTION,
        ) -> ::windows::core::Result<IFabricGetNodeListResult> {
            let rx = self.get_node_list_example2(querydescription);
            rx.blocking_recv()
        }
    }

    async fn get_node(id: i32) {
        println!("id {}: GetNodeCli", id);

        let c = FabricQueryClient::new();

        let querydescription = SBox::new(FABRIC_NODE_QUERY_DESCRIPTION::default());

        let result = c.get_node_list_example(querydescription).await;

        assert!(!result.is_err());

        let result_node_list = result.expect("endcall_failed");

        let nodes = unsafe { result_node_list.get_NodeList() };
        let node_count = unsafe { (*nodes).Count };
        let node_list = unsafe { (*nodes).Items };

        println!("id {}: node_count {}", id, node_count);

        if !node_list.is_null() {
            let node: FABRIC_NODE_QUERY_RESULT_ITEM = unsafe { *node_list };
            println!("id {}: node info: name: {}", id, unsafe {
                node.NodeName.display()
            });
        }
    }

    async fn get_node2(id: i32) {
        println!("id {}: GetNodeCli", id);

        let c = FabricQueryClient::new();

        let result;

        {
            let querydescription = FABRIC_NODE_QUERY_DESCRIPTION::default();
            result = c.get_node_list_example2(&querydescription);
        }

        let send_result = result.await;
        let result_node_list = send_result.expect("fabric failure");

        let nodes = unsafe { result_node_list.get_NodeList() };
        let node_count = unsafe { (*nodes).Count };
        let node_list = unsafe { (*nodes).Items };

        println!("id {}: node_count {}", id, node_count);

        if !node_list.is_null() {
            let node: FABRIC_NODE_QUERY_RESULT_ITEM = unsafe { *node_list };
            println!("id {}: node info: name: {}", id, unsafe {
                node.NodeName.display()
            });
        }
    }

    fn sync_get_node() {
        // check sync api is ok.
        let c = FabricQueryClient::new();
        let querydescription = FABRIC_NODE_QUERY_DESCRIPTION::default();
        let _nodes2 = c.get_node_list_sync_example(&querydescription);
        println!("sync_get_node finished");
    }

    async fn get_stuff() {
        // do get applicationtype
        let c = FabricQueryClient::new();
        {
            let query_description = SBox::new(FABRIC_APPLICATION_TYPE_QUERY_DESCRIPTION::default());
            let result = c.get_application_type_list(query_description).await;
            let app_types = result.expect("cannot get types");
            let _list = unsafe { app_types.get_ApplicationTypeList() };
            // assert_eq!(unsafe { (*list).Count }, 0);
        }

        // get health state
        let h = FabricHealthClient::new();
        {
            let q = SBox::new(FABRIC_CLUSTER_HEALTH_POLICY::default());
            let result = h.get_cluster_health(q).await;
            let health = result.expect("cannto get health");
            let health_ptr = unsafe { health.get_ClusterHealth() };
            let _state = unsafe { (*health_ptr).AggregatedHealthState };
            // assert_eq!(FABRIC_HEALTH_STATE_OK, state);
        }
    }

    // requires local cluster to be running.
    #[test]
    fn fabricclient_test() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let handle = tokio::spawn(async {
                get_node(1).await;
            });

            let handle2 = tokio::spawn(async {
                get_node2(2).await;
            });

            handle.await.expect("handle wait");
            handle2.await.expect("handle2 wait");

            get_stuff().await;
        });

        sync_get_node();
    }

    #[test]
    fn local_client_create() {
        let _mgmt = CreateLocalClient::<IFabricClusterManagementClient3>();
    }
}
