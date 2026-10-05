impl serde::Serialize for Authenticate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg_id != 0 {
            len += 1;
        }
        if self.msg_date != 0 {
            len += 1;
        }
        if !self.jwt.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qcloud.Authenticate", len)?;
        if self.msg_id != 0 {
            struct_ser.serialize_field("msgId", &self.msg_id)?;
        }
        if self.msg_date != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("msgDate", ToString::to_string(&self.msg_date).as_str())?;
        }
        if !self.jwt.is_empty() {
            struct_ser.serialize_field("jwt", &self.jwt)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Authenticate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg_id",
            "msgId",
            "msg_date",
            "msgDate",
            "jwt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgId,
            MsgDate,
            Jwt,
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
                            "msgId" | "msg_id" => Ok(GeneratedField::MsgId),
                            "msgDate" | "msg_date" => Ok(GeneratedField::MsgDate),
                            "jwt" => Ok(GeneratedField::Jwt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Authenticate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qcloud.Authenticate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Authenticate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg_id__ = None;
                let mut msg_date__ = None;
                let mut jwt__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MsgId => {
                            if msg_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgId"));
                            }
                            msg_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MsgDate => {
                            if msg_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgDate"));
                            }
                            msg_date__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Jwt => {
                            if jwt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwt"));
                            }
                            jwt__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Authenticate {
                    msg_id: msg_id__.unwrap_or_default(),
                    msg_date: msg_date__.unwrap_or_default(),
                    jwt: jwt__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qcloud.Authenticate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHANNEL_TYPE_UNSPECIFIED",
            Self::Device => "CHANNEL_TYPE_DEVICE",
            Self::Backend => "CHANNEL_TYPE_BACKEND",
            Self::SessionControllers => "CHANNEL_TYPE_SESSION_CONTROLLERS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChannelType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHANNEL_TYPE_UNSPECIFIED",
            "CHANNEL_TYPE_DEVICE",
            "CHANNEL_TYPE_BACKEND",
            "CHANNEL_TYPE_SESSION_CONTROLLERS",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = ChannelType;

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
                    "CHANNEL_TYPE_UNSPECIFIED" => Ok(ChannelType::Unspecified),
                    "CHANNEL_TYPE_DEVICE" => Ok(ChannelType::Device),
                    "CHANNEL_TYPE_BACKEND" => Ok(ChannelType::Backend),
                    "CHANNEL_TYPE_SESSION_CONTROLLERS" => Ok(ChannelType::SessionControllers),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Disconnect {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg_id != 0 {
            len += 1;
        }
        if self.msg_date != 0 {
            len += 1;
        }
        if self.reconnect {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qcloud.Disconnect", len)?;
        if self.msg_id != 0 {
            struct_ser.serialize_field("msgId", &self.msg_id)?;
        }
        if self.msg_date != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("msgDate", ToString::to_string(&self.msg_date).as_str())?;
        }
        if self.reconnect {
            struct_ser.serialize_field("reconnect", &self.reconnect)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Disconnect {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg_id",
            "msgId",
            "msg_date",
            "msgDate",
            "reconnect",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgId,
            MsgDate,
            Reconnect,
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
                            "msgId" | "msg_id" => Ok(GeneratedField::MsgId),
                            "msgDate" | "msg_date" => Ok(GeneratedField::MsgDate),
                            "reconnect" => Ok(GeneratedField::Reconnect),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Disconnect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qcloud.Disconnect")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Disconnect, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg_id__ = None;
                let mut msg_date__ = None;
                let mut reconnect__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MsgId => {
                            if msg_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgId"));
                            }
                            msg_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MsgDate => {
                            if msg_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgDate"));
                            }
                            msg_date__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Reconnect => {
                            if reconnect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reconnect"));
                            }
                            reconnect__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Disconnect {
                    msg_id: msg_id__.unwrap_or_default(),
                    msg_date: msg_date__.unwrap_or_default(),
                    reconnect: reconnect__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qcloud.Disconnect", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MESSAGE_TYPE_UNSPECIFIED",
            Self::Authenticate => "AUTHENTICATE",
            Self::Subscribe => "SUBSCRIBE",
            Self::Unsubscribe => "UNSUBSCRIBE",
            Self::Payload => "PAYLOAD",
            Self::Error => "ERROR",
            Self::Disconnect => "DISCONNECT",
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
            "MESSAGE_TYPE_UNSPECIFIED",
            "AUTHENTICATE",
            "SUBSCRIBE",
            "UNSUBSCRIBE",
            "PAYLOAD",
            "ERROR",
            "DISCONNECT",
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
                    "MESSAGE_TYPE_UNSPECIFIED" => Ok(MessageType::Unspecified),
                    "AUTHENTICATE" => Ok(MessageType::Authenticate),
                    "SUBSCRIBE" => Ok(MessageType::Subscribe),
                    "UNSUBSCRIBE" => Ok(MessageType::Unsubscribe),
                    "PAYLOAD" => Ok(MessageType::Payload),
                    "ERROR" => Ok(MessageType::Error),
                    "DISCONNECT" => Ok(MessageType::Disconnect),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Payload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg_id != 0 {
            len += 1;
        }
        if self.msg_date != 0 {
            len += 1;
        }
        if self.proto.is_some() {
            len += 1;
        }
        if self.src.is_some() {
            len += 1;
        }
        if !self.dests.is_empty() {
            len += 1;
        }
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qcloud.Payload", len)?;
        if self.msg_id != 0 {
            struct_ser.serialize_field("msgId", &self.msg_id)?;
        }
        if self.msg_date != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("msgDate", ToString::to_string(&self.msg_date).as_str())?;
        }
        if let Some(v) = self.proto.as_ref() {
            struct_ser.serialize_field("proto", v)?;
        }
        if let Some(v) = self.src.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("src", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.dests.is_empty() {
            struct_ser.serialize_field("dests", &self.dests.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Payload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg_id",
            "msgId",
            "msg_date",
            "msgDate",
            "proto",
            "src",
            "dests",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgId,
            MsgDate,
            Proto,
            Src,
            Dests,
            Payload,
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
                            "msgId" | "msg_id" => Ok(GeneratedField::MsgId),
                            "msgDate" | "msg_date" => Ok(GeneratedField::MsgDate),
                            "proto" => Ok(GeneratedField::Proto),
                            "src" => Ok(GeneratedField::Src),
                            "dests" => Ok(GeneratedField::Dests),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Payload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qcloud.Payload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Payload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg_id__ = None;
                let mut msg_date__ = None;
                let mut proto__ = None;
                let mut src__ = None;
                let mut dests__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MsgId => {
                            if msg_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgId"));
                            }
                            msg_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MsgDate => {
                            if msg_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgDate"));
                            }
                            msg_date__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proto => {
                            if proto__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proto"));
                            }
                            proto__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Src => {
                            if src__.is_some() {
                                return Err(serde::de::Error::duplicate_field("src"));
                            }
                            src__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Dests => {
                            if dests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dests"));
                            }
                            dests__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Payload {
                    msg_id: msg_id__.unwrap_or_default(),
                    msg_date: msg_date__.unwrap_or_default(),
                    proto: proto__,
                    src: src__,
                    dests: dests__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qcloud.Payload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Proto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "PROTO_UNKNOWN",
            Self::Qconnect => "PROTO_QCONNECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Proto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROTO_UNKNOWN",
            "PROTO_QCONNECT",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = Proto;

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
                    "PROTO_UNKNOWN" => Ok(Proto::Unknown),
                    "PROTO_QCONNECT" => Ok(Proto::Qconnect),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Subscribe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg_id != 0 {
            len += 1;
        }
        if self.msg_date != 0 {
            len += 1;
        }
        if self.proto != 0 {
            len += 1;
        }
        if !self.channels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qcloud.Subscribe", len)?;
        if self.msg_id != 0 {
            struct_ser.serialize_field("msgId", &self.msg_id)?;
        }
        if self.msg_date != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("msgDate", ToString::to_string(&self.msg_date).as_str())?;
        }
        if self.proto != 0 {
            struct_ser.serialize_field("proto", &self.proto)?;
        }
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Subscribe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg_id",
            "msgId",
            "msg_date",
            "msgDate",
            "proto",
            "channels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgId,
            MsgDate,
            Proto,
            Channels,
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
                            "msgId" | "msg_id" => Ok(GeneratedField::MsgId),
                            "msgDate" | "msg_date" => Ok(GeneratedField::MsgDate),
                            "proto" => Ok(GeneratedField::Proto),
                            "channels" => Ok(GeneratedField::Channels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Subscribe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qcloud.Subscribe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Subscribe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg_id__ = None;
                let mut msg_date__ = None;
                let mut proto__ = None;
                let mut channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MsgId => {
                            if msg_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgId"));
                            }
                            msg_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MsgDate => {
                            if msg_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgDate"));
                            }
                            msg_date__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proto => {
                            if proto__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proto"));
                            }
                            proto__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(Subscribe {
                    msg_id: msg_id__.unwrap_or_default(),
                    msg_date: msg_date__.unwrap_or_default(),
                    proto: proto__.unwrap_or_default(),
                    channels: channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qcloud.Subscribe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Unsubscribe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg_id != 0 {
            len += 1;
        }
        if self.msg_date != 0 {
            len += 1;
        }
        if self.proto != 0 {
            len += 1;
        }
        if !self.channels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qcloud.Unsubscribe", len)?;
        if self.msg_id != 0 {
            struct_ser.serialize_field("msgId", &self.msg_id)?;
        }
        if self.msg_date != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("msgDate", ToString::to_string(&self.msg_date).as_str())?;
        }
        if self.proto != 0 {
            struct_ser.serialize_field("proto", &self.proto)?;
        }
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Unsubscribe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg_id",
            "msgId",
            "msg_date",
            "msgDate",
            "proto",
            "channels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MsgId,
            MsgDate,
            Proto,
            Channels,
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
                            "msgId" | "msg_id" => Ok(GeneratedField::MsgId),
                            "msgDate" | "msg_date" => Ok(GeneratedField::MsgDate),
                            "proto" => Ok(GeneratedField::Proto),
                            "channels" => Ok(GeneratedField::Channels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Unsubscribe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qcloud.Unsubscribe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Unsubscribe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg_id__ = None;
                let mut msg_date__ = None;
                let mut proto__ = None;
                let mut channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MsgId => {
                            if msg_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgId"));
                            }
                            msg_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MsgDate => {
                            if msg_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgDate"));
                            }
                            msg_date__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proto => {
                            if proto__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proto"));
                            }
                            proto__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(Unsubscribe {
                    msg_id: msg_id__.unwrap_or_default(),
                    msg_date: msg_date__.unwrap_or_default(),
                    proto: proto__.unwrap_or_default(),
                    channels: channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qcloud.Unsubscribe", FIELDS, GeneratedVisitor)
    }
}
