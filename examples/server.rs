use ::atomic::Atomic;
use oscquery::func_wrap::*;
use oscquery::param::*;
use oscquery::root::OscQueryGraph;
use oscquery::value::*;
use oscquery::OscQueryServer;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    let root = OscQueryServer::new(
        Some("example".into()),
        &SocketAddr::from_str("127.0.0.1:3000").expect("failed to bind for http"),
        "127.0.0.1:0",
        "127.0.0.1:0",
    )?;

    println!(
        "http: {} osc: {} ws: {}",
        root.http_local_addr(),
        root.osc_local_addr(),
        root.ws_local_addr()
    );

    let c = oscquery::node::Container::new("foo".into(), Some("description of foo".into()))
        .expect("to construct foo");
    let parent_handle = root.add_node(c.into(), None).expect("to add foo");

    let a = Arc::new(Atomic::new(2084i32));
    let m = oscquery::node::GetSet::new(
        "bar".into(),
        None,
        vec![ParamGetSet::Int(
            ValueBuilder::new(a.clone() as _)
                .with_unit("speed.mph".into())
                .build(),
        )],
        Some(Box::new(UpdateFunc(
            move |params: &Vec<rosc::OscType>,
                  address: Option<SocketAddr>,
                  time: Option<(u32, u32)>| {
                {
                    println!("handler got {:?} {:?} {:?}", params, address, time);
                }
            },
        ))),
    );

    std::thread::sleep(std::time::Duration::from_secs(10));
    let _handle = root
        .add_node(m.unwrap().into(), Some(parent_handle))
        .expect("to add bar");

    //let p = parent_handle.clone();
    let m = oscquery::node::Set::new(
        "add".into(),
        None,
        vec![ParamSet::String(
            ValueBuilder::new(Arc::new(()) as _).build(),
        )],
        Some(Box::new(UpdateFunc2(
            move |_params: &Vec<rosc::OscType>,
                  _address: Option<SocketAddr>,
                  _time: Option<(u32, u32)>| {
                {
                    let name = "asdf"; //TODO
                    Some(Box::new(move |r: &mut dyn OscQueryGraph| {
                        let n = oscquery::node::Get::new(
                            name.into(),
                            None,
                            vec![ParamGet::Int(
                                ValueBuilder::new(Arc::new(Atomic::new(1i32)) as _).build(),
                            )],
                        )
                        .expect("failed to create node");
                        let _ = r.add_node(n.into(), None);
                    }))
                }
            },
        ))),
    );
    let _handle = root
        .add_node(m.unwrap().into(), Some(parent_handle))
        .expect("to add bar");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        root.trigger_path("/foo/bar");
    }
}
