use crate::layout::DefaultLayout;
use dioxus::{
    prelude::*,
    router::{Route, Router},
};

use crate::pages::{EngineerPage, PhotographyPage};

pub fn route(cx: Scope) -> Element {
    return cx.render(rsx!(Router {
        Route {
            to: "/", DefaultLayout { EngineerPage {  } }
        }
        Route {
            to: "/photography", DefaultLayout { PhotographyPage {  } }
        }
    }));
}
