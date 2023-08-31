use iced_widget::Row;
use iced_winit::core::{Element, Length};
use plotters::{
    prelude::PathElement,
    series::LineSeries,
    style::{Color, IntoFont, BLACK, RED, WHITE},
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::controls::Message;

#[derive(Debug, Clone)]
pub struct MyBasicChart;

impl Chart<Message> for MyBasicChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let result: Result<(), String> = (|| {
            let mut chart = builder
                .caption("y=x^2", ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                .map_err(|err| err.to_string())?;

            chart
                .configure_mesh()
                .draw()
                .map_err(|err| err.to_string())?;

            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &RED,
                ))
                .map_err(|err| err.to_string())?
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

            chart
                .configure_series_labels()
                .background_style(&WHITE.mix(0.8))
                .border_style(&BLACK)
                .draw()
                .map_err(|err| err.to_string())?;

            Ok(())
        })();

        if let Err(err) = result {
            log::error!("Error building chart: {err:?}");
        }
    }
}

impl MyBasicChart {
    pub fn view(&self) -> Element<Message, iced_wgpu::Renderer<iced::theme::Theme>> {
        let _chart: ChartWidget<'_, Message, iced_wgpu::Renderer<iced::theme::Theme>, &Self> =
            ChartWidget::new(self)
                .width(Length::Fixed(400.0))
                .height(Length::Fixed(300.0));
        // fails if you uncomment this
        // let element: Element<Message, iced_wgpu::Renderer<iced::theme::Theme>> =
        //     Element::new(_chart);

        Row::new().into()
    }
}
