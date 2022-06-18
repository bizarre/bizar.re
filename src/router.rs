use crate::layout::DefaultLayout;
use dioxus::{
    prelude::*,
    router::{Route, Router},
};

use crate::pages::{ProgrammerPage, PhotographyPage};

pub fn route(cx: Scope) -> Element {
    return cx.render(rsx!(Router {
        Route {
            to: "/", DefaultLayout { ProgrammerPage {  } }
        }
        Route {
            to: "/photography", DefaultLayout { PhotographyPage {  } }
        }
    }));
}
