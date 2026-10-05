impl clap::ValueEnum for AudioQuality {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Mp3, Self::Cd, Self::HiresLevel1, Self::HiresLevel2, Self::HiresLevel3]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Unknown => None,
            Self::Mp3 => Some(clap::builder::PossibleValue::new("mp3")),
            Self::Cd => Some(clap::builder::PossibleValue::new("cd")),
            Self::HiresLevel1 => Some(clap::builder::PossibleValue::new("hires level 1")),
            Self::HiresLevel2 => Some(clap::builder::PossibleValue::new("hires level 2")),
            Self::HiresLevel3 => Some(clap::builder::PossibleValue::new("hires level 3"))
        }
    }
}

impl clap::ValueEnum for DeviceType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Speaker, Self::Streamer, Self::Tv, Self::Soundbar, Self::Computer, Self::Mobile, Self::Cast, Self::Headphones, Self::Tablet]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Unknown => None,
            Self::Streamer => Some(clap::builder::PossibleValue::new("streamer")),
            Self::Speaker => Some(clap::builder::PossibleValue::new("speaker")),
            Self::Tv => Some(clap::builder::PossibleValue::new("tv")),
            Self::Soundbar => Some(clap::builder::PossibleValue::new("soundbar")),
            Self::Computer => Some(clap::builder::PossibleValue::new("computer")),
            Self::Mobile => Some(clap::builder::PossibleValue::new("mobile")),
            Self::Cast => Some(clap::builder::PossibleValue::new("cast")),
            Self::Headphones => Some(clap::builder::PossibleValue::new("headphones")),
            Self::Tablet => Some(clap::builder::PossibleValue::new("tablet"))
        }
    }
}