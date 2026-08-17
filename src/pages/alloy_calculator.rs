use leptos::{logging::error, prelude::*};
use vs_alloy_calculator::prelude::*;
use window_lib::{
    components::{Sprite, Window},
    constants::icons,
    Dimensions, SpriteData, WindowData,
};

#[component]
pub fn alloy_calculator(is_open: bool) -> impl IntoView {
    use alloy_names::*;
    let data = WindowData::new(icons::CRUCIBLE, "Vintage Story Alloy Calculator")
        .centered()
        .dimensions(Dimensions { w: 425.0, h: 250.0 })
        .open(is_open);
    let (alloy, set_alloy) = signal(Alloys::TinBronze);
    let num_ingots = RwSignal::new(1);
    let max_ingots = RwSignal::new(25);
    view! {
        <Window data id="alloy-calculator" disable_maximize=true>
            <div class="row">
                <div style="flex-grow:1;">
                    <label for="alloys">"Choose an Alloy: "</label>
                    <br/>
                    <select
                        name="alloys"
                        id="alloys"
                        on:change:target=move |ev| {
                            set_alloy(
                                match ev.target().value().as_str() {
                                    TIN_BRONZE => Alloys::TinBronze,
                                    BISMUTH_BRONZE => Alloys::BismuthBronze,
                                    BLACK_BRONZE => Alloys::BlackBronze,
                                    BRASS => Alloys::Brass,
                                    MOLYBDOCHALKOS => Alloys::Molybdochalkos,
                                    LEAD_SOLDER => Alloys::LeadSolder,
                                    SILVER_SOLDER => Alloys::SilverSolder,
                                    ELECTRUM => Alloys::Electrum,
                                    _ => Alloys::Cupronickel,
                                },
                            )
                        }
                    >

                        <optgroup label="Bronzes">
                            <option value=TIN_BRONZE>"Tin Bronze"</option>
                            <option value=BISMUTH_BRONZE>"Bismuth Bronze"</option>
                            <option value=BLACK_BRONZE>"Black Bronze"</option>
                        </optgroup>
                        <optgroup label="Simple Alloys">
                            <option value=BRASS>"Brass"</option>
                            <option value=MOLYBDOCHALKOS>"Molybdochalkos"</option>
                        </optgroup>
                        <optgroup label="Solders">
                            <option value=LEAD_SOLDER>"Lead Solder"</option>
                            <option value=SILVER_SOLDER>"Silver Solder"</option>
                        </optgroup>
                        <optgroup label="End Game Alloys">
                            <option value=ELECTRUM>"Electrum"</option>
                            <option value=CUPRONICKEL>"Cupronickel"</option>
                        </optgroup>
                    </select>
                </div>
                <div>
                    <label for="num-ingots">"Number of Ingots – " {move || num_ingots()}</label>
                    <br/>
                    <input
                        type="range"
                        id="num-ingots"
                        min="1"
                        max=move || max_ingots()
                        step="1"
                        on:input:target=move |ev| {
                            num_ingots(ev.target().value().parse::<i32>().unwrap())
                        }

                        prop:value=move || num_ingots()
                    />
                </div>
            </div>
            <AlloyConstituentRatioInputs alloy num_ingots max_ingots/>
            <br/>
            <a href="https://github.com/Willow-Duchars/vs_alloy_calculator" rel="external">
                "Github Repo"
            </a>
        </Window>
    }
}

fn to_percentage(num: f64) -> f64 {
    (num * 100.0).round()
}

fn round_float(num: f64) -> f64 {
    to_percentage(num) / 100.0
}

