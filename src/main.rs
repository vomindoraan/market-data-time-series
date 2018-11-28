#[macro_use]
extern crate influx_db_client;
extern crate zmq;

use influx_db_client::{Client, Point, Points, Value, Precision};

fn main() {
    // default with "http://127.0.0.1:8086", db with "test"
    let client = Client::default().set_authentication("admin", "admin");

    let mut point = point!("test1");
    point
        .add_field("foo", Value::String("bar".to_string()))
        .add_field("integer", Value::Integer(11))
        .add_field("float", Value::Float(22.3))
        .add_field("'boolean'", Value::Boolean(false));

    let point1 = Point::new("test1")
        .add_tag("tags", Value::String(String::from("\\\"fda")))
        .add_tag("number", Value::Integer(12))
        .add_tag("float", Value::Float(12.6))
        .add_field("fd", Value::String("'3'".to_string()))
        .add_field("quto", Value::String("\\\"fda".to_string()))
        .add_field("quto1", Value::String("\"fda".to_string()))
        .to_owned();

    let points = points!(point1, point);

    // if Precision is None, the default is second
    // Multiple write
    let _ = client.write_points(points, Some(Precision::Seconds), None).unwrap();

    // query, it's type is Option<Vec<Node>>
    let res = client.query("select * from test1", None).unwrap();
    println!("{:?}", res.unwrap()[0].series);

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.set_subscribe(b"MDT");
    socket.connect("tcp://mt5proxy.tradecore.io:8149").unwrap();
    loop {
        let msg = socket.recv_multipart(0).unwrap();
        println!("{:?}", msg);
    }
}
