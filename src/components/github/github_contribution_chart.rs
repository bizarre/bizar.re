use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use dioxus::{prelude::*, router::*};
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct GithubContributionChartDataItem {
    date: String,
    count: i64,
    color: String,
    intensity: String,
}

#[derive(Deserialize, Debug)]
struct GithubContributionsChartData {
    contributions: Vec<GithubContributionChartDataItem>,
}

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    github_username: &'a str,
    contributions: &'a UseState<Option<i64>>,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let route = use_route(&cx);
    let url = route.url();
    let host = url.to_string().replace(&route.url().path().to_string(), "");

    let current_time = Utc::now().naive_local().date();
    let username = cx.props.github_username.to_owned();
    let chart_data = use_state(&cx, || {
        None as Option<HashMap<String, GithubContributionChartDataItem>>
    });

    let map_dates = use_state(&cx, || {
        let mut to_return = Vec::new();

        for i in 1..=52 * 7 {
            to_return.push(
                NaiveDate::from(current_time - Duration::days(i))
                    .format("%Y-%m-%d")
                    .to_string(),
            );
        }

        to_return
            .iter()
            .rev()
            .map(|d| d.to_owned())
            .collect::<Vec<String>>()
    });

    let contributions = cx.props.contributions;

    use_future(
        &cx,
        (map_dates, chart_data, contributions),
        |(map_dates, chart_data, contributions)| async move {
            if chart_data.is_some() {
                return;
            }

            let client = reqwest::Client::new();
            let res = client
                .get(format!("{}/api/v1/{}", host, username))
                .send()
                .await
                .unwrap()
                .json::<GithubContributionsChartData>()
                .await
                .expect("hi");

            let mut to_set: HashMap<String, GithubContributionChartDataItem> = HashMap::new();
            let mut count: i64 = 0;
            for item in res.contributions.iter().rev() {
                if map_dates.contains(&item.date) {
                    to_set.insert(item.date.to_owned(), item.clone());
                    count += item.count;
                }
            }

            chart_data.set(Some(to_set));
            contributions.set(Some(count));
        },
    );

    let current_time = Utc::now().naive_local().date();
    let end_of_current_week = NaiveDate::from_isoywd(
        current_time.year(),
        current_time.iso_week().week(),
        chrono::Weekday::Fri,
    );
    let chart_contents = match chart_data.get() {
        Some(data) => cx.render(rsx! {(0..52).rev().map(|week| {
            rsx! {
                    div {
                        key: "{week}-week",
                        class: "grid gap-0.5",
                        style: "grid-template-rows: repeat(7, minmax(0, 1fr));",
                    (0..7).rev().map(|day| {
                        let days = week*7 + day;
                        let current = NaiveDate::from(end_of_current_week.clone() - Duration::days(days));
                        if let Some(item) = data.get(&current.format("%Y-%m-%d").to_string()) {
                            let intensity = item.intensity.clone();
                            let color = match intensity.parse::<i32>() {
                                Ok(4) => "rgb(57, 211, 83)",
                                Ok(3) => "rgb(38, 166, 65)",
                                Ok(2) => "rgb(0, 109, 50)",
                                Ok(1) => "rgb(14, 68, 41)",
                                _ => "#151715"
                            };

                            rsx! {
                                div {
                                key: "{days}-box",
                                background_color: "{color}",
                                class: "w-full",
                                padding_bottom: "100%",
                                ""
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    key: "{days}-box",
                                    ""
                                }
                            }
                        }
                    })
                }
            }
        })}),
        None => cx.render(rsx! {
        (0..52*7).map(|n| rsx!{
            div {
                key: "{n}-box",
                class: "w-full skeleton",
                padding_bottom: "100%",
                ""
            }
        })
        }),
    };

    cx.render(rsx! {
        div {
            class: "relative",
            div {
                class: "grid w-full gap-0.5 relative",
                style: "grid-template-columns: repeat(52, minmax(0, 1fr));",
                chart_contents
            }
            div {
                class: "absolute left-0 top-0 h-12 w-4 -ml-6 text-dark",
                style: "font-size: 0.3em;",
                ul {
                    class: "flex flex-col items-end",
                    vec!["Mon", "Wed", "Fri"].iter().map(|day| rsx!{
                        li {
                            key: "{day}",
                            class: "mt-1.5 mb-0.5",
                            label {
                                "{day}"
                            }
                        }
                    })
                }
            }
        }
    })
}
