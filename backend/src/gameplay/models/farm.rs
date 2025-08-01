use crate::gameplay::data::{Crop, CropType, Item};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Farm {
    plots: Vec<Option<Crop>>,
    max_plots: usize,
}

impl Farm {
    pub fn new() -> Self {
        Farm {
            plots: vec![None; 9],
            max_plots: 9, // 9 plots maximum for now
        }
    }

    pub fn status(&self) -> String {
        let mut status = String::new();
        status.push_str("Farm Status:\n");

        for (i, plot) in self.plots.iter().enumerate() {
            match plot {
                Some(crop) => {
                    let ready_status = if crop.is_ready {
                        "(Ready!)"
                    } else {
                        "(Growing...)"
                    };
                    status.push_str(&format!("Plot {}: {:?} {}\n", i, crop, ready_status));
                }
                None => {
                    status.push_str(&format!("Plot {}: Empty\n", i));
                }
            }
        }

        status
    }

    fn check_plot_id(&self, plot_id: usize) -> Result<(), String> {
        if plot_id >= self.max_plots {
            return Err(format!(
                "Plot ID {} is out of range (max: {})",
                plot_id,
                self.max_plots - 1
            ));
        }

        Ok(())
    }

    pub fn plant(
        &mut self,
        plot_id: usize,
        crop_type: CropType,
        current_tick: u32,
    ) -> Result<String, String> {
        self.check_plot_id(plot_id)?;

        if self.plots[plot_id].is_some() {
            return Err("Plot is already occupied!".to_string());
        }

        let crop = Crop::new(crop_type, current_tick);
        self.plots[plot_id] = Some(crop);

        Ok(format!("Planted {:?} in plot {}", crop_type, plot_id))
    }

    pub fn harvest(&mut self, plot_id: usize) -> Result<Item, String> {
        self.check_plot_id(plot_id)?;

        match &self.plots[plot_id] {
            Some(crop) if crop.is_ready => {
                let item = crop.kind.to_item();
                self.plots[plot_id] = None;
                Ok(item)
            }
            Some(_) => Err(format!("Crop in plot {} is not ready to harvest", plot_id)),
            None => Err(format!("Plot {} is empty", plot_id)),
        }
    }

    pub fn update_crops(&mut self, current_tick: u32) {
        for plot in &mut self.plots {
            if let Some(crop) = plot {
                crop.update(current_tick);
            }
        }
    }
}
