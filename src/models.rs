/*
`id` int NOT NULL AUTO_INCREMENT,
`job_id` int NOT NULL DEFAULT '0',
`node_address` varchar(128) COLLATE utf8mb4_bin NOT NULL DEFAULT '',
`type` int NOT NULL DEFAULT '1',
`real_auditor` varchar(128) COLLATE utf8mb4_bin NOT NULL DEFAULT '',
`permitted` varchar(9) COLLATE utf8mb4_bin NOT NULL DEFAULT '',
`reason` text COLLATE utf8mb4_bin,
`candidate_auditor` varchar(256) COLLATE utf8mb4_bin NOT NULL DEFAULT '',
`created_at` datetime(3) DEFAULT CURRENT_TIMESTAMP(3),
`updated_at` datetime(3) DEFAULT CURRENT_TIMESTAMP(3),
`raw_body` text COLLATE utf8mb4_bin,
`real_auditor_name` varchar(128) COLLATE utf8mb4_bin NOT NULL DEFAULT '',
*/
pub mod models {
    pub mod my_date_format {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Deserializer, Serializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }

    pub mod my_date_format_optional {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Deserializer, Serializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if let Some(s) = date {
                let s = format!("{}", s.format(FORMAT));
                serializer.serialize_str(&s)
            } else {
                serializer.serialize_str("")
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s == "" {
                return Ok(None);
            }
            NaiveDateTime::parse_from_str(&s, FORMAT)
                .map(|v| Some(v))
                .map_err(serde::de::Error::custom)
        }
    }

    use serde::{Deserialize, Serialize};
    use sqlx::{types::chrono, FromRow};

    #[derive(Debug, Deserialize, Serialize, FromRow)]
    pub struct AuditInfo {
        pub id: i64,
        pub job_id: i64,
        pub node_address: String,
        pub audit_type: i16,
        pub real_auditor: String,
        pub permitted: String,
        pub reason: String,
        pub candidate_auditor: String,
        #[serde(with = "my_date_format")]
        pub created_at: chrono::NaiveDateTime,
        #[serde(with = "my_date_format")]
        pub updated_at: chrono::NaiveDateTime,
        pub raw_body: String,
        pub real_auditor_name: String,
    }
}
