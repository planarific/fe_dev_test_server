use crate::model::Model;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use std::sync::Mutex;

pub static DATA: Lazy<Mutex<HashMap<u32, Model>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Model {
                id: 1,
                address1: "24 Hesters Way Road".into(),
                address2: "".into(),
                city: "Cheltenham".into(),
                state: "Gloucestershire".into(),
                postal_code: "GL51 0DA".into(),
                thumbnail: "/thumbs/1.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house1.glb".into(),
            },
        ),
        (
            2,
            Model {
                id: 2,
                address1: "10 Beehive Court".into(),
                address2: "".into(),
                city: "Liversedge".into(),
                state: "West Yorkshire".into(),
                postal_code: "WF15 7BT".into(),
                thumbnail: "/thumbs/2.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house2.glb".into(),
            },
        ),
        (
            3,
            Model {
                id: 3,
                address1: "9 Vancouver Road".into(),
                address2: "".into(),
                city: "Worthing".into(),
                state: "West Sussex".into(),
                postal_code: "BN13 2SN".into(),
                thumbnail: "/thumbs/3.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house3.glb".into(),
            },
        ),
        (
            4,
            Model {
                id: 4,
                address1: "Hiram House, Spring Lane".into(),
                address2: "".into(),
                city: "Thrupp".into(),
                state: "Gloucestershire".into(),
                postal_code: "GL5 2DU".into(),
                thumbnail: "/thumbs/4.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house4.glb".into(),
            },
        ),
        (
            5,
            Model {
                id: 5,
                address1: "316B Cricklewood Lane".into(),
                address2: "".into(),
                city: "London".into(),
                state: "".into(),
                postal_code: "NW2 2QE".into(),
                thumbnail: "/thumbs/5.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house5.glb".into(),
            },
        ),
        (
            6,
            Model {
                id: 6,
                address1: "41 Tresawls Avenue".into(),
                address2: "".into(),
                city: "Truro".into(),
                state: "Cornwall".into(),
                postal_code: "TR1 3LA".into(),
                thumbnail: "/thumbs/6.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house6.glb".into(),
            },
        ),
        (
            7,
            Model {
                id: 7,
                address1: "9 Venus Street".into(),
                address2: "".into(),
                city: "Congresbury".into(),
                state: "Avon".into(),
                postal_code: "BS49 5HA".into(),
                thumbnail: "/thumbs/7.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house7.glb".into(),
            },
        ),
        (
            8,
            Model {
                id: 8,
                address1: "4 Wheatley Grove".into(),
                address2: "".into(),
                city: "Walsall".into(),
                state: "West Midlands".into(),
                postal_code: "WS6 6ES".into(),
                thumbnail: "/thumbs/8.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house8.glb".into(),
            },
        ),
        (
            9,
            Model {
                id: 9,
                address1: "6 Regent Court".into(),
                address2: "".into(),
                city: "Bagshot".into(),
                state: "Surrey".into(),
                postal_code: "GU19 5QD".into(),
                thumbnail: "/thumbs/9.png".into(),
                model: "https://planarificfedevtest.blob.core.windows.net/models/house9.glb".into(),
            },
        ),
    ]))
});
