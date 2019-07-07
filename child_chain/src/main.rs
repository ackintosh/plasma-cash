extern crate ethereum_types;
extern crate futures;

use ethereum_types::Address;
use ethabi::{Event, EventParam, ParamType};
use event_watcher::event_db::DefaultEventDb;
use event_watcher::event_watcher::EventWatcher;

fn main() {
    let address: Address = match "dDA6327139485221633A1FcD65f4aC932E60A2e1".parse() {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    let abi: Vec<Event> = vec![
        Event {
            name: "Deposit".to_owned(),
            inputs: vec![
                EventParam {
                    name: "depositor".to_owned(),
                    kind: ParamType::Address,
                    indexed: false,
                },
                EventParam {
                    name: "amount".to_owned(),
                    kind: ParamType::Uint(256),
                    indexed: false,
                },
                EventParam {
                    name: "uid".to_owned(),
                    kind: ParamType::Uint(256),
                    indexed: false,
                }
            ],
            anonymous: false,
        }
    ];

    let db = DefaultEventDb::new();
    let mut watcher = EventWatcher::new("http://localhost:8545", address, abi, db);

    watcher.subscribe(Box::new(|log| {
        println!("{:?}", log);
    }));

    tokio::run(futures::future::lazy(|| {
        tokio::spawn(watcher);
        Ok(())
    }));
}
