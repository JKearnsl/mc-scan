use iced::Length::{Fill, Fixed};
use iced::widget::{Column, Space, button, column, container, row, svg, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Padding, Shadow, Theme, Vector};

use crate::components::ui::{caption, icons, popover, search_input};
use crate::i18n::Tr;
use crate::styles::{SANS, SANS_SEMIBOLD, c, is_dark};

use super::{
    EditionFilter, OnlineModeFilter, ResultsList, ResultsListMessage, SortKey, WhitelistFilter,
};

type Msg = ResultsListMessage;

const CONTROL_H: f32 = 34.0;

pub(super) fn render<'a>(
    list: &'a ResultsList,
    tr: &'static Tr,
    show_login_filters: bool,
) -> Element<'a, Msg> {
    row![
        search_box(list, tr),
        sort_control(list, tr),
        filter_control(list, tr, show_login_filters),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn search_box<'a>(list: &'a ResultsList, tr: &'static Tr) -> Element<'a, Msg> {
    let icon = svg(icons::search())
        .width(Fixed(15.0))
        .height(Fixed(15.0))
        .style(|t: &Theme, _| svg::Style {
            color: Some(muted(t)),
        });

    let input = text_input(tr.search, &list.query_input)
        .on_input(Msg::SearchInput)
        .size(13)
        .font(SANS)
        .padding(0)
        .style(bare_input_style);

    container(
        row![icon, input]
            .spacing(9)
            .align_y(Alignment::Center)
            .height(Fill),
    )
    .style(field_style)
    .padding(Padding::from([0, 12]))
    .height(Fixed(CONTROL_H))
    .width(Fill)
    .into()
}

fn sort_control<'a>(list: &'a ResultsList, tr: &'static Tr) -> Element<'a, Msg> {
    let arrow = if list.filters.descending {
        "\u{2193}"
    } else {
        "\u{2191}"
    };
    let trigger = control_button(
        row![
            svg(icons::sort())
                .width(Fixed(14.0))
                .height(Fixed(14.0))
                .style(|t: &Theme, _| svg::Style {
                    color: Some(muted(t))
                }),
            text(sort_name(list.filters.sort, tr))
                .size(13)
                .font(SANS_SEMIBOLD)
                .style(|t: &Theme| text::Style {
                    color: Some(primary_text(t))
                }),
            text(arrow)
                .size(13)
                .font(SANS_SEMIBOLD)
                .style(|t: &Theme| text::Style {
                    color: Some(accent(t))
                }),
        ]
        .spacing(7)
        .align_y(Alignment::Center)
        .height(Fill),
        Msg::ToggleSortMenu,
        false,
    );

    popover(
        trigger,
        sort_menu(list, tr),
        list.sort_open,
        Msg::DismissMenus,
    )
    .into()
}

fn sort_menu<'a>(list: &'a ResultsList, tr: &'static Tr) -> Element<'a, Msg> {
    let active = list.filters.sort;
    let options = column![
        option_row(tr.sort_recent, SortKey::Found, active),
        option_row(tr.sort_players, SortKey::Players, active),
        option_row(tr.sort_ping, SortKey::Ping, active),
    ]
    .spacing(2);

    let direction = column![
        caption(tr.direction, 11),
        segmented(
            vec![(tr.dir_asc, false), (tr.dir_desc, true)],
            list.filters.descending,
            Msg::SortDescending,
        ),
    ]
    .spacing(7);

    let header = container(caption(tr.sort_by, 11)).padding(Padding {
        top: 4.0,
        right: 10.0,
        bottom: 4.0,
        left: 10.0,
    });

    panel(
        column![
            header,
            options,
            container(divider_line()).padding(Padding::from([6, 6])),
            container(direction).padding(Padding {
                top: 2.0,
                right: 8.0,
                bottom: 8.0,
                left: 8.0,
            }),
        ]
        .spacing(2),
        210.0,
        6.0,
    )
}

fn option_row<'a>(label: &'a str, key: SortKey, active: SortKey) -> Element<'a, Msg> {
    let is_active = key == active;
    let check: Element<'a, Msg> = if is_active {
        svg(icons::check())
            .width(Fixed(14.0))
            .height(Fixed(14.0))
            .style(|t: &Theme, _| svg::Style {
                color: Some(accent(t)),
            })
            .into()
    } else {
        Space::new().width(Fixed(14.0)).into()
    };

    button(
        row![
            text(label)
                .size(13)
                .font(if is_active { SANS_SEMIBOLD } else { SANS })
                .width(Fill)
                .style(move |t: &Theme| text::Style {
                    color: Some(if is_active {
                        primary_text(t)
                    } else {
                        secondary_text(t)
                    }),
                }),
            check,
        ]
        .align_y(Alignment::Center),
    )
    .on_press(Msg::SortPicked(key))
    .style(menu_row_style)
    .padding(Padding::from([8, 10]))
    .width(Fill)
    .into()
}