#[component]
fn alloy_constituent_ratio_inputs(
    alloy: ReadSignal<Alloys>,
    num_ingots: RwSignal<i32>,
    max_ingots: RwSignal<i32>,
) -> impl IntoView {
    let alloy_default = alloy.get_untracked().get_default();
    num_ingots(alloy_default.num_ingots());
    max_ingots(alloy_default.max_ingots());

    let percentages = alloy_default.percentages();
    let (percentage_a, set_percentage_a) = signal(percentages[0]);
    let (percentage_b, set_percentage_b) = signal(percentages[1]);
    let (percentage_c, set_percentage_c) = signal(percentages.get(2).copied());

    let ranges = alloy.get_untracked().percentage_ranges();
    let (range_a, set_range_a) = signal(ranges[0]);
    let (range_b, set_range_b) = signal(ranges[1]);
    let (range_c, set_range_c) = signal(ranges.get(2));

    Effect::new(move |_| {
        let alloy = alloy();
        let alloy_default = alloy.get_default();

        let new_max_ingots = alloy_default.max_ingots();
        if num_ingots.get_untracked() > new_max_ingots {
            num_ingots(new_max_ingots);
        }
        max_ingots(new_max_ingots);

        let percentages = alloy_default.percentages();
        set_percentage_a(percentages[0]);
        set_percentage_b(percentages[1]);
        set_percentage_c(percentages.get(2).copied());

        let ranges = alloy.percentage_ranges();
        set_range_a(ranges[0]);
        set_range_b(ranges[1]);
        set_range_c(ranges.get(2));
    });

    view! {
        <div class="row">
            <div style="flex-grow:1;">
                <div>
                    <label for="constituent-a">
                        {move || {
                            let percentage = percentage_a();
                            format!("{} – {}%", percentage.name(), to_percentage(*percentage))
                        }}

                    </label>
                    <br/>
                    <input
                        type="range"
                        id="constituent-a"
                        min=move || to_percentage(range_a().min)
                        max=move || to_percentage(range_a().max)
                        step="1"
                        on:input:target=move |ev| {
                            let value_a = ev.target().value().parse::<f64>().unwrap() / 100.0;
                            let (value_b, value_c) = match percentage_c.get_untracked() {
                                None => (1.0 - value_a, None),
                                Some(value_c) => {
                                    let value_c = *value_c;
                                    let value_b = 1.0 - value_a - value_c;
                                    let range_b = range_b.get_untracked();
                                    if value_b < range_b.min {
                                        (range_b.min, Some(1.0 - value_a - range_b.min))
                                    } else if value_b > range_b.max {
                                        (range_b.max, Some(1.0 - value_a - range_b.max))
                                    } else {
                                        (value_b, Some(value_c))
                                    }
                                }
                            };
                            set_percentage_a.update(|a| a.update(round_float(value_a)));
                            set_percentage_b.update(|b| b.update(round_float(value_b)));
                            if let Some(value_c) = value_c {
                                set_percentage_c
                                    .update(|c| match c {
                                        Some(c) => c.update(round_float(value_c)),
                                        None => {
                                            *c = match alloy.get_untracked() {
                                                Alloys::BismuthBronze => Some(Bismuth(round_float(value_c))),
                                                Alloys::BlackBronze => Some(Silver(round_float(value_c))),
                                                _ => None,
                                            };
                                        }
                                    })
                            }
                        }

                        prop:value=move || to_percentage(*percentage_a())
                    />
                </div>
                <div>
                    <label for="constituent-b">
                        {move || {
                            let percentage = percentage_b();
                            format!("{} – {}%", percentage.name(), to_percentage(*percentage))
                        }}

                    </label>
                    <br/>
                    <input
                        type="range"
                        id="constituent-b"
                        min=move || to_percentage(range_b().min)
                        max=move || to_percentage(range_b().max)
                        step="1"
                        on:input:target=move |ev| {
                            let value_b = ev.target().value().parse::<f64>().unwrap() / 100.0;
                            let (value_a, value_c) = match range_c.get_untracked() {
                                None => (1.0 - value_b, None),
                                Some(range_c) => {
                                    let value_a = *percentage_a.get_untracked();
                                    let value_c = 1.0 - value_a - value_b;
                                    if value_c < range_c.min {
                                        (1.0 - value_b - range_c.min, Some(range_c.min))
                                    } else if value_c > range_c.max {
                                        (1.0 - value_b - range_c.max, Some(range_c.max))
                                    } else {
                                        (value_a, Some(value_c))
                                    }
                                }
                            };
                            set_percentage_a.update(|a| a.update(round_float(value_a)));
                            set_percentage_b.update(|b| b.update(round_float(value_b)));
                            if let Some(value_c) = value_c {
                                set_percentage_c
                                    .update(|c| match c {
                                        Some(c) => c.update(round_float(value_c)),
                                        None => {
                                            *c = match alloy.get_untracked() {
                                                Alloys::BismuthBronze => Some(Bismuth(round_float(value_c))),
                                                Alloys::BlackBronze => Some(Silver(round_float(value_c))),
                                                _ => None,
                                            };
                                        }
                                    })
                            }
                        }

                        prop:value=move || to_percentage(*percentage_b())
                    />

                </div>
                {move || match alloy() {
                    Alloys::BismuthBronze | Alloys::BlackBronze => {
                        view! {
                            <div>
                                <label for="constituent-c">
                                    {move || {
                                        match percentage_c() {
                                            None => "".into(),
                                            Some(c) => {
                                                format!("{} – {}%", c.name(), to_percentage(*c))
                                            }
                                        }
                                    }}

                                </label>
                                <br/>
                                <input
                                    type="range"
                                    id="constituent-c"
                                    min=move || {
                                        range_c().and_then(|r_c| Some(to_percentage(r_c.min)))
                                    }

                                    max=move || {
                                        range_c().and_then(|r_c| Some(to_percentage(r_c.max)))
                                    }

                                    step="1"
                                    on:input:target=move |ev| {
                                        let value_c = ev.target().value().parse::<f64>().unwrap()
                                            / 100.0;
                                        let value_a = *percentage_a.get_untracked();
                                        let value_b = 1.0 - value_a - value_c;
                                        let range_b = range_b.get_untracked();
                                        let (value_a, value_b) = if value_b < range_b.min {
                                            (1.0 - range_b.min - value_c, range_b.min)
                                        } else if value_b > range_b.max {
                                            (1.0 - range_b.max - value_c, range_b.max)
                                        } else {
                                            (value_a, value_b)
                                        };
                                        set_percentage_a.update(|a| a.update(round_float(value_a)));
                                        set_percentage_b.update(|b| b.update(round_float(value_b)));
                                        set_percentage_c
                                            .update(|c| match c {
                                                Some(c) => c.update(round_float(value_c)),
                                                None => {
                                                    *c = match alloy.get_untracked() {
                                                        Alloys::BismuthBronze => Some(Bismuth(round_float(value_c))),
                                                        Alloys::BlackBronze => Some(Silver(round_float(value_c))),
                                                        _ => None,
                                                    };
                                                }
                                            });
                                    }

                                    prop:value=move || {
                                        match percentage_c() {
                                            None => 0.0,
                                            Some(p_c) => to_percentage(*p_c),
                                        }
                                    }
                                />

                            </div>
                        }
                            .into_any()
                    }
                    _ => view! {}.into_any(),
                }}

            </div>
            <AlloyOutput alloy num_ingots max_ingots percentage_a percentage_b percentage_c/>
        </div>
    }
}

