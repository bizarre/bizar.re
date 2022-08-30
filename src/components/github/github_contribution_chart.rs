use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use dioxus::{prelude::*, router::*};
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr, iter::empty};

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct GithubContributionChartDataItem {
    date: String,
    count: i64,
    color: String,
    intensity: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct GithubContributionsChartData {
    pub contributions: Vec<GithubContributionChartDataItem>,
}

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    github_username: &'a str,
    contributions: &'a UseState<Option<i64>>,
}

trait Colorable {
    fn get_color(self) -> String;
}

impl Colorable for i64 {
    fn get_color(self) -> String {
        match self {
            4 => "#7D39D3",
            3 => "#5E26A6",
            2 => "#390081",
            1 => "#2B0061",
            _ => "#19171C",
        }
        .to_owned()
    }
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    log::info!("hi");
    let current_time = Utc::now().naive_local().date();
    let username = cx.props.github_username.to_owned();
    let chart_data = use_state(&cx, || {
        None as Option<HashMap<String, GithubContributionChartDataItem>>
    });

    let raw_chart_data = use_state(&cx, || None as Option<GithubContributionsChartData>);
    let contributions = cx.props.contributions;

    use_future(&cx, (raw_chart_data,), |(raw_chart_data,)| async move {
        if raw_chart_data.is_some() {
            return;
        }

        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/api/v1/{}", "https://ghcc-bzr.vercel.app", username))
            .send()
            .await
            .unwrap()
            .json::<GithubContributionsChartData>()
            .await;

        raw_chart_data.set(Some(res.unwrap()));
    });

    let end_of_current_year =
        NaiveDate::from_ymd(current_time.year() + 1, 1, 1) - Duration::days(1);

    let offset = (end_of_current_year - current_time).num_days();

    use_future(
        &cx,
        (chart_data, contributions, raw_chart_data),
        |(chart_data, contributions, raw_chart_data)| async move {
            if chart_data.is_some() || contributions.is_some() || raw_chart_data.is_none() {
                return;
            }

            let mut to_set: HashMap<String, GithubContributionChartDataItem> = HashMap::new();
            let mut count: i64 = 0;
            raw_chart_data
                .get()
                .as_ref()
                .unwrap()
                .contributions
                .iter()
                .skip(offset as usize)
                .take(52 * 7)
                .for_each(|item| {
                    to_set.insert(item.date.to_owned(), item.clone());
                    count += item.count;
                });

            chart_data.set(Some(to_set));
            contributions.set(Some(count));
        },
    );

    let end_of_current_week = NaiveDate::from_isoywd(
        current_time.year(),
        current_time.iso_week().week(),
        chrono::Weekday::Sat,
    );
    let chart_contents = { 
        cx.render(rsx! {(0..52).rev().map(|week| {
            rsx! {
                    div {
                        key: "{week}-week",
                        class: "grid gap-0.5 relative",
                        style: "grid-template-rows: repeat(7, minmax(0, 1fr));",
                    (0..7).rev().map(|day| {
                        let days = week*7 + day;
                        let current = NaiveDate::from(end_of_current_week - Duration::days(days));
                        let formatted = current.format("%Y-%m-%d").to_string();
                        let data = chart_data.get().as_ref(); 

                        let loading_opacity = if data.is_some() { "opacity-0" } else { "opacity-100" };
                        let loading = rsx!{
                            div {
                              class: "absolute w-full rounded-sm skeleton duration-500 transition-all {loading_opacity} pointer-events-none",  
                              padding_bottom: "100%",
                            }
                        };

                            if data.is_none() || !data.unwrap().contains_key(&formatted) {
                                let color = 0.get_color();
                                rsx! {
                                    div {
                                        key: "{days}-box",
                                        class: "relative w-full",
                                        padding_bottom: "100%",
                                        div {
                                              background_color: "{color}",
                                                class: "w-full rounded-sm transition-all transform duration-1000 delay-500 absolute",
                                                padding_bottom: "100%"
                                        }
                                        loading
                                        
                                    }
                                }
                            } else {                      
                                let item = data.unwrap().get(&formatted);
                                let item = item.unwrap();
                                let intensity = &item.intensity;
                                let color = intensity.parse::<i64>().unwrap().get_color();
                                let delay =((52 - week) + day) * 30;

                                if intensity == "0" {
                               rsx! {
                                div {
                                    key: "{days}-box",
                                    class: "relative",
                                    div {
                                        background_color: "{color}",
                                        class: "w-full rounded-sm transition-all delay-500 duration-1000 absolute",
                                        padding_bottom: "100%"
                                    }
                                    loading
                                    
                                
                                }}
                                } else {
                               rsx! {
                                div {
                                    key: "{days}-box",
                                    class: "relative",
                                    div {
                                        transition_delay: "{delay}ms",
                                        background_color: "{color}",
                                        class: "w-full rounded-sm transition-all transform duration-1000 hover:delay-0 hover:duration-150 cursor-pointer hover:opacity-50 hover:scale-110 absolute",
                                        padding_bottom: "100%",
                                        box_shadow: "inset 0px 0px 0px 1px rgba(255,255,255,0.06)"
                                    }
                                    loading
                                    
                                    
                                }
                                }
                                }

                            }
                    })
                }
            }
        })}) };

    let colors: Vec<String> = (0..5).map(|x| x.get_color()).collect();

    cx.render(rsx! {
        div {
            class: "relative",
            div {
                class: "grid w-full gap-0.5 relative",
                style: "grid-template-columns: repeat(52, minmax(0, 1fr));",
                chart_contents
                div {
                        class: "absolute -left-0.5 top-0 -ml-8 hidden lg:grid lg:text-xs text-slate text-right h-full",
                        style: "griw-row: span 7; grid-template-columns: repeat(1, minmax(0, 1fr)); grid-template-rows: repeat(7, minmax(0, 1fr));",
                        vec!["Mon", "Wed", "Fri"].iter().enumerate().map(|(i, day)| { 
                            let x = match i {
                                0 => 1,
                                1 => 3,
                                2 => 5,
                                _ => 0
                            }; rsx!{
                            div {
                                key: "{day}",
                                style: "grid-row: {x};",
                                class: "mt-1.5 mb-px",
                                label {
                                    "{day}"
                                }
                            }
                        }})
                }
            }
            div {
                class: "absolute left-0 -top-5 w-full hidden lg:block",
                div {
                    class: "grid gap-0.5 justify-items-end relative select-none cursor-default items-center relative",
                    style: "grid-template-columns: repeat(52, minmax(0, 1fr));",
                    div {
                        class: "relative",
                        style: "grid-column: span 39; font-size: 0.25em;",
                    }
                    div {
                        class: "text-slate text-xs mr-2",
                        style: "grid-column: span 4;", 
                        "Less"
                    }
                    colors.iter().enumerate().map(|(i, x)| {
                        let box_shadow = if i > 0 { "inset 0px 0px 0px 1px rgba(255,255,255,0.06)" } else {""};
                        rsx!{
                            div {
                                key: "{i}-dummy-box",
                                background_color: "{x}",
                                class: "w-full rounded-sm",
                                padding_bottom: "100%",
                                box_shadow: "{box_shadow}"
                            }
                        }
                    })
                    div {
                       class: "text-slate select-none cursor-default text-xs",
                       style: "grid-column: span 4;", 
                       "More"
                    }
                }
            }
        }
    })
}