fn sort_name(key: SortKey, tr: &'static Tr) -> &'static str {
    match key {
        SortKey::Found => tr.sort_recent,
        SortKey::Players => tr.sort_players,
        SortKey::Ping => tr.sort_ping,
    }
}

fn filter_control<'a>(
    list: &'a ResultsList,
    tr: &'static Tr,
    show_login_filters: bool,
) -> Element<'a, Msg> {
    let count = list.filters.active_count();
    let mut label = row![
        svg(icons::filter())
            .width(Fixed(14.0))
            .height(Fixed(14.0))
            .style(|t: &Theme, _| svg::Style {
                color: Some(muted(t))
            }),
        text(tr.filters_title)
            .size(13)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(primary_text(t))
            }),
    ]
    .spacing(7)
    .align_y(Alignment::Center)
    .height(Fill);

    if count > 0 {
        label = label.push(count_badge(count));
    }

    let trigger = control_button(label, Msg::ToggleFilterMenu, count > 0);
    popover(
        trigger,
        filter_panel(list, tr, show_login_filters),
        list.filters_open,
        Msg::DismissMenus,
    )
    .into()
}

fn filter_panel<'a>(
    list: &'a ResultsList,
    tr: &'static Tr,
    show_login_filters: bool,
) -> Element<'a, Msg> {
    let edition = field(
        tr.edition,
        segmented(
            vec![
                (tr.filter_all, EditionFilter::All),
                (tr.filter_java, EditionFilter::Java),
                (tr.filter_bedrock, EditionFilter::Bedrock),
            ],
            list.filters.edition,
            Msg::EditionPicked,
        ),
    );

    let online = field(
        tr.online_mode,
        segmented(
            vec![
                (tr.online_any, OnlineModeFilter::Any),
                (tr.online_yes, OnlineModeFilter::Online),
                (tr.online_no, OnlineModeFilter::Cracked),
            ],
            list.filters.online_mode,
            Msg::OnlineModePicked,
        ),
    );

    let whitelist = field(
        tr.whitelist,
        segmented(
            vec![
                (tr.online_any, WhitelistFilter::Any),
                (tr.enabled, WhitelistFilter::On),
                (tr.disabled, WhitelistFilter::Off),
            ],
            list.filters.whitelist,
            Msg::WhitelistPicked,
        ),
    );

    let version = field(
        tr.version,
        search_input(&list.filters.version, tr.version_hint, Msg::VersionFilter)
            .width(Fill)
            .into(),
    );
    let plugin = field(
        tr.plugins,
        search_input(&list.filters.plugin, tr.plugin_hint, Msg::PluginFilter)
            .width(Fill)
            .into(),
    );

    let reset = button(
        text(tr.reset)
            .size(12)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(secondary_text(t)),
            }),
    )
    .on_press(Msg::ResetFilters)
    .style(menu_row_style)
    .padding(Padding::from([6, 10]));

    let footer = row![Space::new().width(Fill), reset].align_y(Alignment::Center);

    let mut children: Vec<Element<'a, Msg>> = vec![edition];
    if show_login_filters {
        children.push(online);
        children.push(whitelist);
    }
    children.push(version);
    children.push(plugin);
    children.push(footer.into());

    panel(Column::with_children(children).spacing(14), 300.0, 16.0)
}

fn field<'a>(label: &'a str, control: Element<'a, Msg>) -> Element<'a, Msg> {
    column![caption(label, 11), control].spacing(7).into()
}

fn count_badge<'a>(count: usize) -> Element<'a, Msg> {
    container(
        text(count.to_string())
            .size(11)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#08110B")
                } else {
                    c("#FFFFFF")
                }),
            }),
    )
    .style(|t: &Theme| container::Style {
        background: Some(Background::Color(accent(t))),
        border: Border {
            radius: 7.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .padding(Padding::from([1, 6]))
    .into()
}

fn control_button<'a>(
    content: impl Into<Element<'a, Msg>>,
    on_press: Msg,
    active: bool,
) -> Element<'a, Msg> {
    button(content.into())
        .on_press(on_press)
        .style(move |t: &Theme, status| control_style(t, status, active))
        .padding(Padding::from([0, 12]))
        .height(Fixed(CONTROL_H))
        .into()
}

fn segmented<'a, T: Copy + PartialEq + 'a>(
    options: Vec<(&'a str, T)>,
    active: T,
    on_pick: impl Fn(T) -> Msg,
) -> Element<'a, Msg> {
    let mut r = row![].spacing(6);
    for (label, value) in options {
        r = r.push(pill(label, value == active, on_pick(value)));
    }
    r.into()
}

fn pill<'a>(label: &'a str, active: bool, on_press: Msg) -> Element<'a, Msg> {
    button(
        container(text(label).size(12).font(SANS_SEMIBOLD))
            .center_x(Fill)
            .center_y(Fill),
    )
    .on_press(on_press)
    .style(move |t: &Theme, status| pill_style(t, status, active))
    .padding(Padding::from([6, 12]))
    .height(Fixed(30.0))
    .width(Fill)
    .into()
}

