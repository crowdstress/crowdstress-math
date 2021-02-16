#![feature(proc_macro_hygiene, decl_macro)]

#[path = "models/api.rs"]
mod api;
#[path = "config.rs"]
mod config;
#[path = "models/object.rs"]
mod object;
#[path = "models/primitives.rs"]
mod primitives;
#[path = "models/room.rs"]
mod room;
#[path = "models/wall.rs"]
mod wall;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate nanoid;

use crate::api::Raster;
use crate::object::DrawingObject;
use crate::room::Room;
use crate::wall::Wall;
use opencv::core;
use opencv::core::{MatTraitManual, MatExprTrait};
use opencv::imgproc;
use opencv::types;
use rocket_contrib::json::Json;
use std::process::exit;

#[post("/walls", format = "json", data = "<objects>")]
fn walls(objects: Json<Vec<DrawingObject>>) -> Json<Vec<Wall>> {
    let walls: Vec<Wall> = objects
        .iter()
        .map(|object| Wall::from_object(object))
        .flatten()
        .collect();
    Json(walls)
}

#[post("/rooms", format = "json", data = "<data>")]
fn rooms(data: Json<Raster>) -> Json<Vec<Room>> {
    let mut polygons: Vec<Room> = Vec::new();
    let eps = data.epsilon;
    let object_matrix = objects_to_matrix(data.into_inner());
    let mut contours = types::VectorOfMat::new();
    match imgproc::find_contours(
        &object_matrix,
        &mut contours,
        imgproc::RETR_CCOMP,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point { x: 0, y: 0 },
    ) {
        Err(error) => {
            println!("Find contours error {}", error);
        }
        _ => {}
    }

    for contour in contours {
        let mut curve = match core::Mat::default() {
            Ok(value) => value,
            Err(error) => {
                println!("Create matrix error, {}", error);
                exit(-1);
            }
        };
        match imgproc::approx_poly_dp(&contour, &mut curve, eps, true) {
            Err(error) => {
                println!("Approximation error {}", error);
            }
            _ => {}
        }

        let data = match curve.data_typed::<core::Vec2<i32>>() {
            Ok(value) => value,
            Err(error) => {
                println!("Data typed error {}", error);
                exit(-1);
            }
        };
        let points = data
            .to_vec()
            .iter()
            .map(|point| [point[0], point[1]])
            .collect();
        polygons.push(Room {
            id: nanoid::nanoid!(),
            points,
        });
    }

    polygons.remove(0); // Удаляем внешний контур

    Json(polygons)
}

fn objects_to_matrix(data: Raster) -> core::Mat {
    let matrix = match core::Mat::zeros(data.width, data.height, 0) {
        Ok(value) => value,
        Err(error) => {
            println!("Create matrix error, {}", error);
            exit(-1);
        }
    };

    let mut matrix = match matrix.to_mat() {
        Ok(value) => value,
        Err(error) => {
            println!("Convert matrix error, {}", error);
            exit(-1);
        }
    };

    for object in &data.objects {
        if object.object_type == 0 || object.object_type == 4 {
            let point1 = object.points[0];
            let point2 = object.points[1];
            match imgproc::line(
                &mut matrix,
                core::Point {
                    x: point1[0],
                    y: point1[1],
                },
                core::Point {
                    x: point2[0],
                    y: point2[1],
                },
                core::Scalar::from(255.0),
                config::OBJECT_LINE_WIDTH,
                config::OBJECT_LINE_TYPE,
                0,
            ) {
                Err(error) => println!("Draw line error {}", error),
                _ => {}
            };
        }

        if object.object_type == 1 {
            let point1 = object.points[0];
            let point2 = object.points[1];
            let width = (point2[0] as f64 - point1[0] as f64).abs() as i32;
            let height = (point2[1] as f64 - point1[1] as f64).abs() as i32;
            match imgproc::rectangle(
                &mut matrix,
                core::Rect {
                    x: point1[0],
                    y: point1[1],
                    width,
                    height,
                },
                core::Scalar::from(255.0),
                config::OBJECT_LINE_WIDTH,
                config::OBJECT_LINE_TYPE,
                0,
            ) {
                Err(error) => println!("Draw rectangle error {}", error),
                _ => {}
            };
        }
    }

    matrix
}

fn main() {
    rocket::ignite().mount("/", routes![walls, rooms]).launch();
}