#[component]
fn alloy_output(
    alloy: ReadSignal<Alloys>,
    num_ingots: RwSignal<i32>,
    max_ingots: RwSignal<i32>,
    percentage_a: ReadSignal<BaseMetal<f64>>,
    percentage_b: ReadSignal<BaseMetal<f64>>,
    percentage_c: ReadSignal<Option<BaseMetal<f64>>>,
) -> impl IntoView {
    let (output, set_output) = signal(alloy.get_untracked().get_default());
    let (slot_1, set_slot_1) = signal(None);
    let (slot_2, set_slot_2) = signal(None);
    let (slot_3, set_slot_3) = signal(None);
    let (slot_4, set_slot_4) = signal(None);
    // Updated when new alloy is selected. Gets and sets the new alloy to its defaults but keeps the currently selected num_ingots
    Effect::new(move |_| {
        let mut alloy = alloy().get_default();
        if let Err(e) = alloy.set_num_ingots(num_ingots.get_untracked()) {
            error!("Set Default. Error: {e:?}");
        };
        set_output(alloy);
    });
    // Output gets updated when num_ingots changes
    Effect::new(move |_| {
        set_output.update(|alloy| {
            if let Err(e) = alloy.set_num_ingots(num_ingots()) {
                error!("Set Num Ingots. Error: {e:?}");
            };
        })
    });
    // Output gets updated when any of the constituent percentages change
    Effect::new(move |_| {
        set_output.update(|alloy| {
            let (a, b, c) = (percentage_a(), percentage_b(), percentage_c());
            while let Err(_) = match c {
                Some(c) => alloy.set_percentages([a, b, c]),
                None => alloy.set_percentages([a, b]),
            } {
                num_ingots.update(|num| {
                    *num -= 1;
                    let _ = alloy.set_num_ingots(*num);
                });
            }
            max_ingots(alloy.max_ingots());
        });
    });
    Effect::new(move |_| {
        let output = output.read();
        let nuggets = output.nuggets();
        let mut slots = Vec::with_capacity(4);
        let mut constituent_a = nuggets[0];
        let mut constituent_b = nuggets[1];
        while *constituent_a != 0 {
            if (*constituent_a / 128) > 0 {
                slots.push(Some(constituent_a.update_inner_value(128)));
                constituent_a.update(*constituent_a - 128);
            } else {
                slots.push(Some(constituent_a));
                constituent_a.update(0);
            }
        }
        while *constituent_b != 0 {
            if (*constituent_b / 128) > 0 {
                slots.push(Some(constituent_b.update_inner_value(128)));
                constituent_b.update(*constituent_b - 128);
            } else {
                slots.push(Some(constituent_b));
                constituent_b.update(0);
            }
        }
        if let Some(mut constituent_c) = nuggets.get(2).copied() {
            while *constituent_c != 0 {
                if (*constituent_c / 128) > 0 {
                    slots.push(Some(constituent_c.update_inner_value(128)));
                    constituent_c.update(*constituent_c - 128);
                } else {
                    slots.push(Some(constituent_c));
                    constituent_c.update(0);
                }
            }
        }
        let filled_slots = 4 - slots.len();
        for _ in 0..filled_slots {
            slots.push(None);
        }
        set_slot_1(slots[0]);
        set_slot_2(slots[1]);
        set_slot_3(slots[2]);
        set_slot_4(slots[3]);
    });
    view! {
        <div id="crucible">
            <div class="row">
                <div class="crucible-item">
                    {move || {
                        let slot_1 = slot_1();
                        let data = match slot_1 {
                            Some(metal) => choose_sprite(metal.name()),
                            None => choose_sprite("Empty"),
                        };
                        view! {
                            <Sprite data/>
                            <p>{slot_1.and_then(|n| { if *n > 1 { Some(*n) } else { None } })}</p>
                        }
                    }}
                    <Sprite data=choose_sprite("Highlight")/>
                </div>
                <div class="crucible-item">
                    {move || {
                        let slot_2 = slot_2();
                        let data = match slot_2 {
                            Some(metal) => choose_sprite(metal.name()),
                            None => choose_sprite("Empty"),
                        };
                        view! {
                            <Sprite data/>
                            <p>{slot_2.and_then(|n| { if *n > 1 { Some(*n) } else { None } })}</p>
                        }
                    }}
                    <Sprite data=choose_sprite("Highlight")/>
                </div>
                <div class="crucible-item">
                    {move || {
                        let slot_3 = slot_3();
                        let data = match slot_3 {
                            Some(metal) => choose_sprite(metal.name()),
                            None => choose_sprite("Empty"),
                        };
                        view! {
                            <Sprite data/>
                            <p>{slot_3.and_then(|n| { if *n > 1 { Some(*n) } else { None } })}</p>
                        }
                    }}
                    <Sprite data=choose_sprite("Highlight")/>
                </div>
                <div class="crucible-item">
                    {move || {
                        let slot_4 = slot_4();
                        let data = match slot_4 {
                            Some(metal) => choose_sprite(metal.name()),
                            None => choose_sprite("Empty"),
                        };
                        view! {
                            <Sprite data/>
                            <p>{slot_4.and_then(|n| { if *n > 1 { Some(*n) } else { None } })}</p>
                        }
                    }}
                    <Sprite data=choose_sprite("Highlight")/>
                </div>

            </div>
            <div class="row">
                <div style="flex-grow:1;">
                    <div class="crucible-item">
                        <Sprite data=choose_sprite("Crucible")/>
                        <Sprite data=choose_sprite("Highlight")/>

                    </div>
                </div>
                <div class="crucible-item">
                    {move || {
                        let alloy = output.read();
                        let num_ingots = alloy.num_ingots();
                        view! {
                            <Sprite data=choose_sprite(alloy.name())/>
                            <p>{if num_ingots > 1 { Some(num_ingots) } else { None }}</p>
                        }
                    }}
                    <Sprite data=choose_sprite("Highlight")/>
                </div>
            </div>
        </div>
    }
}

