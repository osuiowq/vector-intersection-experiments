extern crate array_tool;
use array_tool::vec::*;
use rand::Rng;
use std::cmp::Ordering;
use std::time::{Instant, Duration};

const sentinel : i32 = <i32>::max_value();
const num_iterations :u32 = 20;

fn main() {
    let mut total_list_len : u32= 0;
    let mut result1 = Duration::new(0,0);
    let mut result2 = Duration::new(0,0);
    let mut result3 = Duration::new(0,0);
 
    for i in 0..num_iterations {
        let l1 = random_vec().unique();
        let l2 = random_vec().unique();
        total_list_len = total_list_len +l1.len() as u32 +l2.len() as u32;
        //println!("l1: {:?} \n l2: {:?}",l1,l2);
        println!("\nIntersection 1 with CMP and Orderings");
        let mut start = Instant::now();
        let mut res = intersect1(l1.clone(), l2.clone());
        let mut duration = start.elapsed();
        result1 = result1 + duration;
        println!("Duration: {:?} with {:?} results", duration, res.len());
        println!("\nIntersection 2 with while loops");
        start = Instant::now();
        res = intersect2(l1.clone(), l2.clone());
        duration = start.elapsed();
        result2 = result2 + duration;
        println!("Duration: {:?} with {:?} results", duration, res.len());
        println!("\nIntersection 3 with array_tool");
        start = Instant::now();
        let res2 = l1.intersect(l2);
        duration = start.elapsed();
        result3 = result3 + duration;
        println!("Duration: {:?} with {:?} results", duration, res2.len());

        //println!("INTERSECTION {:?} \n RES: {:?} ARRAYTOOLS: {:?}", intersect2(res.clone(), res2.clone()), res, res2)
    }
    println!("\n\nTotal with {:?} runs: \nCMP & Orderings {:?} \nWhile loops {:?} \narray_tool: {:?}",num_iterations, (result1/num_iterations),result2/num_iterations,result3/num_iterations);
    println!("Average list length {}",(total_list_len/(2_u32 * num_iterations)));
}
fn random_vec() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::new();

    for _ in 0..rng.gen_range(1, 1000) {
        numbers.push(rng.gen_range(1, 10000));
    }
    numbers.sort();
    println!("\n Numbers: {:?} items", numbers.len());
    numbers
}

fn intersect1(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    let (short, long): (Vec<i32>, Vec<i32>) = if l1.len() < l2.len() {
        (l1, l2)
    } else {
        (l2, l1)
    };
    let mut results = Vec::new();
    let mut i_s = 0;
    let mut i_l = 0;
    while i_s < short.len() && i_l < long.len() {
        match short[i_s].cmp(&long[i_l]) {
            Ordering::Greater => i_l += 1,
            Ordering::Less => i_s += 1,
            Ordering::Equal => {
                results.push(short[i_s].clone());
                i_s += 1;
                i_l += 1;
            }
        }
    }
    results
}

fn intersect2(mut l1: Vec<i32>, mut l2: Vec<i32>) -> Vec<i32> {
    let (mut short, mut long): (Vec<i32>, Vec<i32>) = if l1.len() < l2.len() {
        (l1, l2)
    } else {
        (l2, l1)
    };
    
    let mut results = Vec::new();
    short.push(sentinel);
    long.push(sentinel);
    let mut i_s = 0;
    let mut i_l = 0;
    while i_s < short.len() && i_l < long.len() {
        while short[i_s] < long[i_l] {
            i_s += 1;
        }
        if i_s == short.len() {
            break;
        }
        while  short[i_s] > long[i_l] {
            i_l += 1;
        }
        if i_l == long.len() {
            break;
        }
        if short[i_s] == long[i_l] {
            results.push(short[i_s].clone());
            i_s += 1;
            i_l += 1;
        }
    }
    short.pop();
    long.pop();
    results.pop();
    println!("{:?}",results);
    results
}
