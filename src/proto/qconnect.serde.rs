impl serde::Serialize for ActionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ACTION_TYPE_UNKNOWN",
            Self::Previous => "ACTION_TYPE_PREVIOUS",
            Self::Next => "ACTION_TYPE_NEXT",
            Self::RepeatOff => "ACTION_TYPE_REPEAT_OFF",
            Self::RepeatOne => "ACTION_TYPE_REPEAT_ONE",
            Self::RepeatAll => "ACTION_TYPE_REPEAT_ALL",
            Self::ShuffleOff => "ACTION_TYPE_SHUFFLE_OFF",
            Self::ShuffleOn => "ACTION_TYPE_SHUFFLE_ON",
            Self::Seek => "ACTION_TYPE_SEEK",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ActionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_TYPE_UNKNOWN",
            "ACTION_TYPE_PREVIOUS",
            "ACTION_TYPE_NEXT",
            "ACTION_TYPE_REPEAT_OFF",
            "ACTION_TYPE_REPEAT_ONE",
            "ACTION_TYPE_REPEAT_ALL",
            "ACTION_TYPE_SHUFFLE_OFF",
            "ACTION_TYPE_SHUFFLE_ON",
            "ACTION_TYPE_SEEK",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = ActionType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACTION_TYPE_UNKNOWN" => Ok(ActionType::Unknown),
                    "ACTION_TYPE_PREVIOUS" => Ok(ActionType::Previous),
                    "ACTION_TYPE_NEXT" => Ok(ActionType::Next),
                    "ACTION_TYPE_REPEAT_OFF" => Ok(ActionType::RepeatOff),
                    "ACTION_TYPE_REPEAT_ONE" => Ok(ActionType::RepeatOne),
                    "ACTION_TYPE_REPEAT_ALL" => Ok(ActionType::RepeatAll),
                    "ACTION_TYPE_SHUFFLE_OFF" => Ok(ActionType::ShuffleOff),
                    "ACTION_TYPE_SHUFFLE_ON" => Ok(ActionType::ShuffleOn),
                    "ACTION_TYPE_SEEK" => Ok(ActionType::Seek),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AudioQuality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "AUDIO_QUALITY_UNKNOWN",
            Self::Mp3 => "AUDIO_QUALITY_MP3",
            Self::Cd => "AUDIO_QUALITY_CD",
            Self::HiresLevel1 => "AUDIO_QUALITY_HIRES_LEVEL1",
            Self::HiresLevel2 => "AUDIO_QUALITY_HIRES_LEVEL2",
            Self::HiresLevel3 => "AUDIO_QUALITY_HIRES_LEVEL3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AudioQuality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUDIO_QUALITY_UNKNOWN",
            "AUDIO_QUALITY_MP3",
            "AUDIO_QUALITY_CD",
            "AUDIO_QUALITY_HIRES_LEVEL1",
            "AUDIO_QUALITY_HIRES_LEVEL2",
            "AUDIO_QUALITY_HIRES_LEVEL3",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = AudioQuality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AUDIO_QUALITY_UNKNOWN" => Ok(AudioQuality::Unknown),
                    "AUDIO_QUALITY_MP3" => Ok(AudioQuality::Mp3),
                    "AUDIO_QUALITY_CD" => Ok(AudioQuality::Cd),
                    "AUDIO_QUALITY_HIRES_LEVEL1" => Ok(AudioQuality::HiresLevel1),
                    "AUDIO_QUALITY_HIRES_LEVEL2" => Ok(AudioQuality::HiresLevel2),
                    "AUDIO_QUALITY_HIRES_LEVEL3" => Ok(AudioQuality::HiresLevel3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticateMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.AuthenticateMessage", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticateMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticateMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.AuthenticateMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticateMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthenticateMessage {
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.AuthenticateMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BufferState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "BUFFER_STATE_UNKNOWN",
            Self::Buffering => "BUFFER_STATE_BUFFERING",
            Self::Ok => "BUFFER_STATE_OK",
            Self::Error => "BUFFER_STATE_ERROR",
            Self::Underrun => "BUFFER_STATE_UNDERRUN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BufferState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BUFFER_STATE_UNKNOWN",
            "BUFFER_STATE_BUFFERING",
            "BUFFER_STATE_OK",
            "BUFFER_STATE_ERROR",
            "BUFFER_STATE_UNDERRUN",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = BufferState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "BUFFER_STATE_UNKNOWN" => Ok(BufferState::Unknown),
                    "BUFFER_STATE_BUFFERING" => Ok(BufferState::Buffering),
                    "BUFFER_STATE_OK" => Ok(BufferState::Ok),
                    "BUFFER_STATE_ERROR" => Ok(BufferState::Error),
                    "BUFFER_STATE_UNDERRUN" => Ok(BufferState::Underrun),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrAskForQueueState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrAskForQueueState", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrAskForQueueState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrAskForQueueState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrAskForQueueState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrAskForQueueState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrAskForQueueState {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrAskForQueueState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrAskForRendererState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrAskForRendererState", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrAskForRendererState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrAskForRendererState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrAskForRendererState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrAskForRendererState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrAskForRendererState {
                    renderer_id: renderer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrAskForRendererState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrAutoplayLoadTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.track_ids.is_empty() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrAutoplayLoadTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.track_ids.is_empty() {
            struct_ser.serialize_field("trackIds", &self.track_ids)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrAutoplayLoadTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "track_ids",
            "trackIds",
            "context_uuid",
            "contextUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            TrackIds,
            ContextUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "trackIds" | "track_ids" => Ok(GeneratedField::TrackIds),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrAutoplayLoadTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrAutoplayLoadTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrAutoplayLoadTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut track_ids__ = None;
                let mut context_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrackIds => {
                            if track_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackIds"));
                            }
                            track_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrAutoplayLoadTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    track_ids: track_ids__.unwrap_or_default(),
                    context_uuid: context_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrAutoplayLoadTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrAutoplayRemoveTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrAutoplayRemoveTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrAutoplayRemoveTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "queue_item_ids",
            "queueItemIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            QueueItemIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrAutoplayRemoveTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrAutoplayRemoveTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrAutoplayRemoveTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut queue_item_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrAutoplayRemoveTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrAutoplayRemoveTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrClearQueue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrClearQueue", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrClearQueue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrClearQueue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrClearQueue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrClearQueue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrClearQueue {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrClearQueue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrJoinSession {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.session_uuid.is_some() {
            len += 1;
        }
        if self.device_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrJoinSession", len)?;
        if let Some(v) = self.session_uuid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sessionUuid", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.device_info.as_ref() {
            struct_ser.serialize_field("deviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrJoinSession {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_uuid",
            "sessionUuid",
            "device_info",
            "deviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionUuid,
            DeviceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sessionUuid" | "session_uuid" => Ok(GeneratedField::SessionUuid),
                            "deviceInfo" | "device_info" => Ok(GeneratedField::DeviceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrJoinSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrJoinSession")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrJoinSession, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_uuid__ = None;
                let mut device_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionUuid => {
                            if session_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionUuid"));
                            }
                            session_uuid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DeviceInfo => {
                            if device_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceInfo"));
                            }
                            device_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CtrlSrvrJoinSession {
                    session_uuid: session_uuid__,
                    device_info: device_info__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrJoinSession", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrMuteVolume {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.value {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrMuteVolume", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.value {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrMuteVolume {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrMuteVolume;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrMuteVolume")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrMuteVolume, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrMuteVolume {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrMuteVolume", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrQueueAddTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.track_ids.is_empty() {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrQueueAddTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.track_ids.is_empty() {
            struct_ser.serialize_field("trackIds", &self.track_ids)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrQueueAddTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "track_ids",
            "trackIds",
            "shuffle_seed",
            "shuffleSeed",
            "context_uuid",
            "contextUuid",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            TrackIds,
            ShuffleSeed,
            ContextUuid,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "trackIds" | "track_ids" => Ok(GeneratedField::TrackIds),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrQueueAddTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrQueueAddTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrQueueAddTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut track_ids__ = None;
                let mut shuffle_seed__ = None;
                let mut context_uuid__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrackIds => {
                            if track_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackIds"));
                            }
                            track_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrQueueAddTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    track_ids: track_ids__.unwrap_or_default(),
                    shuffle_seed: shuffle_seed__,
                    context_uuid: context_uuid__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrQueueAddTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrQueueInsertTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.track_ids.is_empty() {
            len += 1;
        }
        if self.insert_after.is_some() {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrQueueInsertTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.track_ids.is_empty() {
            struct_ser.serialize_field("trackIds", &self.track_ids)?;
        }
        if let Some(v) = self.insert_after.as_ref() {
            struct_ser.serialize_field("insertAfter", v)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrQueueInsertTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "track_ids",
            "trackIds",
            "insert_after",
            "insertAfter",
            "shuffle_seed",
            "shuffleSeed",
            "context_uuid",
            "contextUuid",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            TrackIds,
            InsertAfter,
            ShuffleSeed,
            ContextUuid,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "trackIds" | "track_ids" => Ok(GeneratedField::TrackIds),
                            "insertAfter" | "insert_after" => Ok(GeneratedField::InsertAfter),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrQueueInsertTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrQueueInsertTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrQueueInsertTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut track_ids__ = None;
                let mut insert_after__ = None;
                let mut shuffle_seed__ = None;
                let mut context_uuid__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrackIds => {
                            if track_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackIds"));
                            }
                            track_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::InsertAfter => {
                            if insert_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertAfter"));
                            }
                            insert_after__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrQueueInsertTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    track_ids: track_ids__.unwrap_or_default(),
                    insert_after: insert_after__,
                    shuffle_seed: shuffle_seed__,
                    context_uuid: context_uuid__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrQueueInsertTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrQueueLoadTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.track_ids.is_empty() {
            len += 1;
        }
        if self.queue_position != 0 {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if self.shuffle_pivot_index.is_some() {
            len += 1;
        }
        if self.shuffle_mode.is_some() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrQueueLoadTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.track_ids.is_empty() {
            struct_ser.serialize_field("trackIds", &self.track_ids)?;
        }
        if self.queue_position != 0 {
            struct_ser.serialize_field("queuePosition", &self.queue_position)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if let Some(v) = self.shuffle_pivot_index.as_ref() {
            struct_ser.serialize_field("shufflePivotIndex", v)?;
        }
        if let Some(v) = self.shuffle_mode.as_ref() {
            struct_ser.serialize_field("shuffleMode", v)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrQueueLoadTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "track_ids",
            "trackIds",
            "queue_position",
            "queuePosition",
            "shuffle_seed",
            "shuffleSeed",
            "shuffle_pivot_index",
            "shufflePivotIndex",
            "shuffle_mode",
            "shuffleMode",
            "context_uuid",
            "contextUuid",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            TrackIds,
            QueuePosition,
            ShuffleSeed,
            ShufflePivotIndex,
            ShuffleMode,
            ContextUuid,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "trackIds" | "track_ids" => Ok(GeneratedField::TrackIds),
                            "queuePosition" | "queue_position" => Ok(GeneratedField::QueuePosition),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "shufflePivotIndex" | "shuffle_pivot_index" => Ok(GeneratedField::ShufflePivotIndex),
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrQueueLoadTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrQueueLoadTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrQueueLoadTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut track_ids__ = None;
                let mut queue_position__ = None;
                let mut shuffle_seed__ = None;
                let mut shuffle_pivot_index__ = None;
                let mut shuffle_mode__ = None;
                let mut context_uuid__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrackIds => {
                            if track_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackIds"));
                            }
                            track_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::QueuePosition => {
                            if queue_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queuePosition"));
                            }
                            queue_position__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShufflePivotIndex => {
                            if shuffle_pivot_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shufflePivotIndex"));
                            }
                            shuffle_pivot_index__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = map_.next_value()?;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrQueueLoadTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    track_ids: track_ids__.unwrap_or_default(),
                    queue_position: queue_position__.unwrap_or_default(),
                    shuffle_seed: shuffle_seed__,
                    shuffle_pivot_index: shuffle_pivot_index__,
                    shuffle_mode: shuffle_mode__,
                    context_uuid: context_uuid__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrQueueLoadTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrQueueRemoveTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrQueueRemoveTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrQueueRemoveTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "queue_item_ids",
            "queueItemIds",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            QueueItemIds,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrQueueRemoveTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrQueueRemoveTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrQueueRemoveTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut queue_item_ids__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrQueueRemoveTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrQueueRemoveTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrQueueReorderTracks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        if self.insert_after.is_some() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrQueueReorderTracks", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        if let Some(v) = self.insert_after.as_ref() {
            struct_ser.serialize_field("insertAfter", v)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrQueueReorderTracks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "queue_item_ids",
            "queueItemIds",
            "insert_after",
            "insertAfter",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            QueueItemIds,
            InsertAfter,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            "insertAfter" | "insert_after" => Ok(GeneratedField::InsertAfter),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrQueueReorderTracks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrQueueReorderTracks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrQueueReorderTracks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut queue_item_ids__ = None;
                let mut insert_after__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::InsertAfter => {
                            if insert_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertAfter"));
                            }
                            insert_after__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrQueueReorderTracks {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                    insert_after: insert_after__,
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrQueueReorderTracks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetActiveRenderer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetActiveRenderer", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetActiveRenderer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetActiveRenderer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetActiveRenderer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetActiveRenderer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrSetActiveRenderer {
                    renderer_id: renderer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetActiveRenderer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetAutoplayMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_mode {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetAutoplayMode", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if self.autoplay_mode {
            struct_ser.serialize_field("autoplayMode", &self.autoplay_mode)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetAutoplayMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "autoplay_mode",
            "autoplayMode",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            AutoplayMode,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "autoplayMode" | "autoplay_mode" => Ok(GeneratedField::AutoplayMode),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetAutoplayMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetAutoplayMode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetAutoplayMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut autoplay_mode__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayMode => {
                            if autoplay_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayMode"));
                            }
                            autoplay_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrSetAutoplayMode {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    autoplay_mode: autoplay_mode__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetAutoplayMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetLoopMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.loop_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetLoopMode", len)?;
        if self.loop_mode != 0 {
            let v = LoopMode::try_from(self.loop_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.loop_mode)))?;
            struct_ser.serialize_field("loopMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetLoopMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "loop_mode",
            "loopMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoopMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "loopMode" | "loop_mode" => Ok(GeneratedField::LoopMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetLoopMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetLoopMode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetLoopMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut loop_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoopMode => {
                            if loop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopMode"));
                            }
                            loop_mode__ = Some(map_.next_value::<LoopMode>()? as i32);
                        }
                    }
                }
                Ok(CtrlSrvrSetLoopMode {
                    loop_mode: loop_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetLoopMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetMaxAudioQuality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.max_audio_quality != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetMaxAudioQuality", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.max_audio_quality != 0 {
            let v = AudioQuality::try_from(self.max_audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_audio_quality)))?;
            struct_ser.serialize_field("maxAudioQuality", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetMaxAudioQuality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "max_audio_quality",
            "maxAudioQuality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            MaxAudioQuality,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "maxAudioQuality" | "max_audio_quality" => Ok(GeneratedField::MaxAudioQuality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetMaxAudioQuality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetMaxAudioQuality")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetMaxAudioQuality, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut max_audio_quality__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxAudioQuality => {
                            if max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAudioQuality"));
                            }
                            max_audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                    }
                }
                Ok(CtrlSrvrSetMaxAudioQuality {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    max_audio_quality: max_audio_quality__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetMaxAudioQuality", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetPlayerState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.playing_state.is_some() {
            len += 1;
        }
        if self.current_position.is_some() {
            len += 1;
        }
        if self.current_queue_item.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetPlayerState", len)?;
        if let Some(v) = self.playing_state.as_ref() {
            let v = PlayingState::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("playingState", &v)?;
        }
        if let Some(v) = self.current_position.as_ref() {
            struct_ser.serialize_field("currentPosition", v)?;
        }
        if let Some(v) = self.current_queue_item.as_ref() {
            struct_ser.serialize_field("currentQueueItem", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetPlayerState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "playing_state",
            "playingState",
            "current_position",
            "currentPosition",
            "current_queue_item",
            "currentQueueItem",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayingState,
            CurrentPosition,
            CurrentQueueItem,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playingState" | "playing_state" => Ok(GeneratedField::PlayingState),
                            "currentPosition" | "current_position" => Ok(GeneratedField::CurrentPosition),
                            "currentQueueItem" | "current_queue_item" => Ok(GeneratedField::CurrentQueueItem),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetPlayerState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetPlayerState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetPlayerState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut playing_state__ = None;
                let mut current_position__ = None;
                let mut current_queue_item__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayingState => {
                            if playing_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playingState"));
                            }
                            playing_state__ = map_.next_value::<::std::option::Option<PlayingState>>()?.map(|x| x as i32);
                        }
                        GeneratedField::CurrentPosition => {
                            if current_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPosition"));
                            }
                            current_position__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CurrentQueueItem => {
                            if current_queue_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentQueueItem"));
                            }
                            current_queue_item__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CtrlSrvrSetPlayerState {
                    playing_state: playing_state__,
                    current_position: current_position__,
                    current_queue_item: current_queue_item__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetPlayerState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetQueueState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.tracks.is_empty() {
            len += 1;
        }
        if self.shuffle_mode {
            len += 1;
        }
        if !self.shuffled_track_indexes.is_empty() {
            len += 1;
        }
        if self.autoplay_mode {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if !self.autoplay_tracks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetQueueState", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.tracks.is_empty() {
            struct_ser.serialize_field("tracks", &self.tracks)?;
        }
        if self.shuffle_mode {
            struct_ser.serialize_field("shuffleMode", &self.shuffle_mode)?;
        }
        if !self.shuffled_track_indexes.is_empty() {
            struct_ser.serialize_field("shuffledTrackIndexes", &self.shuffled_track_indexes)?;
        }
        if self.autoplay_mode {
            struct_ser.serialize_field("autoplayMode", &self.autoplay_mode)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if !self.autoplay_tracks.is_empty() {
            struct_ser.serialize_field("autoplayTracks", &self.autoplay_tracks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetQueueState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "tracks",
            "shuffle_mode",
            "shuffleMode",
            "shuffled_track_indexes",
            "shuffledTrackIndexes",
            "autoplay_mode",
            "autoplayMode",
            "autoplay_loading",
            "autoplayLoading",
            "autoplay_tracks",
            "autoplayTracks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            Tracks,
            ShuffleMode,
            ShuffledTrackIndexes,
            AutoplayMode,
            AutoplayLoading,
            AutoplayTracks,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "tracks" => Ok(GeneratedField::Tracks),
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            "shuffledTrackIndexes" | "shuffled_track_indexes" => Ok(GeneratedField::ShuffledTrackIndexes),
                            "autoplayMode" | "autoplay_mode" => Ok(GeneratedField::AutoplayMode),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "autoplayTracks" | "autoplay_tracks" => Ok(GeneratedField::AutoplayTracks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetQueueState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetQueueState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetQueueState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut tracks__ = None;
                let mut shuffle_mode__ = None;
                let mut shuffled_track_indexes__ = None;
                let mut autoplay_mode__ = None;
                let mut autoplay_loading__ = None;
                let mut autoplay_tracks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tracks => {
                            if tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracks"));
                            }
                            tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffledTrackIndexes => {
                            if shuffled_track_indexes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffledTrackIndexes"));
                            }
                            shuffled_track_indexes__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::AutoplayMode => {
                            if autoplay_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayMode"));
                            }
                            autoplay_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayTracks => {
                            if autoplay_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayTracks"));
                            }
                            autoplay_tracks__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrSetQueueState {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    tracks: tracks__.unwrap_or_default(),
                    shuffle_mode: shuffle_mode__.unwrap_or_default(),
                    shuffled_track_indexes: shuffled_track_indexes__.unwrap_or_default(),
                    autoplay_mode: autoplay_mode__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    autoplay_tracks: autoplay_tracks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetQueueState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetShuffleMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version_ref.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if self.shuffle_mode {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if self.shuffle_pivot_queue_item_id.is_some() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetShuffleMode", len)?;
        if let Some(v) = self.queue_version_ref.as_ref() {
            struct_ser.serialize_field("queueVersionRef", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if self.shuffle_mode {
            struct_ser.serialize_field("shuffleMode", &self.shuffle_mode)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if let Some(v) = self.shuffle_pivot_queue_item_id.as_ref() {
            struct_ser.serialize_field("shufflePivotQueueItemId", v)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetShuffleMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version_ref",
            "queueVersionRef",
            "action_uuid",
            "actionUuid",
            "shuffle_mode",
            "shuffleMode",
            "shuffle_seed",
            "shuffleSeed",
            "shuffle_pivot_queue_item_id",
            "shufflePivotQueueItemId",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersionRef,
            ActionUuid,
            ShuffleMode,
            ShuffleSeed,
            ShufflePivotQueueItemId,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersionRef" | "queue_version_ref" => Ok(GeneratedField::QueueVersionRef),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "shufflePivotQueueItemId" | "shuffle_pivot_queue_item_id" => Ok(GeneratedField::ShufflePivotQueueItemId),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetShuffleMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetShuffleMode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetShuffleMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version_ref__ = None;
                let mut action_uuid__ = None;
                let mut shuffle_mode__ = None;
                let mut shuffle_seed__ = None;
                let mut shuffle_pivot_queue_item_id__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersionRef => {
                            if queue_version_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersionRef"));
                            }
                            queue_version_ref__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShufflePivotQueueItemId => {
                            if shuffle_pivot_queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shufflePivotQueueItemId"));
                            }
                            shuffle_pivot_queue_item_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CtrlSrvrSetShuffleMode {
                    queue_version_ref: queue_version_ref__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    shuffle_mode: shuffle_mode__.unwrap_or_default(),
                    shuffle_seed: shuffle_seed__,
                    shuffle_pivot_queue_item_id: shuffle_pivot_queue_item_id__,
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetShuffleMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CtrlSrvrSetVolume {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.volume.is_some() {
            len += 1;
        }
        if self.volume_delta.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.CtrlSrvrSetVolume", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if let Some(v) = self.volume.as_ref() {
            struct_ser.serialize_field("volume", v)?;
        }
        if let Some(v) = self.volume_delta.as_ref() {
            struct_ser.serialize_field("volumeDelta", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CtrlSrvrSetVolume {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "volume",
            "volume_delta",
            "volumeDelta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            Volume,
            VolumeDelta,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "volume" => Ok(GeneratedField::Volume),
                            "volumeDelta" | "volume_delta" => Ok(GeneratedField::VolumeDelta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CtrlSrvrSetVolume;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.CtrlSrvrSetVolume")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CtrlSrvrSetVolume, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut volume__ = None;
                let mut volume_delta__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::VolumeDelta => {
                            if volume_delta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volumeDelta"));
                            }
                            volume_delta__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(CtrlSrvrSetVolume {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    volume: volume__,
                    volume_delta: volume_delta__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.CtrlSrvrSetVolume", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceCapabilities {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_audio_quality != 0 {
            len += 1;
        }
        if self.max_audio_quality != 0 {
            len += 1;
        }
        if self.volume_remote_control != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.DeviceCapabilities", len)?;
        if self.min_audio_quality != 0 {
            let v = AudioQuality::try_from(self.min_audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.min_audio_quality)))?;
            struct_ser.serialize_field("minAudioQuality", &v)?;
        }
        if self.max_audio_quality != 0 {
            let v = AudioQuality::try_from(self.max_audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_audio_quality)))?;
            struct_ser.serialize_field("maxAudioQuality", &v)?;
        }
        if self.volume_remote_control != 0 {
            let v = VolumeRemoteControl::try_from(self.volume_remote_control)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.volume_remote_control)))?;
            struct_ser.serialize_field("volumeRemoteControl", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceCapabilities {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_audio_quality",
            "minAudioQuality",
            "max_audio_quality",
            "maxAudioQuality",
            "volume_remote_control",
            "volumeRemoteControl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinAudioQuality,
            MaxAudioQuality,
            VolumeRemoteControl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minAudioQuality" | "min_audio_quality" => Ok(GeneratedField::MinAudioQuality),
                            "maxAudioQuality" | "max_audio_quality" => Ok(GeneratedField::MaxAudioQuality),
                            "volumeRemoteControl" | "volume_remote_control" => Ok(GeneratedField::VolumeRemoteControl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceCapabilities;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.DeviceCapabilities")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceCapabilities, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_audio_quality__ = None;
                let mut max_audio_quality__ = None;
                let mut volume_remote_control__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinAudioQuality => {
                            if min_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minAudioQuality"));
                            }
                            min_audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                        GeneratedField::MaxAudioQuality => {
                            if max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAudioQuality"));
                            }
                            max_audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                        GeneratedField::VolumeRemoteControl => {
                            if volume_remote_control__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volumeRemoteControl"));
                            }
                            volume_remote_control__ = Some(map_.next_value::<VolumeRemoteControl>()? as i32);
                        }
                    }
                }
                Ok(DeviceCapabilities {
                    min_audio_quality: min_audio_quality__.unwrap_or_default(),
                    max_audio_quality: max_audio_quality__.unwrap_or_default(),
                    volume_remote_control: volume_remote_control__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.DeviceCapabilities", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.device_uuid.is_empty() {
            len += 1;
        }
        if !self.friendly_name.is_empty() {
            len += 1;
        }
        if !self.brand.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.serial_number.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.capabilities.is_some() {
            len += 1;
        }
        if !self.software_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.DeviceInfo", len)?;
        if !self.device_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deviceUuid", pbjson::private::base64::encode(&self.device_uuid).as_str())?;
        }
        if !self.friendly_name.is_empty() {
            struct_ser.serialize_field("friendlyName", &self.friendly_name)?;
        }
        if !self.brand.is_empty() {
            struct_ser.serialize_field("brand", &self.brand)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.serial_number.is_empty() {
            struct_ser.serialize_field("serialNumber", &self.serial_number)?;
        }
        if self.r#type != 0 {
            let v = DeviceType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.capabilities.as_ref() {
            struct_ser.serialize_field("capabilities", v)?;
        }
        if !self.software_version.is_empty() {
            struct_ser.serialize_field("softwareVersion", &self.software_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_uuid",
            "deviceUuid",
            "friendly_name",
            "friendlyName",
            "brand",
            "model",
            "serial_number",
            "serialNumber",
            "type",
            "capabilities",
            "software_version",
            "softwareVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceUuid,
            FriendlyName,
            Brand,
            Model,
            SerialNumber,
            Type,
            Capabilities,
            SoftwareVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deviceUuid" | "device_uuid" => Ok(GeneratedField::DeviceUuid),
                            "friendlyName" | "friendly_name" => Ok(GeneratedField::FriendlyName),
                            "brand" => Ok(GeneratedField::Brand),
                            "model" => Ok(GeneratedField::Model),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "type" => Ok(GeneratedField::Type),
                            "capabilities" => Ok(GeneratedField::Capabilities),
                            "softwareVersion" | "software_version" => Ok(GeneratedField::SoftwareVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.DeviceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_uuid__ = None;
                let mut friendly_name__ = None;
                let mut brand__ = None;
                let mut model__ = None;
                let mut serial_number__ = None;
                let mut r#type__ = None;
                let mut capabilities__ = None;
                let mut software_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceUuid => {
                            if device_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceUuid"));
                            }
                            device_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FriendlyName => {
                            if friendly_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("friendlyName"));
                            }
                            friendly_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Brand => {
                            if brand__.is_some() {
                                return Err(serde::de::Error::duplicate_field("brand"));
                            }
                            brand__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<DeviceType>()? as i32);
                        }
                        GeneratedField::Capabilities => {
                            if capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilities"));
                            }
                            capabilities__ = map_.next_value()?;
                        }
                        GeneratedField::SoftwareVersion => {
                            if software_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("softwareVersion"));
                            }
                            software_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceInfo {
                    device_uuid: device_uuid__.unwrap_or_default(),
                    friendly_name: friendly_name__.unwrap_or_default(),
                    brand: brand__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    serial_number: serial_number__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    capabilities: capabilities__,
                    software_version: software_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.DeviceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "DEVICE_TYPE_UNKNOWN",
            Self::Speaker => "DEVICE_TYPE_SPEAKER",
            Self::Streamer => "DEVICE_TYPE_STREAMER",
            Self::Tv => "DEVICE_TYPE_TV",
            Self::Soundbar => "DEVICE_TYPE_SOUNDBAR",
            Self::Computer => "DEVICE_TYPE_COMPUTER",
            Self::Mobile => "DEVICE_TYPE_MOBILE",
            Self::Cast => "DEVICE_TYPE_CAST",
            Self::Headphones => "DEVICE_TYPE_HEADPHONES",
            Self::Tablet => "DEVICE_TYPE_TABLET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DeviceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEVICE_TYPE_UNKNOWN",
            "DEVICE_TYPE_SPEAKER",
            "DEVICE_TYPE_STREAMER",
            "DEVICE_TYPE_TV",
            "DEVICE_TYPE_SOUNDBAR",
            "DEVICE_TYPE_COMPUTER",
            "DEVICE_TYPE_MOBILE",
            "DEVICE_TYPE_CAST",
            "DEVICE_TYPE_HEADPHONES",
            "DEVICE_TYPE_TABLET",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = DeviceType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEVICE_TYPE_UNKNOWN" => Ok(DeviceType::Unknown),
                    "DEVICE_TYPE_SPEAKER" => Ok(DeviceType::Speaker),
                    "DEVICE_TYPE_STREAMER" => Ok(DeviceType::Streamer),
                    "DEVICE_TYPE_TV" => Ok(DeviceType::Tv),
                    "DEVICE_TYPE_SOUNDBAR" => Ok(DeviceType::Soundbar),
                    "DEVICE_TYPE_COMPUTER" => Ok(DeviceType::Computer),
                    "DEVICE_TYPE_MOBILE" => Ok(DeviceType::Mobile),
                    "DEVICE_TYPE_CAST" => Ok(DeviceType::Cast),
                    "DEVICE_TYPE_HEADPHONES" => Ok(DeviceType::Headphones),
                    "DEVICE_TYPE_TABLET" => Ok(DeviceType::Tablet),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Error {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.Error", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Error {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Error;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.Error")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Error, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Error {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.Error", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ERROR_TYPE_UNKNOWN",
            Self::TrackNotFound => "ERROR_TYPE_TRACK_NOT_FOUND",
            Self::TrackNotStreamable => "ERROR_TYPE_TRACK_NOT_STREAMABLE",
            Self::TrackMusicDataInvalid => "ERROR_TYPE_TRACK_MUSIC_DATA_INVALID",
            Self::ServiceError => "ERROR_TYPE_SERVICE_ERROR",
            Self::NetworkError => "ERROR_TYPE_NETWORK_ERROR",
            Self::OtherErrors => "ERROR_TYPE_OTHER_ERRORS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_TYPE_UNKNOWN",
            "ERROR_TYPE_TRACK_NOT_FOUND",
            "ERROR_TYPE_TRACK_NOT_STREAMABLE",
            "ERROR_TYPE_TRACK_MUSIC_DATA_INVALID",
            "ERROR_TYPE_SERVICE_ERROR",
            "ERROR_TYPE_NETWORK_ERROR",
            "ERROR_TYPE_OTHER_ERRORS",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = ErrorType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERROR_TYPE_UNKNOWN" => Ok(ErrorType::Unknown),
                    "ERROR_TYPE_TRACK_NOT_FOUND" => Ok(ErrorType::TrackNotFound),
                    "ERROR_TYPE_TRACK_NOT_STREAMABLE" => Ok(ErrorType::TrackNotStreamable),
                    "ERROR_TYPE_TRACK_MUSIC_DATA_INVALID" => Ok(ErrorType::TrackMusicDataInvalid),
                    "ERROR_TYPE_SERVICE_ERROR" => Ok(ErrorType::ServiceError),
                    "ERROR_TYPE_NETWORK_ERROR" => Ok(ErrorType::NetworkError),
                    "ERROR_TYPE_OTHER_ERRORS" => Ok(ErrorType::OtherErrors),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for JoinSessionReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "JOIN_SESSION_REASON_UNKNOWN",
            Self::ControllerRequest => "JOIN_SESSION_REASON_CONTROLLER_REQUEST",
            Self::Reconnection => "JOIN_SESSION_REASON_RECONNECTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for JoinSessionReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JOIN_SESSION_REASON_UNKNOWN",
            "JOIN_SESSION_REASON_CONTROLLER_REQUEST",
            "JOIN_SESSION_REASON_RECONNECTION",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = JoinSessionReason;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "JOIN_SESSION_REASON_UNKNOWN" => Ok(JoinSessionReason::Unknown),
                    "JOIN_SESSION_REASON_CONTROLLER_REQUEST" => Ok(JoinSessionReason::ControllerRequest),
                    "JOIN_SESSION_REASON_RECONNECTION" => Ok(JoinSessionReason::Reconnection),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LoopMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "LOOP_MODE_UNKNOWN",
            Self::Off => "LOOP_MODE_OFF",
            Self::RepeatOne => "LOOP_MODE_REPEAT_ONE",
            Self::RepeatAll => "LOOP_MODE_REPEAT_ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LoopMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LOOP_MODE_UNKNOWN",
            "LOOP_MODE_OFF",
            "LOOP_MODE_REPEAT_ONE",
            "LOOP_MODE_REPEAT_ALL",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = LoopMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LOOP_MODE_UNKNOWN" => Ok(LoopMode::Unknown),
                    "LOOP_MODE_OFF" => Ok(LoopMode::Off),
                    "LOOP_MODE_REPEAT_ONE" => Ok(LoopMode::RepeatOne),
                    "LOOP_MODE_REPEAT_ALL" => Ok(LoopMode::RepeatAll),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MessageType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "MESSAGE_TYPE_UNKNOWN",
            Self::Error => "MESSAGE_TYPE_ERROR",
            Self::PlaybackError => "MESSAGE_TYPE_PLAYBACK_ERROR",
            Self::Authenticate => "MESSAGE_TYPE_AUTHENTICATE",
            Self::RndrSrvrJoinSession => "MESSAGE_TYPE_RNDR_SRVR_JOIN_SESSION",
            Self::RndrSrvrDeviceInfoUpdated => "MESSAGE_TYPE_RNDR_SRVR_DEVICE_INFO_UPDATED",
            Self::RndrSrvrStateUpdated => "MESSAGE_TYPE_RNDR_SRVR_STATE_UPDATED",
            Self::RndrSrvrRendererAction => "MESSAGE_TYPE_RNDR_SRVR_RENDERER_ACTION",
            Self::RndrSrvrVolumeChanged => "MESSAGE_TYPE_RNDR_SRVR_VOLUME_CHANGED",
            Self::RndrSrvrFileAudioQualityChanged => "MESSAGE_TYPE_RNDR_SRVR_FILE_AUDIO_QUALITY_CHANGED",
            Self::RndrSrvrDeviceAudioQualityChanged => "MESSAGE_TYPE_RNDR_SRVR_DEVICE_AUDIO_QUALITY_CHANGED",
            Self::RndrSrvrMaxAudioQualityChanged => "MESSAGE_TYPE_RNDR_SRVR_MAX_AUDIO_QUALITY_CHANGED",
            Self::RndrSrvrVolumeMuted => "MESSAGE_TYPE_RNDR_SRVR_VOLUME_MUTED",
            Self::SrvrRndrSetState => "MESSAGE_TYPE_SRVR_RNDR_SET_STATE",
            Self::SrvrRndrSetVolume => "MESSAGE_TYPE_SRVR_RNDR_SET_VOLUME",
            Self::SrvrRndrSetActive => "MESSAGE_TYPE_SRVR_RNDR_SET_ACTIVE",
            Self::SrvrRndrSetMaxAudioQuality => "MESSAGE_TYPE_SRVR_RNDR_SET_MAX_AUDIO_QUALITY",
            Self::SrvrRndrSetLoopMode => "MESSAGE_TYPE_SRVR_RNDR_SET_LOOP_MODE",
            Self::SrvrRndrSetShuffleMode => "MESSAGE_TYPE_SRVR_RNDR_SET_SHUFFLE_MODE",
            Self::SrvrRndrMuteVolume => "MESSAGE_TYPE_SRVR_RNDR_MUTE_VOLUME",
            Self::CtrlSrvrJoinSession => "MESSAGE_TYPE_CTRL_SRVR_JOIN_SESSION",
            Self::CtrlSrvrSetPlayerState => "MESSAGE_TYPE_CTRL_SRVR_SET_PLAYER_STATE",
            Self::CtrlSrvrSetActiveRenderer => "MESSAGE_TYPE_CTRL_SRVR_SET_ACTIVE_RENDERER",
            Self::CtrlSrvrSetVolume => "MESSAGE_TYPE_CTRL_SRVR_SET_VOLUME",
            Self::CtrlSrvrClearQueue => "MESSAGE_TYPE_CTRL_SRVR_CLEAR_QUEUE",
            Self::CtrlSrvrQueueLoadTracks => "MESSAGE_TYPE_CTRL_SRVR_QUEUE_LOAD_TRACKS",
            Self::CtrlSrvrQueueInsertTracks => "MESSAGE_TYPE_CTRL_SRVR_QUEUE_INSERT_TRACKS",
            Self::CtrlSrvrQueueAddTracks => "MESSAGE_TYPE_CTRL_SRVR_QUEUE_ADD_TRACKS",
            Self::CtrlSrvrQueueRemoveTracks => "MESSAGE_TYPE_CTRL_SRVR_QUEUE_REMOVE_TRACKS",
            Self::CtrlSrvrQueueReorderTracks => "MESSAGE_TYPE_CTRL_SRVR_QUEUE_REORDER_TRACKS",
            Self::CtrlSrvrSetShuffleMode => "MESSAGE_TYPE_CTRL_SRVR_SET_SHUFFLE_MODE",
            Self::CtrlSrvrSetLoopMode => "MESSAGE_TYPE_CTRL_SRVR_SET_LOOP_MODE",
            Self::CtrlSrvrMuteVolume => "MESSAGE_TYPE_CTRL_SRVR_MUTE_VOLUME",
            Self::CtrlSrvrSetMaxAudioQuality => "MESSAGE_TYPE_CTRL_SRVR_SET_MAX_AUDIO_QUALITY",
            Self::CtrlSrvrSetQueueState => "MESSAGE_TYPE_CTRL_SRVR_SET_QUEUE_STATE",
            Self::CtrlSrvrAskForQueueState => "MESSAGE_TYPE_CTRL_SRVR_ASK_FOR_QUEUE_STATE",
            Self::CtrlSrvrAskForRendererState => "MESSAGE_TYPE_CTRL_SRVR_ASK_FOR_RENDERER_STATE",
            Self::CtrlSrvrSetAutoplayMode => "MESSAGE_TYPE_CTRL_SRVR_SET_AUTOPLAY_MODE",
            Self::CtrlSrvrAutoplayLoadTracks => "MESSAGE_TYPE_CTRL_SRVR_AUTOPLAY_LOAD_TRACKS",
            Self::CtrlSrvrAutoplayRemoveTracks => "MESSAGE_TYPE_CTRL_SRVR_AUTOPLAY_REMOVE_TRACKS",
            Self::SrvrCtrlSessionState => "MESSAGE_TYPE_SRVR_CTRL_SESSION_STATE",
            Self::SrvrCtrlRendererStateUpdated => "MESSAGE_TYPE_SRVR_CTRL_RENDERER_STATE_UPDATED",
            Self::SrvrCtrlAddRenderer => "MESSAGE_TYPE_SRVR_CTRL_ADD_RENDERER",
            Self::SrvrCtrlUpdateRenderer => "MESSAGE_TYPE_SRVR_CTRL_UPDATE_RENDERER",
            Self::SrvrCtrlRemoveRenderer => "MESSAGE_TYPE_SRVR_CTRL_REMOVE_RENDERER",
            Self::SrvrCtrlActiveRendererChanged => "MESSAGE_TYPE_SRVR_CTRL_ACTIVE_RENDERER_CHANGED",
            Self::SrvrCtrlVolumeChanged => "MESSAGE_TYPE_SRVR_CTRL_VOLUME_CHANGED",
            Self::SrvrCtrlQueueErrorMessage => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_ERROR_MESSAGE",
            Self::SrvrCtrlQueueCleared => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_CLEARED",
            Self::SrvrCtrlQueueState => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_STATE",
            Self::SrvrCtrlQueueTracksLoaded => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_LOADED",
            Self::SrvrCtrlQueueTracksInserted => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_INSERTED",
            Self::SrvrCtrlQueueTracksAdded => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_ADDED",
            Self::SrvrCtrlQueueTracksRemoved => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_REMOVED",
            Self::SrvrCtrlQueueTracksReordered => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_REORDERED",
            Self::SrvrCtrlShuffleModeSet => "MESSAGE_TYPE_SRVR_CTRL_SHUFFLE_MODE_SET",
            Self::SrvrCtrlLoopModeSet => "MESSAGE_TYPE_SRVR_CTRL_LOOP_MODE_SET",
            Self::SrvrCtrlVolumeMuted => "MESSAGE_TYPE_SRVR_CTRL_VOLUME_MUTED",
            Self::SrvrCtrlMaxAudioQualityChanged => "MESSAGE_TYPE_SRVR_CTRL_MAX_AUDIO_QUALITY_CHANGED",
            Self::SrvrCtrlFileAudioQualityChanged => "MESSAGE_TYPE_SRVR_CTRL_FILE_AUDIO_QUALITY_CHANGED",
            Self::SrvrCtrlDeviceAudioQualityChanged => "MESSAGE_TYPE_SRVR_CTRL_DEVICE_AUDIO_QUALITY_CHANGED",
            Self::SrvrCtrlAutoplayModeSet => "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_MODE_SET",
            Self::SrvrCtrlAutoplayTracksLoaded => "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_TRACKS_LOADED",
            Self::SrvrCtrlAutoplayTracksRemoved => "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_TRACKS_REMOVED",
            Self::SrvrCtrlQueueTracksAddedFromAutoplay => "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_ADDED_FROM_AUTOPLAY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MessageType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MESSAGE_TYPE_UNKNOWN",
            "MESSAGE_TYPE_ERROR",
            "MESSAGE_TYPE_PLAYBACK_ERROR",
            "MESSAGE_TYPE_AUTHENTICATE",
            "MESSAGE_TYPE_RNDR_SRVR_JOIN_SESSION",
            "MESSAGE_TYPE_RNDR_SRVR_DEVICE_INFO_UPDATED",
            "MESSAGE_TYPE_RNDR_SRVR_STATE_UPDATED",
            "MESSAGE_TYPE_RNDR_SRVR_RENDERER_ACTION",
            "MESSAGE_TYPE_RNDR_SRVR_VOLUME_CHANGED",
            "MESSAGE_TYPE_RNDR_SRVR_FILE_AUDIO_QUALITY_CHANGED",
            "MESSAGE_TYPE_RNDR_SRVR_DEVICE_AUDIO_QUALITY_CHANGED",
            "MESSAGE_TYPE_RNDR_SRVR_MAX_AUDIO_QUALITY_CHANGED",
            "MESSAGE_TYPE_RNDR_SRVR_VOLUME_MUTED",
            "MESSAGE_TYPE_SRVR_RNDR_SET_STATE",
            "MESSAGE_TYPE_SRVR_RNDR_SET_VOLUME",
            "MESSAGE_TYPE_SRVR_RNDR_SET_ACTIVE",
            "MESSAGE_TYPE_SRVR_RNDR_SET_MAX_AUDIO_QUALITY",
            "MESSAGE_TYPE_SRVR_RNDR_SET_LOOP_MODE",
            "MESSAGE_TYPE_SRVR_RNDR_SET_SHUFFLE_MODE",
            "MESSAGE_TYPE_SRVR_RNDR_MUTE_VOLUME",
            "MESSAGE_TYPE_CTRL_SRVR_JOIN_SESSION",
            "MESSAGE_TYPE_CTRL_SRVR_SET_PLAYER_STATE",
            "MESSAGE_TYPE_CTRL_SRVR_SET_ACTIVE_RENDERER",
            "MESSAGE_TYPE_CTRL_SRVR_SET_VOLUME",
            "MESSAGE_TYPE_CTRL_SRVR_CLEAR_QUEUE",
            "MESSAGE_TYPE_CTRL_SRVR_QUEUE_LOAD_TRACKS",
            "MESSAGE_TYPE_CTRL_SRVR_QUEUE_INSERT_TRACKS",
            "MESSAGE_TYPE_CTRL_SRVR_QUEUE_ADD_TRACKS",
            "MESSAGE_TYPE_CTRL_SRVR_QUEUE_REMOVE_TRACKS",
            "MESSAGE_TYPE_CTRL_SRVR_QUEUE_REORDER_TRACKS",
            "MESSAGE_TYPE_CTRL_SRVR_SET_SHUFFLE_MODE",
            "MESSAGE_TYPE_CTRL_SRVR_SET_LOOP_MODE",
            "MESSAGE_TYPE_CTRL_SRVR_MUTE_VOLUME",
            "MESSAGE_TYPE_CTRL_SRVR_SET_MAX_AUDIO_QUALITY",
            "MESSAGE_TYPE_CTRL_SRVR_SET_QUEUE_STATE",
            "MESSAGE_TYPE_CTRL_SRVR_ASK_FOR_QUEUE_STATE",
            "MESSAGE_TYPE_CTRL_SRVR_ASK_FOR_RENDERER_STATE",
            "MESSAGE_TYPE_CTRL_SRVR_SET_AUTOPLAY_MODE",
            "MESSAGE_TYPE_CTRL_SRVR_AUTOPLAY_LOAD_TRACKS",
            "MESSAGE_TYPE_CTRL_SRVR_AUTOPLAY_REMOVE_TRACKS",
            "MESSAGE_TYPE_SRVR_CTRL_SESSION_STATE",
            "MESSAGE_TYPE_SRVR_CTRL_RENDERER_STATE_UPDATED",
            "MESSAGE_TYPE_SRVR_CTRL_ADD_RENDERER",
            "MESSAGE_TYPE_SRVR_CTRL_UPDATE_RENDERER",
            "MESSAGE_TYPE_SRVR_CTRL_REMOVE_RENDERER",
            "MESSAGE_TYPE_SRVR_CTRL_ACTIVE_RENDERER_CHANGED",
            "MESSAGE_TYPE_SRVR_CTRL_VOLUME_CHANGED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_ERROR_MESSAGE",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_CLEARED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_STATE",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_LOADED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_INSERTED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_ADDED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_REMOVED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_REORDERED",
            "MESSAGE_TYPE_SRVR_CTRL_SHUFFLE_MODE_SET",
            "MESSAGE_TYPE_SRVR_CTRL_LOOP_MODE_SET",
            "MESSAGE_TYPE_SRVR_CTRL_VOLUME_MUTED",
            "MESSAGE_TYPE_SRVR_CTRL_MAX_AUDIO_QUALITY_CHANGED",
            "MESSAGE_TYPE_SRVR_CTRL_FILE_AUDIO_QUALITY_CHANGED",
            "MESSAGE_TYPE_SRVR_CTRL_DEVICE_AUDIO_QUALITY_CHANGED",
            "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_MODE_SET",
            "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_TRACKS_LOADED",
            "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_TRACKS_REMOVED",
            "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_ADDED_FROM_AUTOPLAY",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = MessageType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MESSAGE_TYPE_UNKNOWN" => Ok(MessageType::Unknown),
                    "MESSAGE_TYPE_ERROR" => Ok(MessageType::Error),
                    "MESSAGE_TYPE_PLAYBACK_ERROR" => Ok(MessageType::PlaybackError),
                    "MESSAGE_TYPE_AUTHENTICATE" => Ok(MessageType::Authenticate),
                    "MESSAGE_TYPE_RNDR_SRVR_JOIN_SESSION" => Ok(MessageType::RndrSrvrJoinSession),
                    "MESSAGE_TYPE_RNDR_SRVR_DEVICE_INFO_UPDATED" => Ok(MessageType::RndrSrvrDeviceInfoUpdated),
                    "MESSAGE_TYPE_RNDR_SRVR_STATE_UPDATED" => Ok(MessageType::RndrSrvrStateUpdated),
                    "MESSAGE_TYPE_RNDR_SRVR_RENDERER_ACTION" => Ok(MessageType::RndrSrvrRendererAction),
                    "MESSAGE_TYPE_RNDR_SRVR_VOLUME_CHANGED" => Ok(MessageType::RndrSrvrVolumeChanged),
                    "MESSAGE_TYPE_RNDR_SRVR_FILE_AUDIO_QUALITY_CHANGED" => Ok(MessageType::RndrSrvrFileAudioQualityChanged),
                    "MESSAGE_TYPE_RNDR_SRVR_DEVICE_AUDIO_QUALITY_CHANGED" => Ok(MessageType::RndrSrvrDeviceAudioQualityChanged),
                    "MESSAGE_TYPE_RNDR_SRVR_MAX_AUDIO_QUALITY_CHANGED" => Ok(MessageType::RndrSrvrMaxAudioQualityChanged),
                    "MESSAGE_TYPE_RNDR_SRVR_VOLUME_MUTED" => Ok(MessageType::RndrSrvrVolumeMuted),
                    "MESSAGE_TYPE_SRVR_RNDR_SET_STATE" => Ok(MessageType::SrvrRndrSetState),
                    "MESSAGE_TYPE_SRVR_RNDR_SET_VOLUME" => Ok(MessageType::SrvrRndrSetVolume),
                    "MESSAGE_TYPE_SRVR_RNDR_SET_ACTIVE" => Ok(MessageType::SrvrRndrSetActive),
                    "MESSAGE_TYPE_SRVR_RNDR_SET_MAX_AUDIO_QUALITY" => Ok(MessageType::SrvrRndrSetMaxAudioQuality),
                    "MESSAGE_TYPE_SRVR_RNDR_SET_LOOP_MODE" => Ok(MessageType::SrvrRndrSetLoopMode),
                    "MESSAGE_TYPE_SRVR_RNDR_SET_SHUFFLE_MODE" => Ok(MessageType::SrvrRndrSetShuffleMode),
                    "MESSAGE_TYPE_SRVR_RNDR_MUTE_VOLUME" => Ok(MessageType::SrvrRndrMuteVolume),
                    "MESSAGE_TYPE_CTRL_SRVR_JOIN_SESSION" => Ok(MessageType::CtrlSrvrJoinSession),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_PLAYER_STATE" => Ok(MessageType::CtrlSrvrSetPlayerState),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_ACTIVE_RENDERER" => Ok(MessageType::CtrlSrvrSetActiveRenderer),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_VOLUME" => Ok(MessageType::CtrlSrvrSetVolume),
                    "MESSAGE_TYPE_CTRL_SRVR_CLEAR_QUEUE" => Ok(MessageType::CtrlSrvrClearQueue),
                    "MESSAGE_TYPE_CTRL_SRVR_QUEUE_LOAD_TRACKS" => Ok(MessageType::CtrlSrvrQueueLoadTracks),
                    "MESSAGE_TYPE_CTRL_SRVR_QUEUE_INSERT_TRACKS" => Ok(MessageType::CtrlSrvrQueueInsertTracks),
                    "MESSAGE_TYPE_CTRL_SRVR_QUEUE_ADD_TRACKS" => Ok(MessageType::CtrlSrvrQueueAddTracks),
                    "MESSAGE_TYPE_CTRL_SRVR_QUEUE_REMOVE_TRACKS" => Ok(MessageType::CtrlSrvrQueueRemoveTracks),
                    "MESSAGE_TYPE_CTRL_SRVR_QUEUE_REORDER_TRACKS" => Ok(MessageType::CtrlSrvrQueueReorderTracks),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_SHUFFLE_MODE" => Ok(MessageType::CtrlSrvrSetShuffleMode),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_LOOP_MODE" => Ok(MessageType::CtrlSrvrSetLoopMode),
                    "MESSAGE_TYPE_CTRL_SRVR_MUTE_VOLUME" => Ok(MessageType::CtrlSrvrMuteVolume),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_MAX_AUDIO_QUALITY" => Ok(MessageType::CtrlSrvrSetMaxAudioQuality),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_QUEUE_STATE" => Ok(MessageType::CtrlSrvrSetQueueState),
                    "MESSAGE_TYPE_CTRL_SRVR_ASK_FOR_QUEUE_STATE" => Ok(MessageType::CtrlSrvrAskForQueueState),
                    "MESSAGE_TYPE_CTRL_SRVR_ASK_FOR_RENDERER_STATE" => Ok(MessageType::CtrlSrvrAskForRendererState),
                    "MESSAGE_TYPE_CTRL_SRVR_SET_AUTOPLAY_MODE" => Ok(MessageType::CtrlSrvrSetAutoplayMode),
                    "MESSAGE_TYPE_CTRL_SRVR_AUTOPLAY_LOAD_TRACKS" => Ok(MessageType::CtrlSrvrAutoplayLoadTracks),
                    "MESSAGE_TYPE_CTRL_SRVR_AUTOPLAY_REMOVE_TRACKS" => Ok(MessageType::CtrlSrvrAutoplayRemoveTracks),
                    "MESSAGE_TYPE_SRVR_CTRL_SESSION_STATE" => Ok(MessageType::SrvrCtrlSessionState),
                    "MESSAGE_TYPE_SRVR_CTRL_RENDERER_STATE_UPDATED" => Ok(MessageType::SrvrCtrlRendererStateUpdated),
                    "MESSAGE_TYPE_SRVR_CTRL_ADD_RENDERER" => Ok(MessageType::SrvrCtrlAddRenderer),
                    "MESSAGE_TYPE_SRVR_CTRL_UPDATE_RENDERER" => Ok(MessageType::SrvrCtrlUpdateRenderer),
                    "MESSAGE_TYPE_SRVR_CTRL_REMOVE_RENDERER" => Ok(MessageType::SrvrCtrlRemoveRenderer),
                    "MESSAGE_TYPE_SRVR_CTRL_ACTIVE_RENDERER_CHANGED" => Ok(MessageType::SrvrCtrlActiveRendererChanged),
                    "MESSAGE_TYPE_SRVR_CTRL_VOLUME_CHANGED" => Ok(MessageType::SrvrCtrlVolumeChanged),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_ERROR_MESSAGE" => Ok(MessageType::SrvrCtrlQueueErrorMessage),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_CLEARED" => Ok(MessageType::SrvrCtrlQueueCleared),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_STATE" => Ok(MessageType::SrvrCtrlQueueState),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_LOADED" => Ok(MessageType::SrvrCtrlQueueTracksLoaded),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_INSERTED" => Ok(MessageType::SrvrCtrlQueueTracksInserted),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_ADDED" => Ok(MessageType::SrvrCtrlQueueTracksAdded),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_REMOVED" => Ok(MessageType::SrvrCtrlQueueTracksRemoved),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_REORDERED" => Ok(MessageType::SrvrCtrlQueueTracksReordered),
                    "MESSAGE_TYPE_SRVR_CTRL_SHUFFLE_MODE_SET" => Ok(MessageType::SrvrCtrlShuffleModeSet),
                    "MESSAGE_TYPE_SRVR_CTRL_LOOP_MODE_SET" => Ok(MessageType::SrvrCtrlLoopModeSet),
                    "MESSAGE_TYPE_SRVR_CTRL_VOLUME_MUTED" => Ok(MessageType::SrvrCtrlVolumeMuted),
                    "MESSAGE_TYPE_SRVR_CTRL_MAX_AUDIO_QUALITY_CHANGED" => Ok(MessageType::SrvrCtrlMaxAudioQualityChanged),
                    "MESSAGE_TYPE_SRVR_CTRL_FILE_AUDIO_QUALITY_CHANGED" => Ok(MessageType::SrvrCtrlFileAudioQualityChanged),
                    "MESSAGE_TYPE_SRVR_CTRL_DEVICE_AUDIO_QUALITY_CHANGED" => Ok(MessageType::SrvrCtrlDeviceAudioQualityChanged),
                    "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_MODE_SET" => Ok(MessageType::SrvrCtrlAutoplayModeSet),
                    "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_TRACKS_LOADED" => Ok(MessageType::SrvrCtrlAutoplayTracksLoaded),
                    "MESSAGE_TYPE_SRVR_CTRL_AUTOPLAY_TRACKS_REMOVED" => Ok(MessageType::SrvrCtrlAutoplayTracksRemoved),
                    "MESSAGE_TYPE_SRVR_CTRL_QUEUE_TRACKS_ADDED_FROM_AUTOPLAY" => Ok(MessageType::SrvrCtrlQueueTracksAddedFromAutoplay),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "NETWORK_TYPE_UNKNOWN",
            Self::Wifi => "NETWORK_TYPE_WIFI",
            Self::Cellular => "NETWORK_TYPE_CELLULAR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NetworkType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NETWORK_TYPE_UNKNOWN",
            "NETWORK_TYPE_WIFI",
            "NETWORK_TYPE_CELLULAR",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = NetworkType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NETWORK_TYPE_UNKNOWN" => Ok(NetworkType::Unknown),
                    "NETWORK_TYPE_WIFI" => Ok(NetworkType::Wifi),
                    "NETWORK_TYPE_CELLULAR" => Ok(NetworkType::Cellular),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PlaybackError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if self.queue_item_id != 0 {
            len += 1;
        }
        if self.error_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.PlaybackError", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if self.queue_item_id != 0 {
            struct_ser.serialize_field("queueItemId", &self.queue_item_id)?;
        }
        if self.error_type != 0 {
            let v = ErrorType::try_from(self.error_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.error_type)))?;
            struct_ser.serialize_field("errorType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlaybackError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "queue_item_id",
            "queueItemId",
            "error_type",
            "errorType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            QueueItemId,
            ErrorType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "queueItemId" | "queue_item_id" => Ok(GeneratedField::QueueItemId),
                            "errorType" | "error_type" => Ok(GeneratedField::ErrorType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlaybackError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.PlaybackError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlaybackError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut queue_item_id__ = None;
                let mut error_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::QueueItemId => {
                            if queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemId"));
                            }
                            queue_item_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ErrorType => {
                            if error_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorType"));
                            }
                            error_type__ = Some(map_.next_value::<ErrorType>()? as i32);
                        }
                    }
                }
                Ok(PlaybackError {
                    queue_version: queue_version__,
                    queue_item_id: queue_item_id__.unwrap_or_default(),
                    error_type: error_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.PlaybackError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlayingState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "PLAYING_STATE_UNKNOWN",
            Self::Stopped => "PLAYING_STATE_STOPPED",
            Self::Playing => "PLAYING_STATE_PLAYING",
            Self::Paused => "PLAYING_STATE_PAUSED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PlayingState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PLAYING_STATE_UNKNOWN",
            "PLAYING_STATE_STOPPED",
            "PLAYING_STATE_PLAYING",
            "PLAYING_STATE_PAUSED",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = PlayingState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PLAYING_STATE_UNKNOWN" => Ok(PlayingState::Unknown),
                    "PLAYING_STATE_STOPPED" => Ok(PlayingState::Stopped),
                    "PLAYING_STATE_PLAYING" => Ok(PlayingState::Playing),
                    "PLAYING_STATE_PAUSED" => Ok(PlayingState::Paused),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Position {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp != 0 {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.Position", len)?;
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if self.value != 0 {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Position {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Position, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Position {
                    timestamp: timestamp__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.Position", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QConnectBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.messages_time != 0 {
            len += 1;
        }
        if self.messages_id != 0 {
            len += 1;
        }
        if !self.messages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QConnectBatch", len)?;
        if self.messages_time != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("messagesTime", ToString::to_string(&self.messages_time).as_str())?;
        }
        if self.messages_id != 0 {
            struct_ser.serialize_field("messagesId", &self.messages_id)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QConnectBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "messages_time",
            "messagesTime",
            "messages_id",
            "messagesId",
            "messages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessagesTime,
            MessagesId,
            Messages,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messagesTime" | "messages_time" => Ok(GeneratedField::MessagesTime),
                            "messagesId" | "messages_id" => Ok(GeneratedField::MessagesId),
                            "messages" => Ok(GeneratedField::Messages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QConnectBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QConnectBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QConnectBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut messages_time__ = None;
                let mut messages_id__ = None;
                let mut messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessagesTime => {
                            if messages_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagesTime"));
                            }
                            messages_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MessagesId => {
                            if messages_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messagesId"));
                            }
                            messages_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QConnectBatch {
                    messages_time: messages_time__.unwrap_or_default(),
                    messages_id: messages_id__.unwrap_or_default(),
                    messages: messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QConnectBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QConnectMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.message_type != 0 {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if self.playback_error.is_some() {
            len += 1;
        }
        if self.authenticate.is_some() {
            len += 1;
        }
        if self.rndr_srvr_join_session.is_some() {
            len += 1;
        }
        if self.rndr_srvr_device_info_updated.is_some() {
            len += 1;
        }
        if self.rndr_srvr_state_updated.is_some() {
            len += 1;
        }
        if self.rndr_srvr_renderer_action.is_some() {
            len += 1;
        }
        if self.rndr_srvr_volume_changed.is_some() {
            len += 1;
        }
        if self.rndr_srvr_file_audio_quality_changed.is_some() {
            len += 1;
        }
        if self.rndr_srvr_device_audio_quality_changed.is_some() {
            len += 1;
        }
        if self.rndr_srvr_max_audio_quality_changed.is_some() {
            len += 1;
        }
        if self.rndr_srvr_volume_muted.is_some() {
            len += 1;
        }
        if self.srvr_rndr_set_state.is_some() {
            len += 1;
        }
        if self.srvr_rndr_set_volume.is_some() {
            len += 1;
        }
        if self.srvr_rndr_set_active.is_some() {
            len += 1;
        }
        if self.srvr_rndr_set_max_audio_quality.is_some() {
            len += 1;
        }
        if self.srvr_rndr_set_loop_mode.is_some() {
            len += 1;
        }
        if self.srvr_rndr_set_shuffle_mode.is_some() {
            len += 1;
        }
        if self.srvr_rndr_mute_volume.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_join_session.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_player_state.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_active_renderer.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_volume.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_clear_queue.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_queue_load_tracks.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_queue_insert_tracks.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_queue_add_tracks.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_queue_remove_tracks.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_queue_reorder_tracks.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_shuffle_mode.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_loop_mode.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_mute_volume.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_max_audio_quality.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_queue_state.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_ask_for_queue_state.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_ask_for_renderer_state.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_set_autoplay_mode.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_autoplay_load_tracks.is_some() {
            len += 1;
        }
        if self.ctrl_srvr_autoplay_remove_tracks.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_session_state.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_renderer_state_updated.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_add_renderer.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_update_renderer.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_remove_renderer.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_active_renderer_changed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_volume_changed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_error_message.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_cleared.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_state.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_tracks_loaded.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_tracks_inserted.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_tracks_added.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_tracks_removed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_tracks_reordered.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_shuffle_mode_set.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_loop_mode_set.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_volume_muted.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_max_audio_quality_changed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_file_audio_quality_changed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_device_audio_quality_changed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_autoplay_mode_set.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_autoplay_tracks_loaded.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_autoplay_tracks_removed.is_some() {
            len += 1;
        }
        if self.srvr_ctrl_queue_tracks_added_from_autoplay.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QConnectMessage", len)?;
        if self.message_type != 0 {
            let v = MessageType::try_from(self.message_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.message_type)))?;
            struct_ser.serialize_field("messageType", &v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.playback_error.as_ref() {
            struct_ser.serialize_field("playbackError", v)?;
        }
        if let Some(v) = self.authenticate.as_ref() {
            struct_ser.serialize_field("authenticate", v)?;
        }
        if let Some(v) = self.rndr_srvr_join_session.as_ref() {
            struct_ser.serialize_field("rndrSrvrJoinSession", v)?;
        }
        if let Some(v) = self.rndr_srvr_device_info_updated.as_ref() {
            struct_ser.serialize_field("rndrSrvrDeviceInfoUpdated", v)?;
        }
        if let Some(v) = self.rndr_srvr_state_updated.as_ref() {
            struct_ser.serialize_field("rndrSrvrStateUpdated", v)?;
        }
        if let Some(v) = self.rndr_srvr_renderer_action.as_ref() {
            struct_ser.serialize_field("rndrSrvrRendererAction", v)?;
        }
        if let Some(v) = self.rndr_srvr_volume_changed.as_ref() {
            struct_ser.serialize_field("rndrSrvrVolumeChanged", v)?;
        }
        if let Some(v) = self.rndr_srvr_file_audio_quality_changed.as_ref() {
            struct_ser.serialize_field("rndrSrvrFileAudioQualityChanged", v)?;
        }
        if let Some(v) = self.rndr_srvr_device_audio_quality_changed.as_ref() {
            struct_ser.serialize_field("rndrSrvrDeviceAudioQualityChanged", v)?;
        }
        if let Some(v) = self.rndr_srvr_max_audio_quality_changed.as_ref() {
            struct_ser.serialize_field("rndrSrvrMaxAudioQualityChanged", v)?;
        }
        if let Some(v) = self.rndr_srvr_volume_muted.as_ref() {
            struct_ser.serialize_field("rndrSrvrVolumeMuted", v)?;
        }
        if let Some(v) = self.srvr_rndr_set_state.as_ref() {
            struct_ser.serialize_field("srvrRndrSetState", v)?;
        }
        if let Some(v) = self.srvr_rndr_set_volume.as_ref() {
            struct_ser.serialize_field("srvrRndrSetVolume", v)?;
        }
        if let Some(v) = self.srvr_rndr_set_active.as_ref() {
            struct_ser.serialize_field("srvrRndrSetActive", v)?;
        }
        if let Some(v) = self.srvr_rndr_set_max_audio_quality.as_ref() {
            struct_ser.serialize_field("srvrRndrSetMaxAudioQuality", v)?;
        }
        if let Some(v) = self.srvr_rndr_set_loop_mode.as_ref() {
            struct_ser.serialize_field("srvrRndrSetLoopMode", v)?;
        }
        if let Some(v) = self.srvr_rndr_set_shuffle_mode.as_ref() {
            struct_ser.serialize_field("srvrRndrSetShuffleMode", v)?;
        }
        if let Some(v) = self.srvr_rndr_mute_volume.as_ref() {
            struct_ser.serialize_field("srvrRndrMuteVolume", v)?;
        }
        if let Some(v) = self.ctrl_srvr_join_session.as_ref() {
            struct_ser.serialize_field("ctrlSrvrJoinSession", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_player_state.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetPlayerState", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_active_renderer.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetActiveRenderer", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_volume.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetVolume", v)?;
        }
        if let Some(v) = self.ctrl_srvr_clear_queue.as_ref() {
            struct_ser.serialize_field("ctrlSrvrClearQueue", v)?;
        }
        if let Some(v) = self.ctrl_srvr_queue_load_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrQueueLoadTracks", v)?;
        }
        if let Some(v) = self.ctrl_srvr_queue_insert_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrQueueInsertTracks", v)?;
        }
        if let Some(v) = self.ctrl_srvr_queue_add_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrQueueAddTracks", v)?;
        }
        if let Some(v) = self.ctrl_srvr_queue_remove_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrQueueRemoveTracks", v)?;
        }
        if let Some(v) = self.ctrl_srvr_queue_reorder_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrQueueReorderTracks", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_shuffle_mode.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetShuffleMode", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_loop_mode.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetLoopMode", v)?;
        }
        if let Some(v) = self.ctrl_srvr_mute_volume.as_ref() {
            struct_ser.serialize_field("ctrlSrvrMuteVolume", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_max_audio_quality.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetMaxAudioQuality", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_queue_state.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetQueueState", v)?;
        }
        if let Some(v) = self.ctrl_srvr_ask_for_queue_state.as_ref() {
            struct_ser.serialize_field("ctrlSrvrAskForQueueState", v)?;
        }
        if let Some(v) = self.ctrl_srvr_ask_for_renderer_state.as_ref() {
            struct_ser.serialize_field("ctrlSrvrAskForRendererState", v)?;
        }
        if let Some(v) = self.ctrl_srvr_set_autoplay_mode.as_ref() {
            struct_ser.serialize_field("ctrlSrvrSetAutoplayMode", v)?;
        }
        if let Some(v) = self.ctrl_srvr_autoplay_load_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrAutoplayLoadTracks", v)?;
        }
        if let Some(v) = self.ctrl_srvr_autoplay_remove_tracks.as_ref() {
            struct_ser.serialize_field("ctrlSrvrAutoplayRemoveTracks", v)?;
        }
        if let Some(v) = self.srvr_ctrl_session_state.as_ref() {
            struct_ser.serialize_field("srvrCtrlSessionState", v)?;
        }
        if let Some(v) = self.srvr_ctrl_renderer_state_updated.as_ref() {
            struct_ser.serialize_field("srvrCtrlRendererStateUpdated", v)?;
        }
        if let Some(v) = self.srvr_ctrl_add_renderer.as_ref() {
            struct_ser.serialize_field("srvrCtrlAddRenderer", v)?;
        }
        if let Some(v) = self.srvr_ctrl_update_renderer.as_ref() {
            struct_ser.serialize_field("srvrCtrlUpdateRenderer", v)?;
        }
        if let Some(v) = self.srvr_ctrl_remove_renderer.as_ref() {
            struct_ser.serialize_field("srvrCtrlRemoveRenderer", v)?;
        }
        if let Some(v) = self.srvr_ctrl_active_renderer_changed.as_ref() {
            struct_ser.serialize_field("srvrCtrlActiveRendererChanged", v)?;
        }
        if let Some(v) = self.srvr_ctrl_volume_changed.as_ref() {
            struct_ser.serialize_field("srvrCtrlVolumeChanged", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_error_message.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueErrorMessage", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_cleared.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueCleared", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_state.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueState", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_tracks_loaded.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueTracksLoaded", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_tracks_inserted.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueTracksInserted", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_tracks_added.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueTracksAdded", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_tracks_removed.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueTracksRemoved", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_tracks_reordered.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueTracksReordered", v)?;
        }
        if let Some(v) = self.srvr_ctrl_shuffle_mode_set.as_ref() {
            struct_ser.serialize_field("srvrCtrlShuffleModeSet", v)?;
        }
        if let Some(v) = self.srvr_ctrl_loop_mode_set.as_ref() {
            struct_ser.serialize_field("srvrCtrlLoopModeSet", v)?;
        }
        if let Some(v) = self.srvr_ctrl_volume_muted.as_ref() {
            struct_ser.serialize_field("srvrCtrlVolumeMuted", v)?;
        }
        if let Some(v) = self.srvr_ctrl_max_audio_quality_changed.as_ref() {
            struct_ser.serialize_field("srvrCtrlMaxAudioQualityChanged", v)?;
        }
        if let Some(v) = self.srvr_ctrl_file_audio_quality_changed.as_ref() {
            struct_ser.serialize_field("srvrCtrlFileAudioQualityChanged", v)?;
        }
        if let Some(v) = self.srvr_ctrl_device_audio_quality_changed.as_ref() {
            struct_ser.serialize_field("srvrCtrlDeviceAudioQualityChanged", v)?;
        }
        if let Some(v) = self.srvr_ctrl_autoplay_mode_set.as_ref() {
            struct_ser.serialize_field("srvrCtrlAutoplayModeSet", v)?;
        }
        if let Some(v) = self.srvr_ctrl_autoplay_tracks_loaded.as_ref() {
            struct_ser.serialize_field("srvrCtrlAutoplayTracksLoaded", v)?;
        }
        if let Some(v) = self.srvr_ctrl_autoplay_tracks_removed.as_ref() {
            struct_ser.serialize_field("srvrCtrlAutoplayTracksRemoved", v)?;
        }
        if let Some(v) = self.srvr_ctrl_queue_tracks_added_from_autoplay.as_ref() {
            struct_ser.serialize_field("srvrCtrlQueueTracksAddedFromAutoplay", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QConnectMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_type",
            "messageType",
            "error",
            "playback_error",
            "playbackError",
            "authenticate",
            "rndr_srvr_join_session",
            "rndrSrvrJoinSession",
            "rndr_srvr_device_info_updated",
            "rndrSrvrDeviceInfoUpdated",
            "rndr_srvr_state_updated",
            "rndrSrvrStateUpdated",
            "rndr_srvr_renderer_action",
            "rndrSrvrRendererAction",
            "rndr_srvr_volume_changed",
            "rndrSrvrVolumeChanged",
            "rndr_srvr_file_audio_quality_changed",
            "rndrSrvrFileAudioQualityChanged",
            "rndr_srvr_device_audio_quality_changed",
            "rndrSrvrDeviceAudioQualityChanged",
            "rndr_srvr_max_audio_quality_changed",
            "rndrSrvrMaxAudioQualityChanged",
            "rndr_srvr_volume_muted",
            "rndrSrvrVolumeMuted",
            "srvr_rndr_set_state",
            "srvrRndrSetState",
            "srvr_rndr_set_volume",
            "srvrRndrSetVolume",
            "srvr_rndr_set_active",
            "srvrRndrSetActive",
            "srvr_rndr_set_max_audio_quality",
            "srvrRndrSetMaxAudioQuality",
            "srvr_rndr_set_loop_mode",
            "srvrRndrSetLoopMode",
            "srvr_rndr_set_shuffle_mode",
            "srvrRndrSetShuffleMode",
            "srvr_rndr_mute_volume",
            "srvrRndrMuteVolume",
            "ctrl_srvr_join_session",
            "ctrlSrvrJoinSession",
            "ctrl_srvr_set_player_state",
            "ctrlSrvrSetPlayerState",
            "ctrl_srvr_set_active_renderer",
            "ctrlSrvrSetActiveRenderer",
            "ctrl_srvr_set_volume",
            "ctrlSrvrSetVolume",
            "ctrl_srvr_clear_queue",
            "ctrlSrvrClearQueue",
            "ctrl_srvr_queue_load_tracks",
            "ctrlSrvrQueueLoadTracks",
            "ctrl_srvr_queue_insert_tracks",
            "ctrlSrvrQueueInsertTracks",
            "ctrl_srvr_queue_add_tracks",
            "ctrlSrvrQueueAddTracks",
            "ctrl_srvr_queue_remove_tracks",
            "ctrlSrvrQueueRemoveTracks",
            "ctrl_srvr_queue_reorder_tracks",
            "ctrlSrvrQueueReorderTracks",
            "ctrl_srvr_set_shuffle_mode",
            "ctrlSrvrSetShuffleMode",
            "ctrl_srvr_set_loop_mode",
            "ctrlSrvrSetLoopMode",
            "ctrl_srvr_mute_volume",
            "ctrlSrvrMuteVolume",
            "ctrl_srvr_set_max_audio_quality",
            "ctrlSrvrSetMaxAudioQuality",
            "ctrl_srvr_set_queue_state",
            "ctrlSrvrSetQueueState",
            "ctrl_srvr_ask_for_queue_state",
            "ctrlSrvrAskForQueueState",
            "ctrl_srvr_ask_for_renderer_state",
            "ctrlSrvrAskForRendererState",
            "ctrl_srvr_set_autoplay_mode",
            "ctrlSrvrSetAutoplayMode",
            "ctrl_srvr_autoplay_load_tracks",
            "ctrlSrvrAutoplayLoadTracks",
            "ctrl_srvr_autoplay_remove_tracks",
            "ctrlSrvrAutoplayRemoveTracks",
            "srvr_ctrl_session_state",
            "srvrCtrlSessionState",
            "srvr_ctrl_renderer_state_updated",
            "srvrCtrlRendererStateUpdated",
            "srvr_ctrl_add_renderer",
            "srvrCtrlAddRenderer",
            "srvr_ctrl_update_renderer",
            "srvrCtrlUpdateRenderer",
            "srvr_ctrl_remove_renderer",
            "srvrCtrlRemoveRenderer",
            "srvr_ctrl_active_renderer_changed",
            "srvrCtrlActiveRendererChanged",
            "srvr_ctrl_volume_changed",
            "srvrCtrlVolumeChanged",
            "srvr_ctrl_queue_error_message",
            "srvrCtrlQueueErrorMessage",
            "srvr_ctrl_queue_cleared",
            "srvrCtrlQueueCleared",
            "srvr_ctrl_queue_state",
            "srvrCtrlQueueState",
            "srvr_ctrl_queue_tracks_loaded",
            "srvrCtrlQueueTracksLoaded",
            "srvr_ctrl_queue_tracks_inserted",
            "srvrCtrlQueueTracksInserted",
            "srvr_ctrl_queue_tracks_added",
            "srvrCtrlQueueTracksAdded",
            "srvr_ctrl_queue_tracks_removed",
            "srvrCtrlQueueTracksRemoved",
            "srvr_ctrl_queue_tracks_reordered",
            "srvrCtrlQueueTracksReordered",
            "srvr_ctrl_shuffle_mode_set",
            "srvrCtrlShuffleModeSet",
            "srvr_ctrl_loop_mode_set",
            "srvrCtrlLoopModeSet",
            "srvr_ctrl_volume_muted",
            "srvrCtrlVolumeMuted",
            "srvr_ctrl_max_audio_quality_changed",
            "srvrCtrlMaxAudioQualityChanged",
            "srvr_ctrl_file_audio_quality_changed",
            "srvrCtrlFileAudioQualityChanged",
            "srvr_ctrl_device_audio_quality_changed",
            "srvrCtrlDeviceAudioQualityChanged",
            "srvr_ctrl_autoplay_mode_set",
            "srvrCtrlAutoplayModeSet",
            "srvr_ctrl_autoplay_tracks_loaded",
            "srvrCtrlAutoplayTracksLoaded",
            "srvr_ctrl_autoplay_tracks_removed",
            "srvrCtrlAutoplayTracksRemoved",
            "srvr_ctrl_queue_tracks_added_from_autoplay",
            "srvrCtrlQueueTracksAddedFromAutoplay",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageType,
            Error,
            PlaybackError,
            Authenticate,
            RndrSrvrJoinSession,
            RndrSrvrDeviceInfoUpdated,
            RndrSrvrStateUpdated,
            RndrSrvrRendererAction,
            RndrSrvrVolumeChanged,
            RndrSrvrFileAudioQualityChanged,
            RndrSrvrDeviceAudioQualityChanged,
            RndrSrvrMaxAudioQualityChanged,
            RndrSrvrVolumeMuted,
            SrvrRndrSetState,
            SrvrRndrSetVolume,
            SrvrRndrSetActive,
            SrvrRndrSetMaxAudioQuality,
            SrvrRndrSetLoopMode,
            SrvrRndrSetShuffleMode,
            SrvrRndrMuteVolume,
            CtrlSrvrJoinSession,
            CtrlSrvrSetPlayerState,
            CtrlSrvrSetActiveRenderer,
            CtrlSrvrSetVolume,
            CtrlSrvrClearQueue,
            CtrlSrvrQueueLoadTracks,
            CtrlSrvrQueueInsertTracks,
            CtrlSrvrQueueAddTracks,
            CtrlSrvrQueueRemoveTracks,
            CtrlSrvrQueueReorderTracks,
            CtrlSrvrSetShuffleMode,
            CtrlSrvrSetLoopMode,
            CtrlSrvrMuteVolume,
            CtrlSrvrSetMaxAudioQuality,
            CtrlSrvrSetQueueState,
            CtrlSrvrAskForQueueState,
            CtrlSrvrAskForRendererState,
            CtrlSrvrSetAutoplayMode,
            CtrlSrvrAutoplayLoadTracks,
            CtrlSrvrAutoplayRemoveTracks,
            SrvrCtrlSessionState,
            SrvrCtrlRendererStateUpdated,
            SrvrCtrlAddRenderer,
            SrvrCtrlUpdateRenderer,
            SrvrCtrlRemoveRenderer,
            SrvrCtrlActiveRendererChanged,
            SrvrCtrlVolumeChanged,
            SrvrCtrlQueueErrorMessage,
            SrvrCtrlQueueCleared,
            SrvrCtrlQueueState,
            SrvrCtrlQueueTracksLoaded,
            SrvrCtrlQueueTracksInserted,
            SrvrCtrlQueueTracksAdded,
            SrvrCtrlQueueTracksRemoved,
            SrvrCtrlQueueTracksReordered,
            SrvrCtrlShuffleModeSet,
            SrvrCtrlLoopModeSet,
            SrvrCtrlVolumeMuted,
            SrvrCtrlMaxAudioQualityChanged,
            SrvrCtrlFileAudioQualityChanged,
            SrvrCtrlDeviceAudioQualityChanged,
            SrvrCtrlAutoplayModeSet,
            SrvrCtrlAutoplayTracksLoaded,
            SrvrCtrlAutoplayTracksRemoved,
            SrvrCtrlQueueTracksAddedFromAutoplay,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messageType" | "message_type" => Ok(GeneratedField::MessageType),
                            "error" => Ok(GeneratedField::Error),
                            "playbackError" | "playback_error" => Ok(GeneratedField::PlaybackError),
                            "authenticate" => Ok(GeneratedField::Authenticate),
                            "rndrSrvrJoinSession" | "rndr_srvr_join_session" => Ok(GeneratedField::RndrSrvrJoinSession),
                            "rndrSrvrDeviceInfoUpdated" | "rndr_srvr_device_info_updated" => Ok(GeneratedField::RndrSrvrDeviceInfoUpdated),
                            "rndrSrvrStateUpdated" | "rndr_srvr_state_updated" => Ok(GeneratedField::RndrSrvrStateUpdated),
                            "rndrSrvrRendererAction" | "rndr_srvr_renderer_action" => Ok(GeneratedField::RndrSrvrRendererAction),
                            "rndrSrvrVolumeChanged" | "rndr_srvr_volume_changed" => Ok(GeneratedField::RndrSrvrVolumeChanged),
                            "rndrSrvrFileAudioQualityChanged" | "rndr_srvr_file_audio_quality_changed" => Ok(GeneratedField::RndrSrvrFileAudioQualityChanged),
                            "rndrSrvrDeviceAudioQualityChanged" | "rndr_srvr_device_audio_quality_changed" => Ok(GeneratedField::RndrSrvrDeviceAudioQualityChanged),
                            "rndrSrvrMaxAudioQualityChanged" | "rndr_srvr_max_audio_quality_changed" => Ok(GeneratedField::RndrSrvrMaxAudioQualityChanged),
                            "rndrSrvrVolumeMuted" | "rndr_srvr_volume_muted" => Ok(GeneratedField::RndrSrvrVolumeMuted),
                            "srvrRndrSetState" | "srvr_rndr_set_state" => Ok(GeneratedField::SrvrRndrSetState),
                            "srvrRndrSetVolume" | "srvr_rndr_set_volume" => Ok(GeneratedField::SrvrRndrSetVolume),
                            "srvrRndrSetActive" | "srvr_rndr_set_active" => Ok(GeneratedField::SrvrRndrSetActive),
                            "srvrRndrSetMaxAudioQuality" | "srvr_rndr_set_max_audio_quality" => Ok(GeneratedField::SrvrRndrSetMaxAudioQuality),
                            "srvrRndrSetLoopMode" | "srvr_rndr_set_loop_mode" => Ok(GeneratedField::SrvrRndrSetLoopMode),
                            "srvrRndrSetShuffleMode" | "srvr_rndr_set_shuffle_mode" => Ok(GeneratedField::SrvrRndrSetShuffleMode),
                            "srvrRndrMuteVolume" | "srvr_rndr_mute_volume" => Ok(GeneratedField::SrvrRndrMuteVolume),
                            "ctrlSrvrJoinSession" | "ctrl_srvr_join_session" => Ok(GeneratedField::CtrlSrvrJoinSession),
                            "ctrlSrvrSetPlayerState" | "ctrl_srvr_set_player_state" => Ok(GeneratedField::CtrlSrvrSetPlayerState),
                            "ctrlSrvrSetActiveRenderer" | "ctrl_srvr_set_active_renderer" => Ok(GeneratedField::CtrlSrvrSetActiveRenderer),
                            "ctrlSrvrSetVolume" | "ctrl_srvr_set_volume" => Ok(GeneratedField::CtrlSrvrSetVolume),
                            "ctrlSrvrClearQueue" | "ctrl_srvr_clear_queue" => Ok(GeneratedField::CtrlSrvrClearQueue),
                            "ctrlSrvrQueueLoadTracks" | "ctrl_srvr_queue_load_tracks" => Ok(GeneratedField::CtrlSrvrQueueLoadTracks),
                            "ctrlSrvrQueueInsertTracks" | "ctrl_srvr_queue_insert_tracks" => Ok(GeneratedField::CtrlSrvrQueueInsertTracks),
                            "ctrlSrvrQueueAddTracks" | "ctrl_srvr_queue_add_tracks" => Ok(GeneratedField::CtrlSrvrQueueAddTracks),
                            "ctrlSrvrQueueRemoveTracks" | "ctrl_srvr_queue_remove_tracks" => Ok(GeneratedField::CtrlSrvrQueueRemoveTracks),
                            "ctrlSrvrQueueReorderTracks" | "ctrl_srvr_queue_reorder_tracks" => Ok(GeneratedField::CtrlSrvrQueueReorderTracks),
                            "ctrlSrvrSetShuffleMode" | "ctrl_srvr_set_shuffle_mode" => Ok(GeneratedField::CtrlSrvrSetShuffleMode),
                            "ctrlSrvrSetLoopMode" | "ctrl_srvr_set_loop_mode" => Ok(GeneratedField::CtrlSrvrSetLoopMode),
                            "ctrlSrvrMuteVolume" | "ctrl_srvr_mute_volume" => Ok(GeneratedField::CtrlSrvrMuteVolume),
                            "ctrlSrvrSetMaxAudioQuality" | "ctrl_srvr_set_max_audio_quality" => Ok(GeneratedField::CtrlSrvrSetMaxAudioQuality),
                            "ctrlSrvrSetQueueState" | "ctrl_srvr_set_queue_state" => Ok(GeneratedField::CtrlSrvrSetQueueState),
                            "ctrlSrvrAskForQueueState" | "ctrl_srvr_ask_for_queue_state" => Ok(GeneratedField::CtrlSrvrAskForQueueState),
                            "ctrlSrvrAskForRendererState" | "ctrl_srvr_ask_for_renderer_state" => Ok(GeneratedField::CtrlSrvrAskForRendererState),
                            "ctrlSrvrSetAutoplayMode" | "ctrl_srvr_set_autoplay_mode" => Ok(GeneratedField::CtrlSrvrSetAutoplayMode),
                            "ctrlSrvrAutoplayLoadTracks" | "ctrl_srvr_autoplay_load_tracks" => Ok(GeneratedField::CtrlSrvrAutoplayLoadTracks),
                            "ctrlSrvrAutoplayRemoveTracks" | "ctrl_srvr_autoplay_remove_tracks" => Ok(GeneratedField::CtrlSrvrAutoplayRemoveTracks),
                            "srvrCtrlSessionState" | "srvr_ctrl_session_state" => Ok(GeneratedField::SrvrCtrlSessionState),
                            "srvrCtrlRendererStateUpdated" | "srvr_ctrl_renderer_state_updated" => Ok(GeneratedField::SrvrCtrlRendererStateUpdated),
                            "srvrCtrlAddRenderer" | "srvr_ctrl_add_renderer" => Ok(GeneratedField::SrvrCtrlAddRenderer),
                            "srvrCtrlUpdateRenderer" | "srvr_ctrl_update_renderer" => Ok(GeneratedField::SrvrCtrlUpdateRenderer),
                            "srvrCtrlRemoveRenderer" | "srvr_ctrl_remove_renderer" => Ok(GeneratedField::SrvrCtrlRemoveRenderer),
                            "srvrCtrlActiveRendererChanged" | "srvr_ctrl_active_renderer_changed" => Ok(GeneratedField::SrvrCtrlActiveRendererChanged),
                            "srvrCtrlVolumeChanged" | "srvr_ctrl_volume_changed" => Ok(GeneratedField::SrvrCtrlVolumeChanged),
                            "srvrCtrlQueueErrorMessage" | "srvr_ctrl_queue_error_message" => Ok(GeneratedField::SrvrCtrlQueueErrorMessage),
                            "srvrCtrlQueueCleared" | "srvr_ctrl_queue_cleared" => Ok(GeneratedField::SrvrCtrlQueueCleared),
                            "srvrCtrlQueueState" | "srvr_ctrl_queue_state" => Ok(GeneratedField::SrvrCtrlQueueState),
                            "srvrCtrlQueueTracksLoaded" | "srvr_ctrl_queue_tracks_loaded" => Ok(GeneratedField::SrvrCtrlQueueTracksLoaded),
                            "srvrCtrlQueueTracksInserted" | "srvr_ctrl_queue_tracks_inserted" => Ok(GeneratedField::SrvrCtrlQueueTracksInserted),
                            "srvrCtrlQueueTracksAdded" | "srvr_ctrl_queue_tracks_added" => Ok(GeneratedField::SrvrCtrlQueueTracksAdded),
                            "srvrCtrlQueueTracksRemoved" | "srvr_ctrl_queue_tracks_removed" => Ok(GeneratedField::SrvrCtrlQueueTracksRemoved),
                            "srvrCtrlQueueTracksReordered" | "srvr_ctrl_queue_tracks_reordered" => Ok(GeneratedField::SrvrCtrlQueueTracksReordered),
                            "srvrCtrlShuffleModeSet" | "srvr_ctrl_shuffle_mode_set" => Ok(GeneratedField::SrvrCtrlShuffleModeSet),
                            "srvrCtrlLoopModeSet" | "srvr_ctrl_loop_mode_set" => Ok(GeneratedField::SrvrCtrlLoopModeSet),
                            "srvrCtrlVolumeMuted" | "srvr_ctrl_volume_muted" => Ok(GeneratedField::SrvrCtrlVolumeMuted),
                            "srvrCtrlMaxAudioQualityChanged" | "srvr_ctrl_max_audio_quality_changed" => Ok(GeneratedField::SrvrCtrlMaxAudioQualityChanged),
                            "srvrCtrlFileAudioQualityChanged" | "srvr_ctrl_file_audio_quality_changed" => Ok(GeneratedField::SrvrCtrlFileAudioQualityChanged),
                            "srvrCtrlDeviceAudioQualityChanged" | "srvr_ctrl_device_audio_quality_changed" => Ok(GeneratedField::SrvrCtrlDeviceAudioQualityChanged),
                            "srvrCtrlAutoplayModeSet" | "srvr_ctrl_autoplay_mode_set" => Ok(GeneratedField::SrvrCtrlAutoplayModeSet),
                            "srvrCtrlAutoplayTracksLoaded" | "srvr_ctrl_autoplay_tracks_loaded" => Ok(GeneratedField::SrvrCtrlAutoplayTracksLoaded),
                            "srvrCtrlAutoplayTracksRemoved" | "srvr_ctrl_autoplay_tracks_removed" => Ok(GeneratedField::SrvrCtrlAutoplayTracksRemoved),
                            "srvrCtrlQueueTracksAddedFromAutoplay" | "srvr_ctrl_queue_tracks_added_from_autoplay" => Ok(GeneratedField::SrvrCtrlQueueTracksAddedFromAutoplay),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QConnectMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QConnectMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QConnectMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_type__ = None;
                let mut error__ = None;
                let mut playback_error__ = None;
                let mut authenticate__ = None;
                let mut rndr_srvr_join_session__ = None;
                let mut rndr_srvr_device_info_updated__ = None;
                let mut rndr_srvr_state_updated__ = None;
                let mut rndr_srvr_renderer_action__ = None;
                let mut rndr_srvr_volume_changed__ = None;
                let mut rndr_srvr_file_audio_quality_changed__ = None;
                let mut rndr_srvr_device_audio_quality_changed__ = None;
                let mut rndr_srvr_max_audio_quality_changed__ = None;
                let mut rndr_srvr_volume_muted__ = None;
                let mut srvr_rndr_set_state__ = None;
                let mut srvr_rndr_set_volume__ = None;
                let mut srvr_rndr_set_active__ = None;
                let mut srvr_rndr_set_max_audio_quality__ = None;
                let mut srvr_rndr_set_loop_mode__ = None;
                let mut srvr_rndr_set_shuffle_mode__ = None;
                let mut srvr_rndr_mute_volume__ = None;
                let mut ctrl_srvr_join_session__ = None;
                let mut ctrl_srvr_set_player_state__ = None;
                let mut ctrl_srvr_set_active_renderer__ = None;
                let mut ctrl_srvr_set_volume__ = None;
                let mut ctrl_srvr_clear_queue__ = None;
                let mut ctrl_srvr_queue_load_tracks__ = None;
                let mut ctrl_srvr_queue_insert_tracks__ = None;
                let mut ctrl_srvr_queue_add_tracks__ = None;
                let mut ctrl_srvr_queue_remove_tracks__ = None;
                let mut ctrl_srvr_queue_reorder_tracks__ = None;
                let mut ctrl_srvr_set_shuffle_mode__ = None;
                let mut ctrl_srvr_set_loop_mode__ = None;
                let mut ctrl_srvr_mute_volume__ = None;
                let mut ctrl_srvr_set_max_audio_quality__ = None;
                let mut ctrl_srvr_set_queue_state__ = None;
                let mut ctrl_srvr_ask_for_queue_state__ = None;
                let mut ctrl_srvr_ask_for_renderer_state__ = None;
                let mut ctrl_srvr_set_autoplay_mode__ = None;
                let mut ctrl_srvr_autoplay_load_tracks__ = None;
                let mut ctrl_srvr_autoplay_remove_tracks__ = None;
                let mut srvr_ctrl_session_state__ = None;
                let mut srvr_ctrl_renderer_state_updated__ = None;
                let mut srvr_ctrl_add_renderer__ = None;
                let mut srvr_ctrl_update_renderer__ = None;
                let mut srvr_ctrl_remove_renderer__ = None;
                let mut srvr_ctrl_active_renderer_changed__ = None;
                let mut srvr_ctrl_volume_changed__ = None;
                let mut srvr_ctrl_queue_error_message__ = None;
                let mut srvr_ctrl_queue_cleared__ = None;
                let mut srvr_ctrl_queue_state__ = None;
                let mut srvr_ctrl_queue_tracks_loaded__ = None;
                let mut srvr_ctrl_queue_tracks_inserted__ = None;
                let mut srvr_ctrl_queue_tracks_added__ = None;
                let mut srvr_ctrl_queue_tracks_removed__ = None;
                let mut srvr_ctrl_queue_tracks_reordered__ = None;
                let mut srvr_ctrl_shuffle_mode_set__ = None;
                let mut srvr_ctrl_loop_mode_set__ = None;
                let mut srvr_ctrl_volume_muted__ = None;
                let mut srvr_ctrl_max_audio_quality_changed__ = None;
                let mut srvr_ctrl_file_audio_quality_changed__ = None;
                let mut srvr_ctrl_device_audio_quality_changed__ = None;
                let mut srvr_ctrl_autoplay_mode_set__ = None;
                let mut srvr_ctrl_autoplay_tracks_loaded__ = None;
                let mut srvr_ctrl_autoplay_tracks_removed__ = None;
                let mut srvr_ctrl_queue_tracks_added_from_autoplay__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessageType => {
                            if message_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageType"));
                            }
                            message_type__ = Some(map_.next_value::<MessageType>()? as i32);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::PlaybackError => {
                            if playback_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playbackError"));
                            }
                            playback_error__ = map_.next_value()?;
                        }
                        GeneratedField::Authenticate => {
                            if authenticate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authenticate"));
                            }
                            authenticate__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrJoinSession => {
                            if rndr_srvr_join_session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrJoinSession"));
                            }
                            rndr_srvr_join_session__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrDeviceInfoUpdated => {
                            if rndr_srvr_device_info_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrDeviceInfoUpdated"));
                            }
                            rndr_srvr_device_info_updated__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrStateUpdated => {
                            if rndr_srvr_state_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrStateUpdated"));
                            }
                            rndr_srvr_state_updated__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrRendererAction => {
                            if rndr_srvr_renderer_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrRendererAction"));
                            }
                            rndr_srvr_renderer_action__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrVolumeChanged => {
                            if rndr_srvr_volume_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrVolumeChanged"));
                            }
                            rndr_srvr_volume_changed__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrFileAudioQualityChanged => {
                            if rndr_srvr_file_audio_quality_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrFileAudioQualityChanged"));
                            }
                            rndr_srvr_file_audio_quality_changed__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrDeviceAudioQualityChanged => {
                            if rndr_srvr_device_audio_quality_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrDeviceAudioQualityChanged"));
                            }
                            rndr_srvr_device_audio_quality_changed__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrMaxAudioQualityChanged => {
                            if rndr_srvr_max_audio_quality_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrMaxAudioQualityChanged"));
                            }
                            rndr_srvr_max_audio_quality_changed__ = map_.next_value()?;
                        }
                        GeneratedField::RndrSrvrVolumeMuted => {
                            if rndr_srvr_volume_muted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rndrSrvrVolumeMuted"));
                            }
                            rndr_srvr_volume_muted__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrSetState => {
                            if srvr_rndr_set_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrSetState"));
                            }
                            srvr_rndr_set_state__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrSetVolume => {
                            if srvr_rndr_set_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrSetVolume"));
                            }
                            srvr_rndr_set_volume__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrSetActive => {
                            if srvr_rndr_set_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrSetActive"));
                            }
                            srvr_rndr_set_active__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrSetMaxAudioQuality => {
                            if srvr_rndr_set_max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrSetMaxAudioQuality"));
                            }
                            srvr_rndr_set_max_audio_quality__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrSetLoopMode => {
                            if srvr_rndr_set_loop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrSetLoopMode"));
                            }
                            srvr_rndr_set_loop_mode__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrSetShuffleMode => {
                            if srvr_rndr_set_shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrSetShuffleMode"));
                            }
                            srvr_rndr_set_shuffle_mode__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrRndrMuteVolume => {
                            if srvr_rndr_mute_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrRndrMuteVolume"));
                            }
                            srvr_rndr_mute_volume__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrJoinSession => {
                            if ctrl_srvr_join_session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrJoinSession"));
                            }
                            ctrl_srvr_join_session__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetPlayerState => {
                            if ctrl_srvr_set_player_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetPlayerState"));
                            }
                            ctrl_srvr_set_player_state__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetActiveRenderer => {
                            if ctrl_srvr_set_active_renderer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetActiveRenderer"));
                            }
                            ctrl_srvr_set_active_renderer__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetVolume => {
                            if ctrl_srvr_set_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetVolume"));
                            }
                            ctrl_srvr_set_volume__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrClearQueue => {
                            if ctrl_srvr_clear_queue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrClearQueue"));
                            }
                            ctrl_srvr_clear_queue__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrQueueLoadTracks => {
                            if ctrl_srvr_queue_load_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrQueueLoadTracks"));
                            }
                            ctrl_srvr_queue_load_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrQueueInsertTracks => {
                            if ctrl_srvr_queue_insert_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrQueueInsertTracks"));
                            }
                            ctrl_srvr_queue_insert_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrQueueAddTracks => {
                            if ctrl_srvr_queue_add_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrQueueAddTracks"));
                            }
                            ctrl_srvr_queue_add_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrQueueRemoveTracks => {
                            if ctrl_srvr_queue_remove_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrQueueRemoveTracks"));
                            }
                            ctrl_srvr_queue_remove_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrQueueReorderTracks => {
                            if ctrl_srvr_queue_reorder_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrQueueReorderTracks"));
                            }
                            ctrl_srvr_queue_reorder_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetShuffleMode => {
                            if ctrl_srvr_set_shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetShuffleMode"));
                            }
                            ctrl_srvr_set_shuffle_mode__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetLoopMode => {
                            if ctrl_srvr_set_loop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetLoopMode"));
                            }
                            ctrl_srvr_set_loop_mode__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrMuteVolume => {
                            if ctrl_srvr_mute_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrMuteVolume"));
                            }
                            ctrl_srvr_mute_volume__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetMaxAudioQuality => {
                            if ctrl_srvr_set_max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetMaxAudioQuality"));
                            }
                            ctrl_srvr_set_max_audio_quality__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetQueueState => {
                            if ctrl_srvr_set_queue_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetQueueState"));
                            }
                            ctrl_srvr_set_queue_state__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrAskForQueueState => {
                            if ctrl_srvr_ask_for_queue_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrAskForQueueState"));
                            }
                            ctrl_srvr_ask_for_queue_state__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrAskForRendererState => {
                            if ctrl_srvr_ask_for_renderer_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrAskForRendererState"));
                            }
                            ctrl_srvr_ask_for_renderer_state__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrSetAutoplayMode => {
                            if ctrl_srvr_set_autoplay_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrSetAutoplayMode"));
                            }
                            ctrl_srvr_set_autoplay_mode__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrAutoplayLoadTracks => {
                            if ctrl_srvr_autoplay_load_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrAutoplayLoadTracks"));
                            }
                            ctrl_srvr_autoplay_load_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::CtrlSrvrAutoplayRemoveTracks => {
                            if ctrl_srvr_autoplay_remove_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctrlSrvrAutoplayRemoveTracks"));
                            }
                            ctrl_srvr_autoplay_remove_tracks__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlSessionState => {
                            if srvr_ctrl_session_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlSessionState"));
                            }
                            srvr_ctrl_session_state__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlRendererStateUpdated => {
                            if srvr_ctrl_renderer_state_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlRendererStateUpdated"));
                            }
                            srvr_ctrl_renderer_state_updated__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlAddRenderer => {
                            if srvr_ctrl_add_renderer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlAddRenderer"));
                            }
                            srvr_ctrl_add_renderer__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlUpdateRenderer => {
                            if srvr_ctrl_update_renderer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlUpdateRenderer"));
                            }
                            srvr_ctrl_update_renderer__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlRemoveRenderer => {
                            if srvr_ctrl_remove_renderer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlRemoveRenderer"));
                            }
                            srvr_ctrl_remove_renderer__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlActiveRendererChanged => {
                            if srvr_ctrl_active_renderer_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlActiveRendererChanged"));
                            }
                            srvr_ctrl_active_renderer_changed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlVolumeChanged => {
                            if srvr_ctrl_volume_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlVolumeChanged"));
                            }
                            srvr_ctrl_volume_changed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueErrorMessage => {
                            if srvr_ctrl_queue_error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueErrorMessage"));
                            }
                            srvr_ctrl_queue_error_message__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueCleared => {
                            if srvr_ctrl_queue_cleared__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueCleared"));
                            }
                            srvr_ctrl_queue_cleared__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueState => {
                            if srvr_ctrl_queue_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueState"));
                            }
                            srvr_ctrl_queue_state__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueTracksLoaded => {
                            if srvr_ctrl_queue_tracks_loaded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueTracksLoaded"));
                            }
                            srvr_ctrl_queue_tracks_loaded__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueTracksInserted => {
                            if srvr_ctrl_queue_tracks_inserted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueTracksInserted"));
                            }
                            srvr_ctrl_queue_tracks_inserted__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueTracksAdded => {
                            if srvr_ctrl_queue_tracks_added__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueTracksAdded"));
                            }
                            srvr_ctrl_queue_tracks_added__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueTracksRemoved => {
                            if srvr_ctrl_queue_tracks_removed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueTracksRemoved"));
                            }
                            srvr_ctrl_queue_tracks_removed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueTracksReordered => {
                            if srvr_ctrl_queue_tracks_reordered__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueTracksReordered"));
                            }
                            srvr_ctrl_queue_tracks_reordered__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlShuffleModeSet => {
                            if srvr_ctrl_shuffle_mode_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlShuffleModeSet"));
                            }
                            srvr_ctrl_shuffle_mode_set__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlLoopModeSet => {
                            if srvr_ctrl_loop_mode_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlLoopModeSet"));
                            }
                            srvr_ctrl_loop_mode_set__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlVolumeMuted => {
                            if srvr_ctrl_volume_muted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlVolumeMuted"));
                            }
                            srvr_ctrl_volume_muted__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlMaxAudioQualityChanged => {
                            if srvr_ctrl_max_audio_quality_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlMaxAudioQualityChanged"));
                            }
                            srvr_ctrl_max_audio_quality_changed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlFileAudioQualityChanged => {
                            if srvr_ctrl_file_audio_quality_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlFileAudioQualityChanged"));
                            }
                            srvr_ctrl_file_audio_quality_changed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlDeviceAudioQualityChanged => {
                            if srvr_ctrl_device_audio_quality_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlDeviceAudioQualityChanged"));
                            }
                            srvr_ctrl_device_audio_quality_changed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlAutoplayModeSet => {
                            if srvr_ctrl_autoplay_mode_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlAutoplayModeSet"));
                            }
                            srvr_ctrl_autoplay_mode_set__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlAutoplayTracksLoaded => {
                            if srvr_ctrl_autoplay_tracks_loaded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlAutoplayTracksLoaded"));
                            }
                            srvr_ctrl_autoplay_tracks_loaded__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlAutoplayTracksRemoved => {
                            if srvr_ctrl_autoplay_tracks_removed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlAutoplayTracksRemoved"));
                            }
                            srvr_ctrl_autoplay_tracks_removed__ = map_.next_value()?;
                        }
                        GeneratedField::SrvrCtrlQueueTracksAddedFromAutoplay => {
                            if srvr_ctrl_queue_tracks_added_from_autoplay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("srvrCtrlQueueTracksAddedFromAutoplay"));
                            }
                            srvr_ctrl_queue_tracks_added_from_autoplay__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QConnectMessage {
                    message_type: message_type__.unwrap_or_default(),
                    error: error__,
                    playback_error: playback_error__,
                    authenticate: authenticate__,
                    rndr_srvr_join_session: rndr_srvr_join_session__,
                    rndr_srvr_device_info_updated: rndr_srvr_device_info_updated__,
                    rndr_srvr_state_updated: rndr_srvr_state_updated__,
                    rndr_srvr_renderer_action: rndr_srvr_renderer_action__,
                    rndr_srvr_volume_changed: rndr_srvr_volume_changed__,
                    rndr_srvr_file_audio_quality_changed: rndr_srvr_file_audio_quality_changed__,
                    rndr_srvr_device_audio_quality_changed: rndr_srvr_device_audio_quality_changed__,
                    rndr_srvr_max_audio_quality_changed: rndr_srvr_max_audio_quality_changed__,
                    rndr_srvr_volume_muted: rndr_srvr_volume_muted__,
                    srvr_rndr_set_state: srvr_rndr_set_state__,
                    srvr_rndr_set_volume: srvr_rndr_set_volume__,
                    srvr_rndr_set_active: srvr_rndr_set_active__,
                    srvr_rndr_set_max_audio_quality: srvr_rndr_set_max_audio_quality__,
                    srvr_rndr_set_loop_mode: srvr_rndr_set_loop_mode__,
                    srvr_rndr_set_shuffle_mode: srvr_rndr_set_shuffle_mode__,
                    srvr_rndr_mute_volume: srvr_rndr_mute_volume__,
                    ctrl_srvr_join_session: ctrl_srvr_join_session__,
                    ctrl_srvr_set_player_state: ctrl_srvr_set_player_state__,
                    ctrl_srvr_set_active_renderer: ctrl_srvr_set_active_renderer__,
                    ctrl_srvr_set_volume: ctrl_srvr_set_volume__,
                    ctrl_srvr_clear_queue: ctrl_srvr_clear_queue__,
                    ctrl_srvr_queue_load_tracks: ctrl_srvr_queue_load_tracks__,
                    ctrl_srvr_queue_insert_tracks: ctrl_srvr_queue_insert_tracks__,
                    ctrl_srvr_queue_add_tracks: ctrl_srvr_queue_add_tracks__,
                    ctrl_srvr_queue_remove_tracks: ctrl_srvr_queue_remove_tracks__,
                    ctrl_srvr_queue_reorder_tracks: ctrl_srvr_queue_reorder_tracks__,
                    ctrl_srvr_set_shuffle_mode: ctrl_srvr_set_shuffle_mode__,
                    ctrl_srvr_set_loop_mode: ctrl_srvr_set_loop_mode__,
                    ctrl_srvr_mute_volume: ctrl_srvr_mute_volume__,
                    ctrl_srvr_set_max_audio_quality: ctrl_srvr_set_max_audio_quality__,
                    ctrl_srvr_set_queue_state: ctrl_srvr_set_queue_state__,
                    ctrl_srvr_ask_for_queue_state: ctrl_srvr_ask_for_queue_state__,
                    ctrl_srvr_ask_for_renderer_state: ctrl_srvr_ask_for_renderer_state__,
                    ctrl_srvr_set_autoplay_mode: ctrl_srvr_set_autoplay_mode__,
                    ctrl_srvr_autoplay_load_tracks: ctrl_srvr_autoplay_load_tracks__,
                    ctrl_srvr_autoplay_remove_tracks: ctrl_srvr_autoplay_remove_tracks__,
                    srvr_ctrl_session_state: srvr_ctrl_session_state__,
                    srvr_ctrl_renderer_state_updated: srvr_ctrl_renderer_state_updated__,
                    srvr_ctrl_add_renderer: srvr_ctrl_add_renderer__,
                    srvr_ctrl_update_renderer: srvr_ctrl_update_renderer__,
                    srvr_ctrl_remove_renderer: srvr_ctrl_remove_renderer__,
                    srvr_ctrl_active_renderer_changed: srvr_ctrl_active_renderer_changed__,
                    srvr_ctrl_volume_changed: srvr_ctrl_volume_changed__,
                    srvr_ctrl_queue_error_message: srvr_ctrl_queue_error_message__,
                    srvr_ctrl_queue_cleared: srvr_ctrl_queue_cleared__,
                    srvr_ctrl_queue_state: srvr_ctrl_queue_state__,
                    srvr_ctrl_queue_tracks_loaded: srvr_ctrl_queue_tracks_loaded__,
                    srvr_ctrl_queue_tracks_inserted: srvr_ctrl_queue_tracks_inserted__,
                    srvr_ctrl_queue_tracks_added: srvr_ctrl_queue_tracks_added__,
                    srvr_ctrl_queue_tracks_removed: srvr_ctrl_queue_tracks_removed__,
                    srvr_ctrl_queue_tracks_reordered: srvr_ctrl_queue_tracks_reordered__,
                    srvr_ctrl_shuffle_mode_set: srvr_ctrl_shuffle_mode_set__,
                    srvr_ctrl_loop_mode_set: srvr_ctrl_loop_mode_set__,
                    srvr_ctrl_volume_muted: srvr_ctrl_volume_muted__,
                    srvr_ctrl_max_audio_quality_changed: srvr_ctrl_max_audio_quality_changed__,
                    srvr_ctrl_file_audio_quality_changed: srvr_ctrl_file_audio_quality_changed__,
                    srvr_ctrl_device_audio_quality_changed: srvr_ctrl_device_audio_quality_changed__,
                    srvr_ctrl_autoplay_mode_set: srvr_ctrl_autoplay_mode_set__,
                    srvr_ctrl_autoplay_tracks_loaded: srvr_ctrl_autoplay_tracks_loaded__,
                    srvr_ctrl_autoplay_tracks_removed: srvr_ctrl_autoplay_tracks_removed__,
                    srvr_ctrl_queue_tracks_added_from_autoplay: srvr_ctrl_queue_tracks_added_from_autoplay__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QConnectMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueueItemRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QueueItemRef", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueueItemRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueueItemRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QueueItemRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueueItemRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueueItemRef {
                    queue_version: queue_version__,
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QueueItemRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueueRendererState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.playing_state != 0 {
            len += 1;
        }
        if self.buffer_state != 0 {
            len += 1;
        }
        if self.current_position.is_some() {
            len += 1;
        }
        if self.duration != 0 {
            len += 1;
        }
        if self.queue_version.is_some() {
            len += 1;
        }
        if self.current_queue_item_id != 0 {
            len += 1;
        }
        if self.next_queue_item_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QueueRendererState", len)?;
        if self.playing_state != 0 {
            let v = PlayingState::try_from(self.playing_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.playing_state)))?;
            struct_ser.serialize_field("playingState", &v)?;
        }
        if self.buffer_state != 0 {
            let v = BufferState::try_from(self.buffer_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.buffer_state)))?;
            struct_ser.serialize_field("bufferState", &v)?;
        }
        if let Some(v) = self.current_position.as_ref() {
            struct_ser.serialize_field("currentPosition", v)?;
        }
        if self.duration != 0 {
            struct_ser.serialize_field("duration", &self.duration)?;
        }
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if self.current_queue_item_id != 0 {
            struct_ser.serialize_field("currentQueueItemId", &self.current_queue_item_id)?;
        }
        if self.next_queue_item_id != 0 {
            struct_ser.serialize_field("nextQueueItemId", &self.next_queue_item_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueueRendererState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "playing_state",
            "playingState",
            "buffer_state",
            "bufferState",
            "current_position",
            "currentPosition",
            "duration",
            "queue_version",
            "queueVersion",
            "current_queue_item_id",
            "currentQueueItemId",
            "next_queue_item_id",
            "nextQueueItemId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayingState,
            BufferState,
            CurrentPosition,
            Duration,
            QueueVersion,
            CurrentQueueItemId,
            NextQueueItemId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playingState" | "playing_state" => Ok(GeneratedField::PlayingState),
                            "bufferState" | "buffer_state" => Ok(GeneratedField::BufferState),
                            "currentPosition" | "current_position" => Ok(GeneratedField::CurrentPosition),
                            "duration" => Ok(GeneratedField::Duration),
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "currentQueueItemId" | "current_queue_item_id" => Ok(GeneratedField::CurrentQueueItemId),
                            "nextQueueItemId" | "next_queue_item_id" => Ok(GeneratedField::NextQueueItemId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueueRendererState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QueueRendererState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueueRendererState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut playing_state__ = None;
                let mut buffer_state__ = None;
                let mut current_position__ = None;
                let mut duration__ = None;
                let mut queue_version__ = None;
                let mut current_queue_item_id__ = None;
                let mut next_queue_item_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayingState => {
                            if playing_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playingState"));
                            }
                            playing_state__ = Some(map_.next_value::<PlayingState>()? as i32);
                        }
                        GeneratedField::BufferState => {
                            if buffer_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferState"));
                            }
                            buffer_state__ = Some(map_.next_value::<BufferState>()? as i32);
                        }
                        GeneratedField::CurrentPosition => {
                            if current_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPosition"));
                            }
                            current_position__ = map_.next_value()?;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentQueueItemId => {
                            if current_queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentQueueItemId"));
                            }
                            current_queue_item_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextQueueItemId => {
                            if next_queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextQueueItemId"));
                            }
                            next_queue_item_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueueRendererState {
                    playing_state: playing_state__.unwrap_or_default(),
                    buffer_state: buffer_state__.unwrap_or_default(),
                    current_position: current_position__,
                    duration: duration__.unwrap_or_default(),
                    queue_version: queue_version__,
                    current_queue_item_id: current_queue_item_id__.unwrap_or_default(),
                    next_queue_item_id: next_queue_item_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QueueRendererState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueueTrack {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_item_id != 0 {
            len += 1;
        }
        if self.track_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QueueTrack", len)?;
        if self.queue_item_id != 0 {
            struct_ser.serialize_field("queueItemId", &self.queue_item_id)?;
        }
        if self.track_id != 0 {
            struct_ser.serialize_field("trackId", &self.track_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueueTrack {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_item_id",
            "queueItemId",
            "track_id",
            "trackId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueItemId,
            TrackId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueItemId" | "queue_item_id" => Ok(GeneratedField::QueueItemId),
                            "trackId" | "track_id" => Ok(GeneratedField::TrackId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueueTrack;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QueueTrack")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueueTrack, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_item_id__ = None;
                let mut track_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueItemId => {
                            if queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemId"));
                            }
                            queue_item_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrackId => {
                            if track_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackId"));
                            }
                            track_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueueTrack {
                    queue_item_id: queue_item_id__.unwrap_or_default(),
                    track_id: track_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QueueTrack", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueueTrackRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_item_id != 0 {
            len += 1;
        }
        if self.track_id != 0 {
            len += 1;
        }
        if self.context_uuid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QueueTrackRef", len)?;
        if self.queue_item_id != 0 {
            struct_ser.serialize_field("queueItemId", &self.queue_item_id)?;
        }
        if self.track_id != 0 {
            struct_ser.serialize_field("trackId", &self.track_id)?;
        }
        if let Some(v) = self.context_uuid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueueTrackRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_item_id",
            "queueItemId",
            "track_id",
            "trackId",
            "context_uuid",
            "contextUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueItemId,
            TrackId,
            ContextUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueItemId" | "queue_item_id" => Ok(GeneratedField::QueueItemId),
                            "trackId" | "track_id" => Ok(GeneratedField::TrackId),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueueTrackRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QueueTrackRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueueTrackRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_item_id__ = None;
                let mut track_id__ = None;
                let mut context_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueItemId => {
                            if queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemId"));
                            }
                            queue_item_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrackId => {
                            if track_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackId"));
                            }
                            track_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(QueueTrackRef {
                    queue_item_id: queue_item_id__.unwrap_or_default(),
                    track_id: track_id__.unwrap_or_default(),
                    context_uuid: context_uuid__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QueueTrackRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueueVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major != 0 {
            len += 1;
        }
        if self.minor != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.QueueVersion", len)?;
        if self.major != 0 {
            struct_ser.serialize_field("major", &self.major)?;
        }
        if self.minor != 0 {
            struct_ser.serialize_field("minor", &self.minor)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueueVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major",
            "minor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Major,
            Minor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "major" => Ok(GeneratedField::Major),
                            "minor" => Ok(GeneratedField::Minor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueueVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.QueueVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueueVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major__ = None;
                let mut minor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Major => {
                            if major__.is_some() {
                                return Err(serde::de::Error::duplicate_field("major"));
                            }
                            major__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Minor => {
                            if minor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minor"));
                            }
                            minor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueueVersion {
                    major: major__.unwrap_or_default(),
                    minor: minor__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.QueueVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RendererState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.playing_state != 0 {
            len += 1;
        }
        if self.buffer_state != 0 {
            len += 1;
        }
        if self.current_position.is_some() {
            len += 1;
        }
        if self.duration != 0 {
            len += 1;
        }
        if self.current_queue_item_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RendererState", len)?;
        if self.playing_state != 0 {
            let v = PlayingState::try_from(self.playing_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.playing_state)))?;
            struct_ser.serialize_field("playingState", &v)?;
        }
        if self.buffer_state != 0 {
            let v = BufferState::try_from(self.buffer_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.buffer_state)))?;
            struct_ser.serialize_field("bufferState", &v)?;
        }
        if let Some(v) = self.current_position.as_ref() {
            struct_ser.serialize_field("currentPosition", v)?;
        }
        if self.duration != 0 {
            struct_ser.serialize_field("duration", &self.duration)?;
        }
        if self.current_queue_item_id != 0 {
            struct_ser.serialize_field("currentQueueItemId", &self.current_queue_item_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RendererState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "playing_state",
            "playingState",
            "buffer_state",
            "bufferState",
            "current_position",
            "currentPosition",
            "duration",
            "current_queue_item_id",
            "currentQueueItemId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayingState,
            BufferState,
            CurrentPosition,
            Duration,
            CurrentQueueItemId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playingState" | "playing_state" => Ok(GeneratedField::PlayingState),
                            "bufferState" | "buffer_state" => Ok(GeneratedField::BufferState),
                            "currentPosition" | "current_position" => Ok(GeneratedField::CurrentPosition),
                            "duration" => Ok(GeneratedField::Duration),
                            "currentQueueItemId" | "current_queue_item_id" => Ok(GeneratedField::CurrentQueueItemId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RendererState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RendererState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RendererState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut playing_state__ = None;
                let mut buffer_state__ = None;
                let mut current_position__ = None;
                let mut duration__ = None;
                let mut current_queue_item_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayingState => {
                            if playing_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playingState"));
                            }
                            playing_state__ = Some(map_.next_value::<PlayingState>()? as i32);
                        }
                        GeneratedField::BufferState => {
                            if buffer_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferState"));
                            }
                            buffer_state__ = Some(map_.next_value::<BufferState>()? as i32);
                        }
                        GeneratedField::CurrentPosition => {
                            if current_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPosition"));
                            }
                            current_position__ = map_.next_value()?;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrentQueueItemId => {
                            if current_queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentQueueItemId"));
                            }
                            current_queue_item_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RendererState {
                    playing_state: playing_state__.unwrap_or_default(),
                    buffer_state: buffer_state__.unwrap_or_default(),
                    current_position: current_position__,
                    duration: duration__.unwrap_or_default(),
                    current_queue_item_id: current_queue_item_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RendererState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RendererStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "RENDERER_STATUS_UNKNOWN",
            Self::ActiveConnected => "RENDERER_STATUS_ACTIVE_CONNECTED",
            Self::ActiveDisconnected => "RENDERER_STATUS_ACTIVE_DISCONNECTED",
            Self::Inactive => "RENDERER_STATUS_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RendererStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RENDERER_STATUS_UNKNOWN",
            "RENDERER_STATUS_ACTIVE_CONNECTED",
            "RENDERER_STATUS_ACTIVE_DISCONNECTED",
            "RENDERER_STATUS_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = RendererStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RENDERER_STATUS_UNKNOWN" => Ok(RendererStatus::Unknown),
                    "RENDERER_STATUS_ACTIVE_CONNECTED" => Ok(RendererStatus::ActiveConnected),
                    "RENDERER_STATUS_ACTIVE_DISCONNECTED" => Ok(RendererStatus::ActiveDisconnected),
                    "RENDERER_STATUS_INACTIVE" => Ok(RendererStatus::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrDeviceAudioQualityChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sampling_rate != 0 {
            len += 1;
        }
        if self.bit_depth != 0 {
            len += 1;
        }
        if self.nb_channels != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrDeviceAudioQualityChanged", len)?;
        if self.sampling_rate != 0 {
            struct_ser.serialize_field("samplingRate", &self.sampling_rate)?;
        }
        if self.bit_depth != 0 {
            struct_ser.serialize_field("bitDepth", &self.bit_depth)?;
        }
        if self.nb_channels != 0 {
            struct_ser.serialize_field("nbChannels", &self.nb_channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrDeviceAudioQualityChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sampling_rate",
            "samplingRate",
            "bit_depth",
            "bitDepth",
            "nb_channels",
            "nbChannels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SamplingRate,
            BitDepth,
            NbChannels,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "samplingRate" | "sampling_rate" => Ok(GeneratedField::SamplingRate),
                            "bitDepth" | "bit_depth" => Ok(GeneratedField::BitDepth),
                            "nbChannels" | "nb_channels" => Ok(GeneratedField::NbChannels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrDeviceAudioQualityChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrDeviceAudioQualityChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrDeviceAudioQualityChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sampling_rate__ = None;
                let mut bit_depth__ = None;
                let mut nb_channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SamplingRate => {
                            if sampling_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRate"));
                            }
                            sampling_rate__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BitDepth => {
                            if bit_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitDepth"));
                            }
                            bit_depth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NbChannels => {
                            if nb_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nbChannels"));
                            }
                            nb_channels__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RndrSrvrDeviceAudioQualityChanged {
                    sampling_rate: sampling_rate__.unwrap_or_default(),
                    bit_depth: bit_depth__.unwrap_or_default(),
                    nb_channels: nb_channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrDeviceAudioQualityChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrDeviceInfoUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrDeviceInfoUpdated", len)?;
        if let Some(v) = self.device_info.as_ref() {
            struct_ser.serialize_field("deviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrDeviceInfoUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_info",
            "deviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deviceInfo" | "device_info" => Ok(GeneratedField::DeviceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrDeviceInfoUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrDeviceInfoUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrDeviceInfoUpdated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceInfo => {
                            if device_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceInfo"));
                            }
                            device_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RndrSrvrDeviceInfoUpdated {
                    device_info: device_info__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrDeviceInfoUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrFileAudioQualityChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sampling_rate != 0 {
            len += 1;
        }
        if self.bit_depth != 0 {
            len += 1;
        }
        if self.nb_channels != 0 {
            len += 1;
        }
        if self.audio_quality != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrFileAudioQualityChanged", len)?;
        if self.sampling_rate != 0 {
            struct_ser.serialize_field("samplingRate", &self.sampling_rate)?;
        }
        if self.bit_depth != 0 {
            struct_ser.serialize_field("bitDepth", &self.bit_depth)?;
        }
        if self.nb_channels != 0 {
            struct_ser.serialize_field("nbChannels", &self.nb_channels)?;
        }
        if self.audio_quality != 0 {
            let v = AudioQuality::try_from(self.audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.audio_quality)))?;
            struct_ser.serialize_field("audioQuality", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrFileAudioQualityChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sampling_rate",
            "samplingRate",
            "bit_depth",
            "bitDepth",
            "nb_channels",
            "nbChannels",
            "audio_quality",
            "audioQuality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SamplingRate,
            BitDepth,
            NbChannels,
            AudioQuality,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "samplingRate" | "sampling_rate" => Ok(GeneratedField::SamplingRate),
                            "bitDepth" | "bit_depth" => Ok(GeneratedField::BitDepth),
                            "nbChannels" | "nb_channels" => Ok(GeneratedField::NbChannels),
                            "audioQuality" | "audio_quality" => Ok(GeneratedField::AudioQuality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrFileAudioQualityChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrFileAudioQualityChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrFileAudioQualityChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sampling_rate__ = None;
                let mut bit_depth__ = None;
                let mut nb_channels__ = None;
                let mut audio_quality__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SamplingRate => {
                            if sampling_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRate"));
                            }
                            sampling_rate__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BitDepth => {
                            if bit_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitDepth"));
                            }
                            bit_depth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NbChannels => {
                            if nb_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nbChannels"));
                            }
                            nb_channels__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AudioQuality => {
                            if audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audioQuality"));
                            }
                            audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                    }
                }
                Ok(RndrSrvrFileAudioQualityChanged {
                    sampling_rate: sampling_rate__.unwrap_or_default(),
                    bit_depth: bit_depth__.unwrap_or_default(),
                    nb_channels: nb_channels__.unwrap_or_default(),
                    audio_quality: audio_quality__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrFileAudioQualityChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrJoinSession {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_uuid.is_empty() {
            len += 1;
        }
        if self.device_info.is_some() {
            len += 1;
        }
        if self.reason != 0 {
            len += 1;
        }
        if self.initial_state.is_some() {
            len += 1;
        }
        if self.is_active.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrJoinSession", len)?;
        if !self.session_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sessionUuid", pbjson::private::base64::encode(&self.session_uuid).as_str())?;
        }
        if let Some(v) = self.device_info.as_ref() {
            struct_ser.serialize_field("deviceInfo", v)?;
        }
        if self.reason != 0 {
            let v = JoinSessionReason::try_from(self.reason)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        if let Some(v) = self.initial_state.as_ref() {
            struct_ser.serialize_field("initialState", v)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrJoinSession {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_uuid",
            "sessionUuid",
            "device_info",
            "deviceInfo",
            "reason",
            "initial_state",
            "initialState",
            "is_active",
            "isActive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionUuid,
            DeviceInfo,
            Reason,
            InitialState,
            IsActive,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sessionUuid" | "session_uuid" => Ok(GeneratedField::SessionUuid),
                            "deviceInfo" | "device_info" => Ok(GeneratedField::DeviceInfo),
                            "reason" => Ok(GeneratedField::Reason),
                            "initialState" | "initial_state" => Ok(GeneratedField::InitialState),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrJoinSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrJoinSession")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrJoinSession, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_uuid__ = None;
                let mut device_info__ = None;
                let mut reason__ = None;
                let mut initial_state__ = None;
                let mut is_active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionUuid => {
                            if session_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionUuid"));
                            }
                            session_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceInfo => {
                            if device_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceInfo"));
                            }
                            device_info__ = map_.next_value()?;
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value::<JoinSessionReason>()? as i32);
                        }
                        GeneratedField::InitialState => {
                            if initial_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialState"));
                            }
                            initial_state__ = map_.next_value()?;
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RndrSrvrJoinSession {
                    session_uuid: session_uuid__.unwrap_or_default(),
                    device_info: device_info__,
                    reason: reason__.unwrap_or_default(),
                    initial_state: initial_state__,
                    is_active: is_active__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrJoinSession", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrMaxAudioQualityChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_audio_quality != 0 {
            len += 1;
        }
        if self.network_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrMaxAudioQualityChanged", len)?;
        if self.max_audio_quality != 0 {
            let v = AudioQuality::try_from(self.max_audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_audio_quality)))?;
            struct_ser.serialize_field("maxAudioQuality", &v)?;
        }
        if let Some(v) = self.network_type.as_ref() {
            let v = NetworkType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("networkType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrMaxAudioQualityChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_audio_quality",
            "maxAudioQuality",
            "network_type",
            "networkType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxAudioQuality,
            NetworkType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxAudioQuality" | "max_audio_quality" => Ok(GeneratedField::MaxAudioQuality),
                            "networkType" | "network_type" => Ok(GeneratedField::NetworkType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrMaxAudioQualityChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrMaxAudioQualityChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrMaxAudioQualityChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_audio_quality__ = None;
                let mut network_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxAudioQuality => {
                            if max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAudioQuality"));
                            }
                            max_audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                        GeneratedField::NetworkType => {
                            if network_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkType"));
                            }
                            network_type__ = map_.next_value::<::std::option::Option<NetworkType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(RndrSrvrMaxAudioQualityChanged {
                    max_audio_quality: max_audio_quality__.unwrap_or_default(),
                    network_type: network_type__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrMaxAudioQualityChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrRendererAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seek_position.is_some() {
            len += 1;
        }
        if self.action != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrRendererAction", len)?;
        if let Some(v) = self.seek_position.as_ref() {
            struct_ser.serialize_field("seekPosition", v)?;
        }
        if self.action != 0 {
            let v = ActionType::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrRendererAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seek_position",
            "seekPosition",
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SeekPosition,
            Action,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seekPosition" | "seek_position" => Ok(GeneratedField::SeekPosition),
                            "action" => Ok(GeneratedField::Action),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrRendererAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrRendererAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrRendererAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seek_position__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SeekPosition => {
                            if seek_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seekPosition"));
                            }
                            seek_position__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<ActionType>()? as i32);
                        }
                    }
                }
                Ok(RndrSrvrRendererAction {
                    seek_position: seek_position__,
                    action: action__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrRendererAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrStateUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrStateUpdated", len)?;
        if let Some(v) = self.state.as_ref() {
            struct_ser.serialize_field("state", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrStateUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrStateUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrStateUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrStateUpdated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RndrSrvrStateUpdated {
                    state: state__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrStateUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrVolumeChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.volume != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrVolumeChanged", len)?;
        if self.volume != 0 {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrVolumeChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Volume,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrVolumeChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrVolumeChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrVolumeChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut volume__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RndrSrvrVolumeChanged {
                    volume: volume__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrVolumeChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RndrSrvrVolumeMuted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.RndrSrvrVolumeMuted", len)?;
        if self.value {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RndrSrvrVolumeMuted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RndrSrvrVolumeMuted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.RndrSrvrVolumeMuted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RndrSrvrVolumeMuted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RndrSrvrVolumeMuted {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.RndrSrvrVolumeMuted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlActiveRendererChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.active_renderer_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlActiveRendererChanged", len)?;
        if self.active_renderer_id != 0 {
            struct_ser.serialize_field("activeRendererId", &self.active_renderer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlActiveRendererChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active_renderer_id",
            "activeRendererId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActiveRendererId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "activeRendererId" | "active_renderer_id" => Ok(GeneratedField::ActiveRendererId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlActiveRendererChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlActiveRendererChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlActiveRendererChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_renderer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActiveRendererId => {
                            if active_renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeRendererId"));
                            }
                            active_renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlActiveRendererChanged {
                    active_renderer_id: active_renderer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlActiveRendererChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlAddRenderer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.device_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlAddRenderer", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if let Some(v) = self.device_info.as_ref() {
            struct_ser.serialize_field("deviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlAddRenderer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "device_info",
            "deviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            DeviceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "deviceInfo" | "device_info" => Ok(GeneratedField::DeviceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlAddRenderer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlAddRenderer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlAddRenderer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut device_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceInfo => {
                            if device_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceInfo"));
                            }
                            device_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SrvrCtrlAddRenderer {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    device_info: device_info__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlAddRenderer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlAutoplayModeSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_mode {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlAutoplayModeSet", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if self.autoplay_mode {
            struct_ser.serialize_field("autoplayMode", &self.autoplay_mode)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlAutoplayModeSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "autoplay_mode",
            "autoplayMode",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            AutoplayMode,
            AutoplayReset,
            AutoplayLoading,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "autoplayMode" | "autoplay_mode" => Ok(GeneratedField::AutoplayMode),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlAutoplayModeSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlAutoplayModeSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlAutoplayModeSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut autoplay_mode__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayMode => {
                            if autoplay_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayMode"));
                            }
                            autoplay_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SrvrCtrlAutoplayModeSet {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    autoplay_mode: autoplay_mode__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlAutoplayModeSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlAutoplayTracksLoaded {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.tracks.is_empty() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlAutoplayTracksLoaded", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.tracks.is_empty() {
            struct_ser.serialize_field("tracks", &self.tracks)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlAutoplayTracksLoaded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "tracks",
            "context_uuid",
            "contextUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            Tracks,
            ContextUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "tracks" => Ok(GeneratedField::Tracks),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlAutoplayTracksLoaded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlAutoplayTracksLoaded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlAutoplayTracksLoaded, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut tracks__ = None;
                let mut context_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tracks => {
                            if tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracks"));
                            }
                            tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlAutoplayTracksLoaded {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    tracks: tracks__.unwrap_or_default(),
                    context_uuid: context_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlAutoplayTracksLoaded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlAutoplayTracksRemoved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlAutoplayTracksRemoved", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlAutoplayTracksRemoved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "queue_item_ids",
            "queueItemIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            QueueItemIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlAutoplayTracksRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlAutoplayTracksRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlAutoplayTracksRemoved, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut queue_item_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlAutoplayTracksRemoved {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlAutoplayTracksRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlDeviceAudioQualityChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.sampling_rate != 0 {
            len += 1;
        }
        if self.bit_depth != 0 {
            len += 1;
        }
        if self.nb_channels != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlDeviceAudioQualityChanged", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.sampling_rate != 0 {
            struct_ser.serialize_field("samplingRate", &self.sampling_rate)?;
        }
        if self.bit_depth != 0 {
            struct_ser.serialize_field("bitDepth", &self.bit_depth)?;
        }
        if self.nb_channels != 0 {
            struct_ser.serialize_field("nbChannels", &self.nb_channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlDeviceAudioQualityChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "sampling_rate",
            "samplingRate",
            "bit_depth",
            "bitDepth",
            "nb_channels",
            "nbChannels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            SamplingRate,
            BitDepth,
            NbChannels,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "samplingRate" | "sampling_rate" => Ok(GeneratedField::SamplingRate),
                            "bitDepth" | "bit_depth" => Ok(GeneratedField::BitDepth),
                            "nbChannels" | "nb_channels" => Ok(GeneratedField::NbChannels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlDeviceAudioQualityChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlDeviceAudioQualityChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlDeviceAudioQualityChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut sampling_rate__ = None;
                let mut bit_depth__ = None;
                let mut nb_channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SamplingRate => {
                            if sampling_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRate"));
                            }
                            sampling_rate__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BitDepth => {
                            if bit_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitDepth"));
                            }
                            bit_depth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NbChannels => {
                            if nb_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nbChannels"));
                            }
                            nb_channels__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlDeviceAudioQualityChanged {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    sampling_rate: sampling_rate__.unwrap_or_default(),
                    bit_depth: bit_depth__.unwrap_or_default(),
                    nb_channels: nb_channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlDeviceAudioQualityChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlFileAudioQualityChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.sampling_rate != 0 {
            len += 1;
        }
        if self.bit_depth != 0 {
            len += 1;
        }
        if self.nb_channels != 0 {
            len += 1;
        }
        if self.audio_quality != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlFileAudioQualityChanged", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.sampling_rate != 0 {
            struct_ser.serialize_field("samplingRate", &self.sampling_rate)?;
        }
        if self.bit_depth != 0 {
            struct_ser.serialize_field("bitDepth", &self.bit_depth)?;
        }
        if self.nb_channels != 0 {
            struct_ser.serialize_field("nbChannels", &self.nb_channels)?;
        }
        if self.audio_quality != 0 {
            let v = AudioQuality::try_from(self.audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.audio_quality)))?;
            struct_ser.serialize_field("audioQuality", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlFileAudioQualityChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "sampling_rate",
            "samplingRate",
            "bit_depth",
            "bitDepth",
            "nb_channels",
            "nbChannels",
            "audio_quality",
            "audioQuality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            SamplingRate,
            BitDepth,
            NbChannels,
            AudioQuality,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "samplingRate" | "sampling_rate" => Ok(GeneratedField::SamplingRate),
                            "bitDepth" | "bit_depth" => Ok(GeneratedField::BitDepth),
                            "nbChannels" | "nb_channels" => Ok(GeneratedField::NbChannels),
                            "audioQuality" | "audio_quality" => Ok(GeneratedField::AudioQuality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlFileAudioQualityChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlFileAudioQualityChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlFileAudioQualityChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut sampling_rate__ = None;
                let mut bit_depth__ = None;
                let mut nb_channels__ = None;
                let mut audio_quality__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SamplingRate => {
                            if sampling_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRate"));
                            }
                            sampling_rate__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BitDepth => {
                            if bit_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitDepth"));
                            }
                            bit_depth__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NbChannels => {
                            if nb_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nbChannels"));
                            }
                            nb_channels__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AudioQuality => {
                            if audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audioQuality"));
                            }
                            audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                    }
                }
                Ok(SrvrCtrlFileAudioQualityChanged {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    sampling_rate: sampling_rate__.unwrap_or_default(),
                    bit_depth: bit_depth__.unwrap_or_default(),
                    nb_channels: nb_channels__.unwrap_or_default(),
                    audio_quality: audio_quality__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlFileAudioQualityChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlLoopModeSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.loop_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlLoopModeSet", len)?;
        if self.loop_mode != 0 {
            let v = LoopMode::try_from(self.loop_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.loop_mode)))?;
            struct_ser.serialize_field("loopMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlLoopModeSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "loop_mode",
            "loopMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoopMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "loopMode" | "loop_mode" => Ok(GeneratedField::LoopMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlLoopModeSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlLoopModeSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlLoopModeSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut loop_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoopMode => {
                            if loop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopMode"));
                            }
                            loop_mode__ = Some(map_.next_value::<LoopMode>()? as i32);
                        }
                    }
                }
                Ok(SrvrCtrlLoopModeSet {
                    loop_mode: loop_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlLoopModeSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlMaxAudioQualityChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.max_audio_quality != 0 {
            len += 1;
        }
        if self.network_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlMaxAudioQualityChanged", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.max_audio_quality != 0 {
            let v = AudioQuality::try_from(self.max_audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_audio_quality)))?;
            struct_ser.serialize_field("maxAudioQuality", &v)?;
        }
        if let Some(v) = self.network_type.as_ref() {
            let v = NetworkType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("networkType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlMaxAudioQualityChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "max_audio_quality",
            "maxAudioQuality",
            "network_type",
            "networkType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            MaxAudioQuality,
            NetworkType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "maxAudioQuality" | "max_audio_quality" => Ok(GeneratedField::MaxAudioQuality),
                            "networkType" | "network_type" => Ok(GeneratedField::NetworkType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlMaxAudioQualityChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlMaxAudioQualityChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlMaxAudioQualityChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut max_audio_quality__ = None;
                let mut network_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxAudioQuality => {
                            if max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAudioQuality"));
                            }
                            max_audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                        GeneratedField::NetworkType => {
                            if network_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkType"));
                            }
                            network_type__ = map_.next_value::<::std::option::Option<NetworkType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(SrvrCtrlMaxAudioQualityChanged {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    max_audio_quality: max_audio_quality__.unwrap_or_default(),
                    network_type: network_type__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlMaxAudioQualityChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueCleared {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueCleared", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueCleared {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueCleared;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueCleared")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueCleared, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueCleared {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueCleared", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueErrorMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueErrorMessage", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueErrorMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueErrorMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueErrorMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueErrorMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SrvrCtrlQueueErrorMessage {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueErrorMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if self.action_uuid.is_some() {
            len += 1;
        }
        if !self.tracks.is_empty() {
            len += 1;
        }
        if self.shuffle_mode {
            len += 1;
        }
        if !self.shuffled_track_indexes.is_empty() {
            len += 1;
        }
        if self.autoplay_mode {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if !self.autoplay_tracks.is_empty() {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueState", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if let Some(v) = self.action_uuid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.tracks.is_empty() {
            struct_ser.serialize_field("tracks", &self.tracks)?;
        }
        if self.shuffle_mode {
            struct_ser.serialize_field("shuffleMode", &self.shuffle_mode)?;
        }
        if !self.shuffled_track_indexes.is_empty() {
            struct_ser.serialize_field("shuffledTrackIndexes", &self.shuffled_track_indexes)?;
        }
        if self.autoplay_mode {
            struct_ser.serialize_field("autoplayMode", &self.autoplay_mode)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if !self.autoplay_tracks.is_empty() {
            struct_ser.serialize_field("autoplayTracks", &self.autoplay_tracks)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "tracks",
            "shuffle_mode",
            "shuffleMode",
            "shuffled_track_indexes",
            "shuffledTrackIndexes",
            "autoplay_mode",
            "autoplayMode",
            "autoplay_loading",
            "autoplayLoading",
            "autoplay_tracks",
            "autoplayTracks",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            Tracks,
            ShuffleMode,
            ShuffledTrackIndexes,
            AutoplayMode,
            AutoplayLoading,
            AutoplayTracks,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "tracks" => Ok(GeneratedField::Tracks),
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            "shuffledTrackIndexes" | "shuffled_track_indexes" => Ok(GeneratedField::ShuffledTrackIndexes),
                            "autoplayMode" | "autoplay_mode" => Ok(GeneratedField::AutoplayMode),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "autoplayTracks" | "autoplay_tracks" => Ok(GeneratedField::AutoplayTracks),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut tracks__ = None;
                let mut shuffle_mode__ = None;
                let mut shuffled_track_indexes__ = None;
                let mut autoplay_mode__ = None;
                let mut autoplay_loading__ = None;
                let mut autoplay_tracks__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Tracks => {
                            if tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracks"));
                            }
                            tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffledTrackIndexes => {
                            if shuffled_track_indexes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffledTrackIndexes"));
                            }
                            shuffled_track_indexes__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::AutoplayMode => {
                            if autoplay_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayMode"));
                            }
                            autoplay_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayTracks => {
                            if autoplay_tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayTracks"));
                            }
                            autoplay_tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueState {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__,
                    tracks: tracks__.unwrap_or_default(),
                    shuffle_mode: shuffle_mode__.unwrap_or_default(),
                    shuffled_track_indexes: shuffled_track_indexes__.unwrap_or_default(),
                    autoplay_mode: autoplay_mode__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    autoplay_tracks: autoplay_tracks__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueTracksAdded {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.tracks.is_empty() {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueTracksAdded", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.tracks.is_empty() {
            struct_ser.serialize_field("tracks", &self.tracks)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueTracksAdded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "tracks",
            "shuffle_seed",
            "shuffleSeed",
            "context_uuid",
            "contextUuid",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            Tracks,
            ShuffleSeed,
            ContextUuid,
            AutoplayReset,
            AutoplayLoading,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "tracks" => Ok(GeneratedField::Tracks),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueTracksAdded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueTracksAdded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueTracksAdded, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut tracks__ = None;
                let mut shuffle_seed__ = None;
                let mut context_uuid__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tracks => {
                            if tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracks"));
                            }
                            tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueTracksAdded {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    tracks: tracks__.unwrap_or_default(),
                    shuffle_seed: shuffle_seed__,
                    context_uuid: context_uuid__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueTracksAdded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueTracksAddedFromAutoplay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueTracksAddedFromAutoplay", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueTracksAddedFromAutoplay {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "queue_item_ids",
            "queueItemIds",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            QueueItemIds,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueTracksAddedFromAutoplay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueTracksAddedFromAutoplay")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueTracksAddedFromAutoplay, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut queue_item_ids__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueTracksAddedFromAutoplay {
                    queue_version: queue_version__,
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueTracksAddedFromAutoplay", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueTracksInserted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.tracks.is_empty() {
            len += 1;
        }
        if self.insert_after.is_some() {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueTracksInserted", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.tracks.is_empty() {
            struct_ser.serialize_field("tracks", &self.tracks)?;
        }
        if let Some(v) = self.insert_after.as_ref() {
            struct_ser.serialize_field("insertAfter", v)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueTracksInserted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "tracks",
            "insert_after",
            "insertAfter",
            "shuffle_seed",
            "shuffleSeed",
            "context_uuid",
            "contextUuid",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            Tracks,
            InsertAfter,
            ShuffleSeed,
            ContextUuid,
            AutoplayReset,
            AutoplayLoading,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "tracks" => Ok(GeneratedField::Tracks),
                            "insertAfter" | "insert_after" => Ok(GeneratedField::InsertAfter),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueTracksInserted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueTracksInserted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueTracksInserted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut tracks__ = None;
                let mut insert_after__ = None;
                let mut shuffle_seed__ = None;
                let mut context_uuid__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tracks => {
                            if tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracks"));
                            }
                            tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InsertAfter => {
                            if insert_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertAfter"));
                            }
                            insert_after__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueTracksInserted {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    tracks: tracks__.unwrap_or_default(),
                    insert_after: insert_after__,
                    shuffle_seed: shuffle_seed__,
                    context_uuid: context_uuid__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueTracksInserted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueTracksLoaded {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.tracks.is_empty() {
            len += 1;
        }
        if self.queue_position != 0 {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if self.shuffle_pivot_queue_item_id.is_some() {
            len += 1;
        }
        if self.shuffle_mode.is_some() {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueTracksLoaded", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.tracks.is_empty() {
            struct_ser.serialize_field("tracks", &self.tracks)?;
        }
        if self.queue_position != 0 {
            struct_ser.serialize_field("queuePosition", &self.queue_position)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if let Some(v) = self.shuffle_pivot_queue_item_id.as_ref() {
            struct_ser.serialize_field("shufflePivotQueueItemId", v)?;
        }
        if let Some(v) = self.shuffle_mode.as_ref() {
            struct_ser.serialize_field("shuffleMode", v)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueTracksLoaded {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "tracks",
            "queue_position",
            "queuePosition",
            "shuffle_seed",
            "shuffleSeed",
            "shuffle_pivot_queue_item_id",
            "shufflePivotQueueItemId",
            "shuffle_mode",
            "shuffleMode",
            "context_uuid",
            "contextUuid",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            Tracks,
            QueuePosition,
            ShuffleSeed,
            ShufflePivotQueueItemId,
            ShuffleMode,
            ContextUuid,
            AutoplayReset,
            AutoplayLoading,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "tracks" => Ok(GeneratedField::Tracks),
                            "queuePosition" | "queue_position" => Ok(GeneratedField::QueuePosition),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "shufflePivotQueueItemId" | "shuffle_pivot_queue_item_id" => Ok(GeneratedField::ShufflePivotQueueItemId),
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueTracksLoaded;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueTracksLoaded")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueTracksLoaded, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut tracks__ = None;
                let mut queue_position__ = None;
                let mut shuffle_seed__ = None;
                let mut shuffle_pivot_queue_item_id__ = None;
                let mut shuffle_mode__ = None;
                let mut context_uuid__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tracks => {
                            if tracks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracks"));
                            }
                            tracks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueuePosition => {
                            if queue_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queuePosition"));
                            }
                            queue_position__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShufflePivotQueueItemId => {
                            if shuffle_pivot_queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shufflePivotQueueItemId"));
                            }
                            shuffle_pivot_queue_item_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = map_.next_value()?;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueTracksLoaded {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    tracks: tracks__.unwrap_or_default(),
                    queue_position: queue_position__.unwrap_or_default(),
                    shuffle_seed: shuffle_seed__,
                    shuffle_pivot_queue_item_id: shuffle_pivot_queue_item_id__,
                    shuffle_mode: shuffle_mode__,
                    context_uuid: context_uuid__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueTracksLoaded", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueTracksRemoved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueTracksRemoved", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueTracksRemoved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "queue_item_ids",
            "queueItemIds",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            QueueItemIds,
            AutoplayReset,
            AutoplayLoading,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueTracksRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueTracksRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueTracksRemoved, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut queue_item_ids__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueTracksRemoved {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueTracksRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlQueueTracksReordered {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if !self.queue_item_ids.is_empty() {
            len += 1;
        }
        if self.insert_after.is_some() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlQueueTracksReordered", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if !self.queue_item_ids.is_empty() {
            struct_ser.serialize_field("queueItemIds", &self.queue_item_ids)?;
        }
        if let Some(v) = self.insert_after.as_ref() {
            struct_ser.serialize_field("insertAfter", v)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlQueueTracksReordered {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "queue_item_ids",
            "queueItemIds",
            "insert_after",
            "insertAfter",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            QueueItemIds,
            InsertAfter,
            AutoplayReset,
            AutoplayLoading,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "queueItemIds" | "queue_item_ids" => Ok(GeneratedField::QueueItemIds),
                            "insertAfter" | "insert_after" => Ok(GeneratedField::InsertAfter),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlQueueTracksReordered;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlQueueTracksReordered")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlQueueTracksReordered, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut queue_item_ids__ = None;
                let mut insert_after__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueItemIds => {
                            if queue_item_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueItemIds"));
                            }
                            queue_item_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::InsertAfter => {
                            if insert_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("insertAfter"));
                            }
                            insert_after__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlQueueTracksReordered {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    queue_item_ids: queue_item_ids__.unwrap_or_default(),
                    insert_after: insert_after__,
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlQueueTracksReordered", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlRemoveRenderer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlRemoveRenderer", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlRemoveRenderer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlRemoveRenderer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlRemoveRenderer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlRemoveRenderer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlRemoveRenderer {
                    renderer_id: renderer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlRemoveRenderer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlRendererStateUpdated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.player_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlRendererStateUpdated", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.status != 0 {
            let v = RendererStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.player_state.as_ref() {
            struct_ser.serialize_field("playerState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlRendererStateUpdated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "status",
            "player_state",
            "playerState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            Status,
            PlayerState,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "status" => Ok(GeneratedField::Status),
                            "playerState" | "player_state" => Ok(GeneratedField::PlayerState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlRendererStateUpdated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlRendererStateUpdated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlRendererStateUpdated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut status__ = None;
                let mut player_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<RendererStatus>()? as i32);
                        }
                        GeneratedField::PlayerState => {
                            if player_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerState"));
                            }
                            player_state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SrvrCtrlRendererStateUpdated {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    player_state: player_state__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlRendererStateUpdated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlSessionState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_uuid.is_empty() {
            len += 1;
        }
        if self.active_renderer_id != 0 {
            len += 1;
        }
        if self.queue_version.is_some() {
            len += 1;
        }
        if self.playing_state != 0 {
            len += 1;
        }
        if self.loop_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlSessionState", len)?;
        if !self.session_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sessionUuid", pbjson::private::base64::encode(&self.session_uuid).as_str())?;
        }
        if self.active_renderer_id != 0 {
            struct_ser.serialize_field("activeRendererId", &self.active_renderer_id)?;
        }
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if self.playing_state != 0 {
            let v = PlayingState::try_from(self.playing_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.playing_state)))?;
            struct_ser.serialize_field("playingState", &v)?;
        }
        if self.loop_mode != 0 {
            let v = LoopMode::try_from(self.loop_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.loop_mode)))?;
            struct_ser.serialize_field("loopMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlSessionState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_uuid",
            "sessionUuid",
            "active_renderer_id",
            "activeRendererId",
            "queue_version",
            "queueVersion",
            "playing_state",
            "playingState",
            "loop_mode",
            "loopMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionUuid,
            ActiveRendererId,
            QueueVersion,
            PlayingState,
            LoopMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sessionUuid" | "session_uuid" => Ok(GeneratedField::SessionUuid),
                            "activeRendererId" | "active_renderer_id" => Ok(GeneratedField::ActiveRendererId),
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "playingState" | "playing_state" => Ok(GeneratedField::PlayingState),
                            "loopMode" | "loop_mode" => Ok(GeneratedField::LoopMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlSessionState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlSessionState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlSessionState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_uuid__ = None;
                let mut active_renderer_id__ = None;
                let mut queue_version__ = None;
                let mut playing_state__ = None;
                let mut loop_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionUuid => {
                            if session_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionUuid"));
                            }
                            session_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ActiveRendererId => {
                            if active_renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeRendererId"));
                            }
                            active_renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::PlayingState => {
                            if playing_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playingState"));
                            }
                            playing_state__ = Some(map_.next_value::<PlayingState>()? as i32);
                        }
                        GeneratedField::LoopMode => {
                            if loop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopMode"));
                            }
                            loop_mode__ = Some(map_.next_value::<LoopMode>()? as i32);
                        }
                    }
                }
                Ok(SrvrCtrlSessionState {
                    session_uuid: session_uuid__.unwrap_or_default(),
                    active_renderer_id: active_renderer_id__.unwrap_or_default(),
                    queue_version: queue_version__,
                    playing_state: playing_state__.unwrap_or_default(),
                    loop_mode: loop_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlSessionState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlShuffleModeSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.queue_version.is_some() {
            len += 1;
        }
        if !self.action_uuid.is_empty() {
            len += 1;
        }
        if self.shuffle_mode {
            len += 1;
        }
        if self.shuffle_seed.is_some() {
            len += 1;
        }
        if self.shuffle_pivot_queue_item_id.is_some() {
            len += 1;
        }
        if self.autoplay_reset {
            len += 1;
        }
        if self.autoplay_loading {
            len += 1;
        }
        if self.queue_hash.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlShuffleModeSet", len)?;
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if !self.action_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionUuid", pbjson::private::base64::encode(&self.action_uuid).as_str())?;
        }
        if self.shuffle_mode {
            struct_ser.serialize_field("shuffleMode", &self.shuffle_mode)?;
        }
        if let Some(v) = self.shuffle_seed.as_ref() {
            struct_ser.serialize_field("shuffleSeed", v)?;
        }
        if let Some(v) = self.shuffle_pivot_queue_item_id.as_ref() {
            struct_ser.serialize_field("shufflePivotQueueItemId", v)?;
        }
        if self.autoplay_reset {
            struct_ser.serialize_field("autoplayReset", &self.autoplay_reset)?;
        }
        if self.autoplay_loading {
            struct_ser.serialize_field("autoplayLoading", &self.autoplay_loading)?;
        }
        if let Some(v) = self.queue_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("queueHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlShuffleModeSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queue_version",
            "queueVersion",
            "action_uuid",
            "actionUuid",
            "shuffle_mode",
            "shuffleMode",
            "shuffle_seed",
            "shuffleSeed",
            "shuffle_pivot_queue_item_id",
            "shufflePivotQueueItemId",
            "autoplay_reset",
            "autoplayReset",
            "autoplay_loading",
            "autoplayLoading",
            "queue_hash",
            "queueHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueueVersion,
            ActionUuid,
            ShuffleMode,
            ShuffleSeed,
            ShufflePivotQueueItemId,
            AutoplayReset,
            AutoplayLoading,
            QueueHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "actionUuid" | "action_uuid" => Ok(GeneratedField::ActionUuid),
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            "shuffleSeed" | "shuffle_seed" => Ok(GeneratedField::ShuffleSeed),
                            "shufflePivotQueueItemId" | "shuffle_pivot_queue_item_id" => Ok(GeneratedField::ShufflePivotQueueItemId),
                            "autoplayReset" | "autoplay_reset" => Ok(GeneratedField::AutoplayReset),
                            "autoplayLoading" | "autoplay_loading" => Ok(GeneratedField::AutoplayLoading),
                            "queueHash" | "queue_hash" => Ok(GeneratedField::QueueHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlShuffleModeSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlShuffleModeSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlShuffleModeSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queue_version__ = None;
                let mut action_uuid__ = None;
                let mut shuffle_mode__ = None;
                let mut shuffle_seed__ = None;
                let mut shuffle_pivot_queue_item_id__ = None;
                let mut autoplay_reset__ = None;
                let mut autoplay_loading__ = None;
                let mut queue_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::ActionUuid => {
                            if action_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionUuid"));
                            }
                            action_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShuffleSeed => {
                            if shuffle_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleSeed"));
                            }
                            shuffle_seed__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ShufflePivotQueueItemId => {
                            if shuffle_pivot_queue_item_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shufflePivotQueueItemId"));
                            }
                            shuffle_pivot_queue_item_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AutoplayReset => {
                            if autoplay_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayReset"));
                            }
                            autoplay_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoplayLoading => {
                            if autoplay_loading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoplayLoading"));
                            }
                            autoplay_loading__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QueueHash => {
                            if queue_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueHash"));
                            }
                            queue_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlShuffleModeSet {
                    queue_version: queue_version__,
                    action_uuid: action_uuid__.unwrap_or_default(),
                    shuffle_mode: shuffle_mode__.unwrap_or_default(),
                    shuffle_seed: shuffle_seed__,
                    shuffle_pivot_queue_item_id: shuffle_pivot_queue_item_id__,
                    autoplay_reset: autoplay_reset__.unwrap_or_default(),
                    autoplay_loading: autoplay_loading__.unwrap_or_default(),
                    queue_hash: queue_hash__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlShuffleModeSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlUpdateRenderer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.device_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlUpdateRenderer", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if let Some(v) = self.device_info.as_ref() {
            struct_ser.serialize_field("deviceInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlUpdateRenderer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "device_info",
            "deviceInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            DeviceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "deviceInfo" | "device_info" => Ok(GeneratedField::DeviceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlUpdateRenderer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlUpdateRenderer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlUpdateRenderer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut device_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceInfo => {
                            if device_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceInfo"));
                            }
                            device_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SrvrCtrlUpdateRenderer {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    device_info: device_info__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlUpdateRenderer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlVolumeChanged {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.volume != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlVolumeChanged", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.volume != 0 {
            struct_ser.serialize_field("volume", &self.volume)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlVolumeChanged {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "volume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            Volume,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "volume" => Ok(GeneratedField::Volume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlVolumeChanged;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlVolumeChanged")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlVolumeChanged, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut volume__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SrvrCtrlVolumeChanged {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    volume: volume__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlVolumeChanged", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrCtrlVolumeMuted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.renderer_id != 0 {
            len += 1;
        }
        if self.value {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrCtrlVolumeMuted", len)?;
        if self.renderer_id != 0 {
            struct_ser.serialize_field("rendererId", &self.renderer_id)?;
        }
        if self.value {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrCtrlVolumeMuted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "renderer_id",
            "rendererId",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RendererId,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rendererId" | "renderer_id" => Ok(GeneratedField::RendererId),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrCtrlVolumeMuted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrCtrlVolumeMuted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrCtrlVolumeMuted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut renderer_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RendererId => {
                            if renderer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rendererId"));
                            }
                            renderer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SrvrCtrlVolumeMuted {
                    renderer_id: renderer_id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrCtrlVolumeMuted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrMuteVolume {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrMuteVolume", len)?;
        if self.value {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrMuteVolume {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrMuteVolume;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrMuteVolume")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrMuteVolume, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SrvrRndrMuteVolume {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrMuteVolume", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrSetActive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.active {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrSetActive", len)?;
        if self.active {
            struct_ser.serialize_field("active", &self.active)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrSetActive {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Active,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "active" => Ok(GeneratedField::Active),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrSetActive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrSetActive")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrSetActive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SrvrRndrSetActive {
                    active: active__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrSetActive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrSetLoopMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.loop_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrSetLoopMode", len)?;
        if self.loop_mode != 0 {
            let v = LoopMode::try_from(self.loop_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.loop_mode)))?;
            struct_ser.serialize_field("loopMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrSetLoopMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "loop_mode",
            "loopMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoopMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "loopMode" | "loop_mode" => Ok(GeneratedField::LoopMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrSetLoopMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrSetLoopMode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrSetLoopMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut loop_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoopMode => {
                            if loop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopMode"));
                            }
                            loop_mode__ = Some(map_.next_value::<LoopMode>()? as i32);
                        }
                    }
                }
                Ok(SrvrRndrSetLoopMode {
                    loop_mode: loop_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrSetLoopMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrSetMaxAudioQuality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_audio_quality != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrSetMaxAudioQuality", len)?;
        if self.max_audio_quality != 0 {
            let v = AudioQuality::try_from(self.max_audio_quality)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_audio_quality)))?;
            struct_ser.serialize_field("maxAudioQuality", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrSetMaxAudioQuality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_audio_quality",
            "maxAudioQuality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxAudioQuality,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxAudioQuality" | "max_audio_quality" => Ok(GeneratedField::MaxAudioQuality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrSetMaxAudioQuality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrSetMaxAudioQuality")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrSetMaxAudioQuality, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_audio_quality__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxAudioQuality => {
                            if max_audio_quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAudioQuality"));
                            }
                            max_audio_quality__ = Some(map_.next_value::<AudioQuality>()? as i32);
                        }
                    }
                }
                Ok(SrvrRndrSetMaxAudioQuality {
                    max_audio_quality: max_audio_quality__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrSetMaxAudioQuality", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrSetShuffleMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.shuffle_mode {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrSetShuffleMode", len)?;
        if self.shuffle_mode {
            struct_ser.serialize_field("shuffleMode", &self.shuffle_mode)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrSetShuffleMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "shuffle_mode",
            "shuffleMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ShuffleMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "shuffleMode" | "shuffle_mode" => Ok(GeneratedField::ShuffleMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrSetShuffleMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrSetShuffleMode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrSetShuffleMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut shuffle_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ShuffleMode => {
                            if shuffle_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleMode"));
                            }
                            shuffle_mode__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SrvrRndrSetShuffleMode {
                    shuffle_mode: shuffle_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrSetShuffleMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrSetState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.playing_state.is_some() {
            len += 1;
        }
        if self.current_position.is_some() {
            len += 1;
        }
        if self.queue_version.is_some() {
            len += 1;
        }
        if self.current_track.is_some() {
            len += 1;
        }
        if self.next_track.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrSetState", len)?;
        if let Some(v) = self.playing_state.as_ref() {
            let v = PlayingState::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("playingState", &v)?;
        }
        if let Some(v) = self.current_position.as_ref() {
            struct_ser.serialize_field("currentPosition", v)?;
        }
        if let Some(v) = self.queue_version.as_ref() {
            struct_ser.serialize_field("queueVersion", v)?;
        }
        if let Some(v) = self.current_track.as_ref() {
            struct_ser.serialize_field("currentTrack", v)?;
        }
        if let Some(v) = self.next_track.as_ref() {
            struct_ser.serialize_field("nextTrack", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrSetState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "playing_state",
            "playingState",
            "current_position",
            "currentPosition",
            "queue_version",
            "queueVersion",
            "current_track",
            "currentTrack",
            "next_track",
            "nextTrack",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayingState,
            CurrentPosition,
            QueueVersion,
            CurrentTrack,
            NextTrack,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playingState" | "playing_state" => Ok(GeneratedField::PlayingState),
                            "currentPosition" | "current_position" => Ok(GeneratedField::CurrentPosition),
                            "queueVersion" | "queue_version" => Ok(GeneratedField::QueueVersion),
                            "currentTrack" | "current_track" => Ok(GeneratedField::CurrentTrack),
                            "nextTrack" | "next_track" => Ok(GeneratedField::NextTrack),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrSetState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrSetState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrSetState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut playing_state__ = None;
                let mut current_position__ = None;
                let mut queue_version__ = None;
                let mut current_track__ = None;
                let mut next_track__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayingState => {
                            if playing_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playingState"));
                            }
                            playing_state__ = map_.next_value::<::std::option::Option<PlayingState>>()?.map(|x| x as i32);
                        }
                        GeneratedField::CurrentPosition => {
                            if current_position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPosition"));
                            }
                            current_position__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::QueueVersion => {
                            if queue_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queueVersion"));
                            }
                            queue_version__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentTrack => {
                            if current_track__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentTrack"));
                            }
                            current_track__ = map_.next_value()?;
                        }
                        GeneratedField::NextTrack => {
                            if next_track__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextTrack"));
                            }
                            next_track__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SrvrRndrSetState {
                    playing_state: playing_state__,
                    current_position: current_position__,
                    queue_version: queue_version__,
                    current_track: current_track__,
                    next_track: next_track__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrSetState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrvrRndrSetVolume {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.volume.is_some() {
            len += 1;
        }
        if self.volume_delta.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.SrvrRndrSetVolume", len)?;
        if let Some(v) = self.volume.as_ref() {
            struct_ser.serialize_field("volume", v)?;
        }
        if let Some(v) = self.volume_delta.as_ref() {
            struct_ser.serialize_field("volumeDelta", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrvrRndrSetVolume {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "volume",
            "volume_delta",
            "volumeDelta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Volume,
            VolumeDelta,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "volume" => Ok(GeneratedField::Volume),
                            "volumeDelta" | "volume_delta" => Ok(GeneratedField::VolumeDelta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrvrRndrSetVolume;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.SrvrRndrSetVolume")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SrvrRndrSetVolume, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut volume__ = None;
                let mut volume_delta__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Volume => {
                            if volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volume"));
                            }
                            volume__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::VolumeDelta => {
                            if volume_delta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("volumeDelta"));
                            }
                            volume_delta__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SrvrRndrSetVolume {
                    volume: volume__,
                    volume_delta: volume_delta__,
                })
            }
        }
        deserializer.deserialize_struct("qconnect.SrvrRndrSetVolume", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrackRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.track_id != 0 {
            len += 1;
        }
        if !self.context_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qconnect.TrackRef", len)?;
        if self.track_id != 0 {
            struct_ser.serialize_field("trackId", &self.track_id)?;
        }
        if !self.context_uuid.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("contextUuid", pbjson::private::base64::encode(&self.context_uuid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrackRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "track_id",
            "trackId",
            "context_uuid",
            "contextUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrackId,
            ContextUuid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "trackId" | "track_id" => Ok(GeneratedField::TrackId),
                            "contextUuid" | "context_uuid" => Ok(GeneratedField::ContextUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrackRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qconnect.TrackRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TrackRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut track_id__ = None;
                let mut context_uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TrackId => {
                            if track_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackId"));
                            }
                            track_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ContextUuid => {
                            if context_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextUuid"));
                            }
                            context_uuid__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TrackRef {
                    track_id: track_id__.unwrap_or_default(),
                    context_uuid: context_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qconnect.TrackRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VolumeRemoteControl {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "VOLUME_REMOTE_CONTROL_UNKNOWN",
            Self::NotAllowed => "VOLUME_REMOTE_CONTROL_NOT_ALLOWED",
            Self::Allowed => "VOLUME_REMOTE_CONTROL_ALLOWED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VolumeRemoteControl {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VOLUME_REMOTE_CONTROL_UNKNOWN",
            "VOLUME_REMOTE_CONTROL_NOT_ALLOWED",
            "VOLUME_REMOTE_CONTROL_ALLOWED",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = VolumeRemoteControl;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VOLUME_REMOTE_CONTROL_UNKNOWN" => Ok(VolumeRemoteControl::Unknown),
                    "VOLUME_REMOTE_CONTROL_NOT_ALLOWED" => Ok(VolumeRemoteControl::NotAllowed),
                    "VOLUME_REMOTE_CONTROL_ALLOWED" => Ok(VolumeRemoteControl::Allowed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