fn choose_sprite(name: &str) -> SpriteData {
    use {alloy_names::*, base_metal_names::*, window_lib::constants::alloy_calculator::*};
    match name {
        NICKEL => NUGGET_PENTLANDITE,
        COPPER => NUGGET_NATIVE_COPPER,
        ZINC => NUGGET_SPHALERITE,
        SILVER => NUGGET_NATIVE_SILVER,
        TIN => NUGGET_CASSITERITE,
        GOLD => NUGGET_NATIVE_GOLD,
        LEAD => NUGGET_GALENA,
        BISMUTH => NUGGET_BISMUTHINITE,
        TIN_BRONZE => INGOT_TIN_BRONZE,
        BISMUTH_BRONZE => INGOT_BISMUTH_BRONZE,
        BLACK_BRONZE => INGOT_BLACK_BRONZE,
        BRASS => INGOT_BRASS,
        MOLYBDOCHALKOS => INGOT_MOLYBDOCHALKOS,
        LEAD_SOLDER => INGOT_LEAD_SOLDER,
        SILVER_SOLDER => INGOT_SILVER_SOLDER,
        ELECTRUM => INGOT_ELECTRUM,
        CUPRONICKEL => INGOT_CUPRONICKEL,
        "Crucible" => CRUCIBLE,
        "Charcoal" => CHARCOAL,
        "Highlight" => SLOT_HIGHLIGHT,
        _ => EMPTY_SLOT,
    }
}
