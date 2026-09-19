//! Optional, deterministic annotations evaluated after base-plate construction.
//!
//! Rules are explicit: no extension changes the calendar, formation, star/door
//! positions, or the base chart's hour-void and hour-horse conventions.

mod growth;
mod model;
mod options;
mod placement;
mod strength;

pub use model::*;
pub use options::*;

use crate::Chart;

pub(crate) fn calculate(chart: &Chart, options: &ExtensionOptions) -> ChartExtensions {
    let hidden_stems = options
        .hidden_stems
        .map(|rule| placement::hidden_stems(chart, rule));
    let placements = placement::stem_placements(chart, hidden_stems.as_ref());
    ChartExtensions {
        strength: options
            .strength
            .map(|rule| strength::calculate(chart, &placements, rule)),
        growth_stages: options
            .growth_stages
            .map(|rule| growth::calculate(&placements, rule)),
        punishments: options
            .punishments
            .map(|rule| growth::punishments(&placements, rule)),
        tombs: options.tombs.map(|rule| growth::tombs(&placements, rule)),
        day_horse: options
            .day_horse
            .map(|rule| placement::day_horse(chart, rule)),
        door_pressure: options
            .door_pressure
            .map(|rule| strength::door_pressure(chart, rule)),
        hidden_stems,
    }
}
