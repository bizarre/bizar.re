use crate::layout::DefaultLayout;
use dioxus::{
    prelude::*,
    router::{Route, Router},
};

use crate::pages::{JournalEntryPage, PhotographyPage, ProgrammerPage};

pub fn route(cx: Scope) -> Element {
    return cx.render(rsx!(Router {
        Route {
            to: "/photography", DefaultLayout { PhotographyPage {  } }
        }
        Route { to: "/:entry", DefaultLayout { JournalEntryPage {  } } }
        Route {
            to: "/", DefaultLayout { ProgrammerPage {  } }
        }
    }));
}
