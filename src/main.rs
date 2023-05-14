use anyhow::Error;
use std::{io, thread, time::Duration};

use crate::db::types::{DatabaseManager, FoodEntry, NutritionEntry};

pub mod db;
mod ui;

fn test(
  db_man: &mut DatabaseManager,
  populate_entries: bool,
  test_entries: bool,
  populate_nutrition: bool,
  test_nutrition: bool,
) -> Result<(), Error> {
  if populate_entries {
    let tests = [
      "2012-12-12T12:12:12Z",
      "2018-12-12T12:12:12Z",
      "2023-01-12T12:12:12Z",
      "2023-01-13T12:12:12Z",
      "2023-01-01T12:12:12Z",
    ];
    for t in tests.iter() {
      let entry = FoodEntry {
        name: "testing".to_string(),
        datetime: t.parse::<chrono::DateTime<chrono::Utc>>()?,
        nutrition_id: 69,
      };
      db_man.insert_into_journal(&entry)?;
    }
  }

  if test_entries {
    let journal = db_man.get_journal_for_day(chrono::Utc::now().date_naive())?;
    println!("\nJournal for today:\n");
    for (k, v) in journal {
      println!("{k}: {:#?}", v);
    }

    let journal = db_man.get_journal_for_week(2023, 2)?;
    println!("Journal for week 2 of 2023:\n");
    for (k, v) in journal {
      println!("{k}: {:#?}", v);
    }

    let journal = db_man.get_journal_for_month(2023, 1)?;
    println!("\nJournal for month 1 of 2023:\n");
    for (k, v) in journal {
      println!("{k}: {:#?}", v);
    }

    let journal = db_man.get_journal_for_year(2018)?;
    println!("\nJournal for all of 2018:\n");
    for (k, v) in journal {
      println!("{k}: {:#?}", v);
    }

    let journal = db_man.get_journal_for_range(
      chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
      chrono::NaiveDate::from_ymd_opt(2023, 5, 12).unwrap(),
    )?;
    println!("\nJournal for 2000-today:\n");
    for (k, v) in journal {
      println!("{k}: {:#?}", v);
    }
  }

  if populate_nutrition {
    let avocado = NutritionEntry {
      name: "Avocado".to_string(),
      serv_size: 100.0,
      calories: 160,
      protein: Some(2.0),
      fat_total: Some(14.7),
      fat_sat: None,
      fat_trans: None,
      cholesterol: None,
      carbs_total: Some(8.5),
      fiber: Some(6.7),
      sugar: None,
      carbs_net: Some(1.8),
      sodium: Some(7.0),
      potassium: Some(485.0),
    };

    println!("Populating nutrition database...");
    db_man.insert_into_nutridata(&avocado)?;
  }

  if test_nutrition {
    println!("Testing nutrition database...");
    let avocado = db_man.get_nutridata(1);
    println!("{:#?}", avocado);
  }

  Ok(())
}

fn main() -> Result<(), Error> {
  // Load database
  let mut db_man = DatabaseManager::new("kalorie.db".to_string())?;

  let settings = db_man.settings.get()?;
  println!("Loaded settings:\n{:#?}", settings);

  test(&mut db_man, false, false, true, true)?;

  Ok(())
}
