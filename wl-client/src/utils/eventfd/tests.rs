macro_rules! tests {
    () => {
        use {
            crate::utils::poller::{Poller, readable},
            std::{sync::Arc, time::Duration},
            tokio::time::timeout,
        };

        #[tokio::test]
        async fn read() {
            let eventfd = Arc::new(super::Eventfd::new().unwrap());
            let poller = Poller::new(&eventfd).unwrap();
            let res = timeout(Duration::from_millis(500), readable(&poller.data)).await;
            assert!(res.is_err());
            eventfd.bump().unwrap();
            readable(&poller.data).await.unwrap();
            eventfd.clear().unwrap();
            eventfd.clear().unwrap();
            let res = timeout(Duration::from_millis(500), readable(&poller.data)).await;
            assert!(res.is_err());
        }
    };
}
