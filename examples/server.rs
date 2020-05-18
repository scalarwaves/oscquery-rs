use ::atomic::Atomic;
use oscquery::param::*;
use oscquery::root::Root;
use oscquery::service::http::HttpService;
use oscquery::value::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

fn main() {
    let root = Arc::new(Root::new(Some("server example".into())));
    let c = oscquery::node::Container::new("foo".into(), Some("description of foo".into()));
    assert!(c.is_ok());
    let res = root.add_node(c.unwrap().into(), None);
    assert!(res.is_ok());

    let a = Arc::new(Atomic::new(2084i32));
    let m = oscquery::node::GetSet::new(
        "bar".into(),
        None,
        vec![ParamGetSet::Int(
            ValueBuilder::new(a.clone() as _)
                .with_unit("speed.mph".into())
                .build(),
        )],
        Some(Box::new(move |params, address, time| {
            println!("handler got {:?} {:?} {:?}", params, address, time);
            true
        })),
    );

    let mut osc = root.spawn_osc("127.0.0.1:0").unwrap();
    osc.add_send_addr(SocketAddr::from_str("127.0.0.1:3010").unwrap());

    let ws = root.spawn_ws("127.0.0.1:3002").unwrap();

    let _handle = HttpService::new(
        root.clone(),
        &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
    );

    std::thread::sleep(std::time::Duration::from_secs(10));
    let parent = res.unwrap();
    let res = root.add_node(m.unwrap().into(), Some(parent.clone()));
    assert!(res.is_ok());

    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = root.rm_node(parent.clone());
    assert!(res.is_ok());

    //can remove a second time
    let res = root.rm_node(parent);
    assert!(res.is_err());

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if let Some(msg) = osc.trigger_path("/foo/bar") {
            ws.send(msg);
        }
    }
}
