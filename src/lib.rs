#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod config;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate nanoid;

use crate::api::{GetRooms, GetWalls};
use crowdstress_common::prelude::*;
use opencv::core;
use opencv::core::{MatExprTrait, MatTraitManual};
use opencv::imgproc;
use opencv::types;
use rocket::Rocket;
use rocket_contrib::json::Json;
use std::cmp;
use std::process::exit;

#[post("/walls", format = "json", data = "<data>")]
fn walls(data: Json<GetWalls>) -> Json<Vec<Section>> {
    let walls: Vec<Section> = data
        .objects
        .iter()
        .map(|object| Section::from_object(object))
        .flatten()
        .collect();
    Json(walls)
}

#[post("/rooms", format = "json", data = "<data>")]
fn rooms(data: Json<GetRooms>) -> Json<Vec<Room>> {
    let mut polygons: Vec<Polygon> = Vec::new();
    let eps = data.epsilon;
    let exits: Vec<Exit> = data
        .objects
        .iter()
        .filter(|object| object.object_type == 4)
        .map(|object| Exit {
            id: String::from(&object.id),
            section: Section {
                start: object.points[0],
                end: object.points[1],
            },
        })
        .collect();
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
            .map(|point| Point::new(point[0] as f64, point[1] as f64))
            .collect();
        polygons.push(points);
    }

    polygons.remove(0); // Удаляем внешний контур

    let mut rooms: Vec<Room> = Vec::with_capacity(polygons.len());

    for polygon in polygons {
        let sections = geometry::polygon_to_sections(&polygon);
        let mut exit_ids: Vec<String> = Vec::new();

        for section in sections {
            for exit in &exits {
                if exit_ids.contains(&exit.id) {
                    continue;
                }

                let exit_point1 = &exit.section.start;
                let exit_point2 = &exit.section.end;

                let vector1 = geometry::get_vector_to_line(&section, exit_point1);
                let vector2 = geometry::get_vector_to_line(&section, exit_point2);

                let is_intersects1 = geometry::is_lines_intersects(
                    &vector1.product(999.0).to_line(*exit_point1),
                    &section,
                );

                let is_intersects2 = geometry::is_lines_intersects(
                    &vector2.product(999.0).to_line(*exit_point1),
                    &section,
                );

                if is_intersects1
                    && is_intersects2
                    && vector1.get_length() as i32 == 0
                    && vector2.get_length() as i32 == 0
                {
                    exit_ids.push(String::from(&exit.id));
                }
            }
        }

        rooms.push(Room {
            id: nanoid::nanoid!(),
            points: polygon,
            exits: exit_ids,
        });
    }

    Json(rooms)
}

fn objects_to_matrix(data: GetRooms) -> core::Mat {
    let matrix = match core::Mat::zeros(data.height, data.width, 0) {
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
            let point1 = object.points[0].to_i32();
            let point2 = object.points[1].to_i32();
            match imgproc::line(
                &mut matrix,
                core::Point {
                    x: point1.x,
                    y: point1.y,
                },
                core::Point {
                    x: point2.x,
                    y: point2.y,
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
            let point1 = object.points[0].to_i32();
            let point2 = object.points[1].to_i32();
            let width = (point2.x - point1.x).abs();
            let height = (point2.y - point1.y).abs();
            match imgproc::rectangle(
                &mut matrix,
                core::Rect {
                    x: cmp::min(point1.x, point2.x),
                    y: cmp::min(point1.y, point2.y),
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

pub fn start() -> Rocket {
    rocket::ignite().mount("/", routes![walls, rooms])
}
