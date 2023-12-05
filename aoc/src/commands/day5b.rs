use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use itertools::Itertools;

use crate::utils::slurp_file;

use super::{
    day5a::{Mapping, SeedRanges},
    CommandImpl, DynError,
};

#[derive(Parser, Debug)]
pub struct Day5b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day5b {
    fn main(&self) -> Result<(), DynError> {
        let (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_location,
        ) = self.parse_input_b()?;

        let mut location = usize::MAX;
        for seed in seeds.seed_ranges.into_iter() {
            let soils = seed_to_soil.lookup_range(&seed);
            let loc = soils
                .into_iter()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .map(|range| soil_to_fertilizer.lookup_range(&range))
                .flatten()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .map(|range| fertilizer_to_water.lookup_range(&range))
                .flatten()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .map(|range| water_to_light.lookup_range(&range))
                .flatten()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .map(|range| light_to_temp.lookup_range(&range))
                .flatten()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .map(|range| temp_to_humidity.lookup_range(&range))
                .flatten()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .map(|range| humidity_to_location.lookup_range(&range))
                .flatten()
                .filter(|range| range.is_some())
                .map(|range| range.unwrap())
                .sorted_by_key(|range| range.start)
                .next();

            // let fertilizer = soil_to_fertilizer.lookup_range(&soil);
            // let water = fertilizer_to_water.lookup_range(&fertilizer);
            // let light = water_to_light.lookup_range(&water);
            // let temp = light_to_temp.lookup_range(&light);
            // let humidity = temp_to_humidity.lookup_range(&temp);
            // let loc = humidity_to_location.lookup_range(&humidity);
            if loc.clone().unwrap().start < location {
                location = loc.unwrap().start
            }
        }

        println!("Day5b Answer: {}", location);
        Ok(())
    }
}

impl Day5b {
    fn parse_input_b(
        &self,
    ) -> Result<(SeedRanges, Mapping, Mapping, Mapping, Mapping, Mapping, Mapping, Mapping), DynError>
    {
        let mut seeds = None;
        let mut seed_to_soil = None;
        let mut soil_to_fertilizer = None;
        let mut fertilizer_to_water = None;
        let mut water_to_light = None;
        let mut light_to_temp = None;
        let mut temp_to_humidity = None;
        let mut humidity_to_location = None;
        let mut reader = slurp_file::<_, String>(&self.input);
        while let Some(line) = reader.next() {
            let line = line?;
            if line.starts_with("seeds") {
                seeds = Some(SeedRanges::from_str(&line)?);
            } else if line.starts_with("seed-to-soil") {
                seed_to_soil = Some(Mapping::parse_mapping(&mut reader)?);
            } else if line.starts_with("soil-to-fertilizer") {
                soil_to_fertilizer = Some(Mapping::parse_mapping(&mut reader)?);
            } else if line.starts_with("fertilizer-to-water") {
                fertilizer_to_water = Some(Mapping::parse_mapping(&mut reader)?);
            } else if line.starts_with("water-to-light") {
                water_to_light = Some(Mapping::parse_mapping(&mut reader)?);
            } else if line.starts_with("light-to-temperature") {
                light_to_temp = Some(Mapping::parse_mapping(&mut reader)?);
            } else if line.starts_with("temperature-to-humidity") {
                temp_to_humidity = Some(Mapping::parse_mapping(&mut reader)?);
            } else if line.starts_with("humidity-to-location") {
                humidity_to_location = Some(Mapping::parse_mapping(&mut reader)?);
            } else {
                continue;
            }
        }
        let mut seeds = seeds.expect("Failed to read seeds");
        let mut seed_to_soil = seed_to_soil.expect("Failed to read seed_to_soil");
        let mut soil_to_fertilizer = soil_to_fertilizer.expect("Failed to read soil_to_fertilizer");
        let mut fertilizer_to_water =
            fertilizer_to_water.expect("Failed to read fertilizer_to_water");
        let mut water_to_light = water_to_light.expect("Failed to read water_to_light");
        let mut light_to_temp = light_to_temp.expect("Failed to read light_to_temp");
        let mut temp_to_humidity = temp_to_humidity.expect("Failed to read temp_to_humidity");
        let mut humidity_to_location =
            humidity_to_location.expect("Failed to read humidity_to_location");
        Ok((
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_location,
        ))
    }
}
