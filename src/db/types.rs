use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UserSettings {
  pub first_open: u8,
  pub name: String,
  pub target_cals: u16,
  pub target_carbs: u16,
}

pub static NUTRITION_VALUES: [&str; 12] = [
  "calories",
  "protein",
  "fat_total",
  "fat_sat",
  "fat_trans",
  "cholesterol",
  "carbs_total",
  "fiber",
  "sugar",
  "carbs_net",
  "sodium",
  "potassium",
];

#[derive(Debug, Clone)]
pub struct NutritionEntry {
  pub name: String,
  pub amount: f64,
  pub serv_size: f64,
  pub data: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct FoodEntry {
  pub name: String,
  pub datetime: DateTime<Utc>,
  pub amount: f64,
  pub nutrition_id: u16,
  pub nutrition_data: Option<NutritionEntry>,
}

#[derive(Debug)]
pub struct DailyInfo {
  pub date: String,
  pub entries: Vec<FoodEntry>,
}

#[derive(Debug, Clone)]
pub struct SettingsEntry {
  pub name: String,
  pub value: String,
  pub visible: bool,
}

pub struct SettingsManager {
  pub settings: HashMap<String, SettingsEntry>,
}

pub struct DatabaseManager {
  pub settings: SettingsManager,

  pub db_path: String,
  pub conn: Connection,
}
