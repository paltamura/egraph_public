use crate::types::{self, Domain, Point};

pub fn join_doms(domains: Vec<Domain>) -> Domain {
    let mut new_name = "".to_string();
    let mut new_vec = vec![];
    for dom in domains {
        new_name = format!("{}_{}", new_name, dom.0);
        new_vec.extend(dom.1);
    }
    types::Domain(new_name, new_vec)
}

pub fn join_points(points: Vec<Point>) -> Point {
    let mut new_vec = vec![];
    for point in points {
        new_vec.extend(point.0.clone());
    }
    types::Point(new_vec)
}

// pub fn split_doms(domain: Domain, domain_sizes: Vec<u32>) -> Vec<Domain> {
//     todo!()
// }

pub fn split_points(point: Point, point_sizes: Vec<usize>) -> Vec<Point> {
    let sum: usize = point_sizes.iter().sum();
    let len = point.0.len();
    let eq = sum.eq(&len);
    if !eq {
        panic!();
    }
    let mut new_points = vec![];
    let mut clon = point.0.clone();
    for size in point_sizes {
        let mut range = vec![];
        for _i in 0..size {
            range.push(clon.get(0).unwrap().clone());
            clon.drain(0..1);
        }
        let new_point = Point(range);
        new_points.push(new_point);
    }
    new_points
}

pub fn xploit(domain: Domain) -> Vec<Point> {
    let mut vecvec = vec![];
    for dim in domain.1 {
        vecvec.push(dim.1.clone());
    }
    let products = cartesian_product(vecvec);
    let mut points = vec![];
    for product in products {
        points.push(types::Point(product));
    }
    points
}

fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: Vec<T>) -> Vec<Vec<T>> {
    a.into_iter()
        .flat_map(|xs| {
            b.iter()
                .cloned()
                .map(|y| {
                    let mut vec = xs.clone();
                    vec.push(y);
                    vec
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn cartesian_product<T: Clone>(lists: Vec<Vec<T>>) -> Vec<Vec<T>> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();

            rest.iter()
                .cloned()
                .fold(init, |vec, list| partial_cartesian(vec, list))
        }
        None => {
            vec![]
        }
    }
}