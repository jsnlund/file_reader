// use polars::series::Series;
// use polars::prelude::NamedFrom;
// use polars::prelude::{NamedFrom, DataFrame};
// use polars::df;
// use chrono::NaiveDate;
use polars::prelude::*;
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_secs(2);

    let x = three();
    let y = this_series();
    let z = that_series(&y);

    println!("The value of x is {x}");
    println!("...and the series is {y}");
    println!("{:?}",z);

    thread::sleep(delay)
    // let s = Series::new("a", [1,2,3,4]);
    // println!("{}", s);
}

fn three() -> i8 {
    3
}

fn this_series() -> Series {
    Series::new("a", 0..10i32)
    // println!("{}", s);
}

fn that_series(a: &Series) -> Series {
    a.filter(&a.gt(7i32).unwrap()).unwrap()
}

// fn foo() -> DataFrame {
//     let df: DataFrame = df!("integer" => &[1,2,3,4,5],
//     "date" => &[
//         NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
//         NaiveDate::from_ymd_opt(2022, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
//         NaiveDate::from_ymd_opt(2022, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
//         NaiveDate::from_ymd_opt(2022, 1, 4).unwrap().and_hms_opt(0, 0, 0).unwrap(),
//         NaiveDate::from_ymd_opt(2022, 1, 5).unwrap().and_hms_opt(0, 0, 0).unwrap()
//     ],
//     "float" => &[4.0, 5.0, 6.0, 7.0, 8.0]
// ).expect("should not fail");
// }