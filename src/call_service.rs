use log::{info,debug};
use pyprov::Service;


async fn call_service() {
    env_logger::init();
    let service = Service::try_init(None).await.expect("init");
    debug!("config: {:?}", &service.0);

    let n = Value::I32(10);
    let buf = to_vec(&n).unwrap();
    let res: i32 = from_slice(&service.invoke("f.factorial", &buf).await.unwrap()).unwrap();

}