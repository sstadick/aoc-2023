use std::{path::PathBuf, str::FromStr};

use clap::Parser;

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
        for seed in seeds.seed_ranges.into_iter().flatten() {
            let soil = seed_to_soil.lookup(seed);
            let fertilizer = soil_to_fertilizer.lookup(soil);
            let water = fertilizer_to_water.lookup(fertilizer);
            let light = water_to_light.lookup(water);
            let temp = light_to_temp.lookup(light);
            let humidity = temp_to_humidity.lookup(temp);
            let loc = humidity_to_location.lookup(humidity);
            if loc < location {
                location = loc
            }
        }
        // println!("seed_to_soil: {:?}", seed_to_soil);
        // println!("soil_to_fertilizer: {:?}", soil_to_fertilizer);
        // println!("fert_to_water: {:?}", fertilizer_to_water);
        // println!("water_to_light: {:?}", water_to_light);
        // println!("light_to_temp: {:?}", light_to_temp);
        // println!("temp_to_humid: {:?}", temp_to_humidity);
        // println!("humi_to_loc: {:?}", humidity_to_location);
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