fn panel<'a>(content: impl Into<Element<'a, Msg>>, width: f32, pad: f32) -> Element<'a, Msg> {
    container(content.into())
        .style(panel_style)
        .padding(pad)
        .width(Fixed(width))
        .into()
}

fn divider_line<'a>() -> Element<'a, Msg> {
    container(Space::new())
        .style(|t: &Theme| container::Style {
            background: Some(Background::Color(if is_dark(t) {
                c("#242C38")
            } else {
                c("#E8ECF1")
            })),
            ..Default::default()
        })
        .width(Fill)
        .height(Fixed(1.0))
        .into()
}

fn accent(t: &Theme) -> Color {
    if is_dark(t) {
        c("#3DD68C")
    } else {
        c("#18A862")
    }
}
fn primary_text(t: &Theme) -> Color {
    if is_dark(t) {
        c("#E8EBF0")
    } else {
        c("#161A20")
    }
}
fn secondary_text(t: &Theme) -> Color {
    if is_dark(t) {
        c("#A2ABBA")
    } else {
        c("#3A4049")
    }
}
fn muted(t: &Theme) -> Color {
    if is_dark(t) {
        c("#6B7480")
    } else {
        c("#8A929E")
    }
}
fn accent_wash(t: &Theme, a: f32) -> Color {
    let (r, g, b) = if is_dark(t) {
        (0.239f32, 0.839, 0.549)
    } else {
        (0.094f32, 0.659, 0.384)
    };
    Color { r, g, b, a }
}

fn field_style(t: &Theme) -> container::Style {
    let dark = is_dark(t);
    container::Style {
        background: Some(Background::Color(if dark {
            c("#181D25")
        } else {
            c("#FFFFFF")
        })),
        border: Border {
            color: if dark { c("#232A34") } else { c("#DDE2E8") },
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

fn bare_input_style(t: &Theme, _status: text_input::Status) -> text_input::Style {
    let dark = is_dark(t);
    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: primary_text(t),
        selection: accent_wash(t, 0.25),
    }
}

fn control_style(t: &Theme, status: button::Status, active: bool) -> button::Style {
    let dark = is_dark(t);
    let (bg, border_c, txt) = if active {
        (
            accent_wash(t, if dark { 0.12 } else { 0.09 }),
            accent_wash(t, if dark { 0.35 } else { 0.28 }),
            primary_text(t),
        )
    } else {
        (
            if dark { c("#181D25") } else { c("#FFFFFF") },
            if dark { c("#232A34") } else { c("#DDE2E8") },
            primary_text(t),
        )
    };
    let base = button::Style {
        background: Some(Background::Color(bg)),
        text_color: txt,
        border: Border {
            color: border_c,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered | button::Status::Pressed if !active => button::Style {
            background: Some(Background::Color(if dark {
                c("#1F2630")
            } else {
                c("#F2F4F7")
            })),
            border: Border {
                color: if dark { c("#2E3849") } else { c("#C8D0DA") },
                ..base.border
            },
            ..base
        },
        _ => base,
    }
}

fn pill_style(t: &Theme, status: button::Status, active: bool) -> button::Style {
    let dark = is_dark(t);
    let base = if active {
        button::Style {
            background: Some(Background::Color(accent_wash(
                t,
                if dark { 0.16 } else { 0.12 },
            ))),
            text_color: if dark { c("#9FE9C4") } else { c("#0B6040") },
            border: Border {
                color: accent_wash(t, if dark { 0.40 } else { 0.30 }),
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    } else {
        button::Style {
            background: Some(Background::Color(if dark {
                c("#12161C")
            } else {
                c("#F1F3F6")
            })),
            text_color: muted(t),
            border: Border {
                color: if dark { c("#232A34") } else { c("#E1E5EA") },
                width: 1.0,
                radius: 7.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    };
    match status {
        button::Status::Hovered if active => button::Style {
            background: Some(Background::Color(accent_wash(
                t,
                if dark { 0.22 } else { 0.16 },
            ))),
            ..base
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(if dark {
                c("#1B222C")
            } else {
                c("#E8ECF1")
            })),
            text_color: primary_text(t),
            ..base
        },
        _ => base,
    }
}

fn menu_row_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: primary_text(t),
        border: Border {
            radius: 7.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(Background::Color(if dark {
                c("#222A35")
            } else {
                c("#F1F3F6")
            })),
            ..base
        },
        _ => base,
    }
}

fn panel_style(t: &Theme) -> container::Style {
    let dark = is_dark(t);
    container::Style {
        background: Some(Background::Color(if dark {
            c("#1A2029")
        } else {
            c("#FFFFFF")
        })),
        border: Border {
            color: if dark { c("#2A323E") } else { c("#E4E8ED") },
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: if dark { 0.45 } else { 0.16 },
            },
            offset: Vector::new(0.0, 10.0),
            blur_radius: 28.0,
        },
        ..Default::default()
    }
}
