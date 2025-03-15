use chrono::{DateTime, NaiveDate, TimeZone as _, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub result: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub autonid: i32,
    pub stage: i32,
    pub client_id: String,
    pub cif_id: String,
    pub is_revised: bool,
    pub is_rejected: bool,
    pub is_finished: bool,
    pub account_status: i32,
    pub mobile_phone: String,
    pub email: String,
    pub full_name: String,
    pub spouse_relationship: i32,
    pub spouse_name: String,
    pub mother_name: String,
    pub nationality: i32,
    pub idcard_country: String,
    pub idcard_number: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub idcard_expire_date: chrono::DateTime<Utc>,
    pub sex: i32,
    pub birth_place: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub birth_date: chrono::DateTime<Utc>,
    pub birth_country: String,
    pub religion: i32,
    pub marital_status: i32,
    pub education: i32,
    pub idcard_city: i32,
    pub idcard_district: String,
    pub idcard_subdistrict: String,
    pub idcard_rt: String,
    pub idcard_rw: String,
    pub idcard_zipcode: String,
    pub idcard_address: String,
    pub copy_id: bool,
    pub domicile_city: i32,
    pub domicile_district: String,
    pub domicile_subdistrict: String,
    pub domicile_rt: String,
    pub domicile_rw: String,
    pub domicile_address: String,
    pub domicile_zipcode: String,
    pub question_rdn: i32,
    pub bank_name: String,
    pub bank_branch: String,
    pub bank_account_number: String,
    pub bank_account_holder: String,
    pub question_npwp: i32,
    pub npwp_number: String,
    pub npwp_reason: String,
    pub company_name: String,
    pub fund_source: String,
    pub fund_source_text: String,
    pub occupation: i32,
    pub occupation_text: String,
    pub nature_bussiness: i32,
    pub nature_bussiness_text: String,
    pub position: i32,
    pub position_text: String,
    pub income_peranum: i32,
    pub question_1: bool,
    pub question_1text: String,
    pub question_2: bool,
    pub question_2text: String,
    pub question_3: bool,
    pub question_3text: String,
    pub question_4: bool,
    pub question_4text: String,
    pub question_5: bool,
    pub question_5text: String,
    pub question_6: bool,
    pub question_6text: String,
    pub invesment_objective: i32,
    pub risk: i32,
    pub question_fatca: String,
    pub fatca_1: String,
    pub fatca_2: String,
    pub fatca_3: String,
    pub spouse_income_peranum: i32,
    pub spouse_occupation: i32,
    pub spouse_occupation_text: String,
    pub spouse_position: i32,
    pub spouse_nature_bussiness: i32,
    pub spouse_fund_source: String,
    pub spouse_fund_source_text: String,
    pub spouse_company_name: String,
    pub spouse_company_city: i32,
    pub spouse_company_address: String,
    pub spouse_company_zipcode: String,
    pub idcard_file: String,
    pub selfie_file: String,
    pub signature_file: String,
    pub npwp_file: String,
    pub sales: i32,
    pub company_city: i32,
    pub company_address: String,
    pub company_zipcode: String,
    pub beneficiary_owner: i32,
    pub residence_status: i32,
}

fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

fn deserialize_date_only<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;
    if let Some(date) = date_str {
        let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)?;
        let datetime = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
        return Ok(Some(datetime));
    }
    Ok(None)
}