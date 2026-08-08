use iced::Length::Fixed;
use iced::widget::space::Space;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::icons;
use crate::components::{
    scan_progress,
    ui::{BtnVariant, btn, status},
};
use crate::styles::{MONO_SEMIBOLD, SANS, c, is_dark};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let header = container(header_col(app))
        .style(|_t| container::Style {
            background: None,
            ..Default::default()
        })
        .padding(Padding {
            top: 18.0,
            right: 20.0,
            bottom: 16.0,
            left: 20.0,
        })
        .width(Fill);

    let results = app.results.view(app.tr()).map(Message::ResultsList);

    let mut col = column![header].width(Fill).height(Fill);
    if app.results.count() > 0 {
        let toolbar =
            container(
                app.results
                    .toolbar(app.tr(), app.settings.online_mode_check)
                    .map(Message::ResultsList),
            )
            .padding(Padding {
                top: 0.0,
                right: 20.0,
                bottom: 14.0,
                left: 20.0,
            });
        col = col.push(toolbar);
    }
    col.push(results).into()
}

fn header_col(app: &McScan) -> Element<'_, Message> {
    let mut col = column![title_row(app)].spacing(0);

    if app.is_scanning && app.total_targets > 0 {
        col = col
            .push(Space::new().height(Fixed(14.0)))
            .push(scan_progress::render(app));
    }

    col.into()
}

fn title_row(app: &McScan) -> Element<'_, Message> {
    let found = app.results.count();
    let visible = app.results.visible_count();
    let tr = app.tr();

    let title = row![
        text(crate::APP_NAME)
            .size(18)
            .font(MONO_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#E8EBF0")
                } else {
                    c("#161A20")
                }),
            }),
        container(
            text(tr.subtitle)
                .size(13.5)
                .font(SANS)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) {
                        c("#6B7480")
                    } else {
                        c("#5a626c")
                    }),
                })
        )
        .padding(Padding {
            top: 3.0,
            ..Default::default()
        }),
    ]
    .align_y(Alignment::Center)
    .spacing(12);

    let mut r = row![title, Space::new().width(Fill)]
        .align_y(Alignment::Center)
        .spacing(10);

    if found > 0 {
        let label = if visible == found {
            format!("{} {}", found, tr.found)
        } else {
            format!("{} / {} {}", visible, found, tr.found)
        };
        r = r.push(status(label)).push(btn(
            BtnVariant::Icon {
                handle: icons::export(),
                size: 12.0,
            },
            Message::ExportResults,
        ));
    }

    r.into()
}
