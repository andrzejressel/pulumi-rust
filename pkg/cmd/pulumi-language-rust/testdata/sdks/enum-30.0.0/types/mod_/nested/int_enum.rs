#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum IntEnum {
    IntOne,
    IntTwo,
}

impl pulumi_gestalt_rust::__private::serde::Serialize for IntEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            IntEnum::IntOne => 1,
            IntEnum::IntTwo => 2,
        };
        serializer.serialize_i64(value)
    }
}

impl<'de> pulumi_gestalt_rust::__private::serde::Deserialize<'de> for IntEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = i64::deserialize(deserializer)?;
        match f {
            1 => Ok(IntEnum::IntOne),
            2 => Ok(IntEnum::IntTwo),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}
