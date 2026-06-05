pub enum WeightUnit {
    Gram,
    Kilogram,
    Ounce,
    Pound,
}

pub enum VolumeUnit {
    Mililiter,
    Liter,
    Cup,
    Tablespoon,
}

pub enum SubjectiveUnit {
    Piece,
    Dash,
    ToTaste,
}

pub enum MeasurementUnit {
    Weight(WeightUnit),
    Volume(VolumeUnit),
    Subjective(SubjectiveUnit),
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum MeasurementCategory {
    Weight,
    Volume,
    #[default]
    Subjective,
}
