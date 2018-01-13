use std::cmp::Ordering::Equal;
use util;

#[derive(Debug)]
pub struct Graph {
    pub name: String,
    pub name_base: String,
    pub name_folder: String,
    pub unit: String,
    pub x_unit: String,
    pub series: Vec<Series>,
    pub virt: bool,
    pub joinable: bool,
    pub area: bool,
    pub zero: bool,
}

#[derive(Debug)]
pub struct Series {
    pub name: String,
    pub data: Vec<(f64, f64)>,
}

impl Graph {
    pub fn from_default(
        name: String,
        unit: String,
        x_unit: String,
        series: Vec<Series>,
        virt: bool,
    ) -> Graph {
        let (name_folder, name_base) = util::split_name(&name);
        Graph {
            name,
            name_base,
            name_folder,
            unit,
            x_unit,
            series,
            area: false,
            virt,
            zero: false,
            joinable: false,
        }
    }

    pub fn gen_highchart(&self) -> String {
        let mut series_content = String::new();
        let mut min_y = 0f64;
        for s in &self.series {
            let data = s.data
                .iter()
                .map(|p| {
                    let (x, y) = *p;
                    format!("[{},{}]", x, y)
                })
                .collect::<Vec<String>>()
                .join(",");

            let series_text = format!(
                "{{
                name: '{name}',
                data: [{data}]
            }},",
                name = s.name,
                data = data
            );

            series_content += &series_text;

            let min_y_local = s.data
                .iter()
                .map(|p| {
                    let (_, y) = *p;
                    y
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                .unwrap();

            if min_y_local < min_y {
                min_y = min_y_local;
            }
        }

        let unit = format!(" ({})", self.unit);

        let graph_type = if self.area { "area" } else { "line" };

        let min_y_text = if self.zero {
            format!(
                "yAxis: {{
                min: {min_y}
            }},",
                min_y = min_y
            )
        } else {
            "".to_string()
        };

        let (gen_l, gen_r) = if self.virt {
            ("[ ".to_string(), " ]".to_string())
        } else {
            ("".to_string(), "".to_string())
        };

        format!(
            "\
<div id=\"{name}\" style=\"min-width: 310px; height: 400px; margin: 0 auto\"></div>
<script>
    Highcharts.chart('{name}', {{
        chart: {{
            type: '{graph_type}',
            zoomType: 'x'
        }},
        title: {{
            text: '{title}{unit}'
        }},
        subtitle: {{
            text: '{generated_left}{name}{generated_right}'
        }},
        {min_y_text}
        xAxis: {{
            events: {{
                setExtremes: syncExtremes
            }},
            title: {{
                text: '{x_unit}'
            }}
        }},
        credits: {{
            enabled: false
        }},
        series: [{series_content}]
    }});
</script>\
",
            name = self.name,
            unit = unit,
            title = self.name_base,
            graph_type = graph_type,
            min_y_text = min_y_text,
            x_unit = self.x_unit,
            series_content = series_content,
            generated_left = gen_l,
            generated_right = gen_r
        )
    }
}

impl Series {
    pub fn differentiate(&self) -> Series {
        Series {
            name: self.name.clone(),
            data: util::differention(&self.data),
        }
    }

    pub fn integrate(&self) -> (Series, f64) {
        let (data, total_area) = util::integration(&self.data);
        (
            Series {
                name: self.name.clone(),
                data,
            },
            total_area,
        )
    }

    pub fn delta(&self) -> Series {
        Series {
            name: self.name.clone(),
            data: util::delta(&self.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::f64;

    #[bench]
    fn bench_gen_highchart_100_000(b: &mut Bencher) {
        let data = {
            let mut data = Vec::with_capacity(100_000);
            for i in 0..100_000 {
                let point = (
                    (i as f64) * f64::consts::PI,
                    (i as f64) * f64::consts::PI * f64::consts::E,
                );
                data.push(point);
            }
            data
        };
        let series = Series {
            name: "Series".to_string(),
            data,
        };
        let graph = Graph::from_default(
            "test".to_string(),
            "unit".to_string(),
            "time".to_string(),
            vec![series],
            false,
        );

        b.iter(|| graph.gen_highchart());
    }
}
