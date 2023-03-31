use std::collections::{HashSet, VecDeque};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyInt};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct State {
    robots_ore: u32,
    robots_clay: u32,
    robots_obsidian: u32,
    robots_geode: u32,
    resources_ore: u32,
    resources_clay: u32,
    resources_obsidian: u32,
    resources_geode: u32,
    timer: u32,
}

impl State {
    fn hash(&self) -> u128 {
        self.robots_ore as u128
            + self.robots_clay as u128 * 100
            + self.robots_obsidian as u128 * 10000
            + self.robots_geode as u128 * 1000000
            + self.resources_ore as u128 * 100000000
            + self.resources_clay as u128 * 10000000000
            + self.resources_obsidian as u128 * 1000000000000
            + self.resources_geode as u128 * 100000000000000
            + self.timer as u128 * 10000000000000000
    }
}

/// Do stuff
#[pyfunction]
fn simulation(bp: &PyDict, py_time_limit: &PyInt) -> PyResult<u32> {
    let time_limit: u32 = py_time_limit.extract()?;
    let tmp_ore_ore: u32 = bp
        .get_item("ore")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let tmp_clay_ore: u32 = bp
        .get_item("clay")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let tmp_obsidian_ore: u32 = bp
        .get_item("obsidian")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let tmp_obsidian_clay: u32 = bp
        .get_item("obsidian")
        .unwrap()
        .get_item("clay")
        .unwrap()
        .extract()
        .unwrap();
    let tmp_geode_ore: u32 = bp
        .get_item("geode")
        .unwrap()
        .get_item("ore")
        .unwrap()
        .extract()
        .unwrap();
    let tmp_geode_obsidian: u32 = bp
        .get_item("geode")
        .unwrap()
        .get_item("obsidian")
        .unwrap()
        .extract()
        .unwrap();

    // max_ore = max([x['ore'] for x in bp.values()])
    let max_ore: u32 = bp
        .values()
        .iter()
        .map(|x| x.get_item("ore").unwrap().extract().or_else(|_| Ok(0u32)))
        .flat_map(|e: Result<u32, ()>| e)
        .max()
        .unwrap();
    // max_clay = max([x.get('clay',0) for x in bp.values()])
    let max_clay: u32 = bp
        .values()
        .iter()
        .map(|x| {
            if let Ok(clay) = x.get_item("clay") {
                clay.extract()
            } else {
                Ok(0)
            }
        })
        .flat_map(|e| e)
        .max()
        .unwrap();
    // max_obsidian = max([x.get('obsidian',0) for x in bp.values()])
    let max_obsidian: u32 = bp
        .values()
        .iter()
        .map(|x| {
            if let Ok(obsidian) = x.get_item("obsidian") {
                obsidian.extract()
            } else {
                Ok(0)
            }
        })
        .flat_map(|e| e)
        .max()
        .unwrap();

    // dbg!(&max_ore, &max_clay, &max_obsidian);

    // # robots, resources, timer
    // stack = [(1,0,0,0,0,0,0,0,0)]
    let mut stack: VecDeque<State> = VecDeque::new();
    stack.push_back(State {
        robots_ore: 1u32,
        robots_clay: 0u32,
        robots_obsidian: 0u32,
        robots_geode: 0u32,
        resources_ore: 0u32,
        resources_clay: 0u32,
        resources_obsidian: 0u32,
        resources_geode: 0u32,
        timer: 0u32,
    });

    // seen = set()
    let mut seen: HashSet<u128> = HashSet::new();

    // ret_max = 0
    let mut ret_max = 0u32;

    let mut gen = 0u32;
    let mut geodes_at_gen = 0u32;
    // while stack:
    while let Some(state) = stack.pop_front() {
        //     # print(len(stack), end=" ")
        // dbg!(stack.len());

        //     (robots_ore, robots_clay, robots_obsidian, robots_geode,
        //     resources_ore, resources_clay, resources_obsidian, resources_geode,
        //         timer) = stack.pop()

        //     state = (robots_ore, robots_clay, robots_obsidian, robots_geode,
        //         resources_ore, resources_clay, resources_obsidian, resources_geode,
        //         timer)

        //     if state in seen:
        //         continue
        //     seen.add(state)
        if seen.contains(&state.hash()) {
            continue;
        }
        seen.insert(state.hash());

        //     if timer == time_limit:
        //         if resources_geode>ret_max:
        //             ret_max = resources_geode
        //             # print(f"geode {resources_geode}")
        if state.timer == time_limit {
            if state.resources_geode > ret_max {
                ret_max = state.resources_geode;
            }
            continue;
        }

        gen = if state.timer > gen {
            state.timer
        } else {
            gen
        };

        geodes_at_gen = if state.robots_geode > geodes_at_gen {
            state.robots_geode
        } else {
            geodes_at_gen
        };

        if state.timer == gen && state.robots_geode < geodes_at_gen {
            continue
        }

        //     else:
        //         # no robot build, just storage
        //         stack.append((robots_ore, robots_clay, robots_obsidian, robots_geode,
        //             robots_ore + resources_ore, robots_clay + resources_clay,
        //             robots_obsidian + resources_obsidian, robots_geode + resources_geode,
        //         timer+1))
        stack.push_back(State {
            robots_ore: state.robots_ore,
            robots_clay: state.robots_clay,
            robots_obsidian: state.robots_obsidian,
            robots_geode: state.robots_geode,
            resources_ore: state.resources_ore + state.robots_ore,
            resources_clay: state.resources_clay + state.robots_clay,
            resources_obsidian: state.resources_obsidian + state.robots_obsidian,
            resources_geode: state.resources_geode + state.robots_geode,
            timer: state.timer + 1,
        });
        //         # robot build
        //         if bp['ore']['ore'] <= resources_ore and robots_ore < max_ore:
        //             stack.append((robots_ore+1, robots_clay, robots_obsidian, robots_geode,
        //                 robots_ore + resources_ore-bp['ore']['ore'], robots_clay + resources_clay, robots_obsidian + resources_obsidian, robots_geode + resources_geode,
        //             timer+1))
        if tmp_ore_ore <= state.resources_ore && state.robots_ore < max_ore {
            stack.push_back(State {
                robots_ore: state.robots_ore + 1,
                robots_clay: state.robots_clay,
                robots_obsidian: state.robots_obsidian,
                robots_geode: state.robots_geode,
                resources_ore: state.resources_ore + state.robots_ore - tmp_ore_ore,
                resources_clay: state.resources_clay + state.robots_clay,
                resources_obsidian: state.resources_obsidian + state.robots_obsidian,
                resources_geode: state.resources_geode + state.robots_geode,
                timer: state.timer + 1,
            });
        }
        //         if bp['clay']['ore'] <= resources_ore and robots_clay < max_clay:
        //             stack.append((robots_ore, robots_clay+1, robots_obsidian, robots_geode,
        //                 robots_ore+resources_ore-bp['clay']['ore'], robots_clay + resources_clay, robots_obsidian + resources_obsidian, robots_geode + resources_geode,
        //             timer+1))
        if tmp_clay_ore <= state.resources_ore && state.robots_clay < max_clay {
            stack.push_back(State {
                robots_ore: state.robots_ore,
                robots_clay: state.robots_clay + 1,
                robots_obsidian: state.robots_obsidian,
                robots_geode: state.robots_geode,
                resources_ore: state.resources_ore + state.robots_ore - tmp_clay_ore,
                resources_clay: state.resources_clay + state.robots_clay,
                resources_obsidian: state.resources_obsidian + state.robots_obsidian,
                resources_geode: state.resources_geode + state.robots_geode,
                timer: state.timer + 1,
            });
        }

        //         if bp['obsidian']['ore'] <= resources_ore and bp['obsidian']['clay'] <= resources_clay and robots_clay<max_clay:
        //             stack.append((robots_ore, robots_clay, robots_obsidian+1, robots_geode,
        //                 robots_ore + resources_ore-bp['obsidian']['ore'], robots_clay + resources_clay-bp['obsidian']['clay'], robots_obsidian + resources_obsidian, robots_geode + resources_geode,
        //             timer+1))
        if tmp_obsidian_ore <= state.resources_ore
            && tmp_obsidian_clay <= state.resources_clay
            && state.robots_obsidian < max_obsidian
        {
            stack.push_back(State {
                robots_ore: state.robots_ore,
                robots_clay: state.robots_clay,
                robots_obsidian: state.robots_obsidian + 1,
                robots_geode: state.robots_geode,
                resources_ore: state.resources_ore + state.robots_ore - tmp_obsidian_ore,
                resources_clay: state.resources_clay + state.robots_clay - tmp_obsidian_clay,
                resources_obsidian: state.resources_obsidian + state.robots_obsidian,
                resources_geode: state.resources_geode + state.robots_geode,
                timer: state.timer + 1,
            });
        }

        //         if bp['geode']['ore'] <= resources_ore and bp['geode']['obsidian'] <= resources_obsidian:
        //             stack.append((robots_ore, robots_clay, robots_obsidian, robots_geode+1,
        //                 robots_ore + resources_ore-bp['geode']['ore'], robots_clay + resources_clay, robots_obsidian + resources_obsidian-bp['geode']['obsidian'], robots_geode + resources_geode,
        //             timer+1))
        if tmp_geode_ore <= state.resources_ore && tmp_geode_obsidian <= state.resources_obsidian {
            stack.push_back(State {
                robots_ore: state.robots_ore,
                robots_clay: state.robots_clay,
                robots_obsidian: state.robots_obsidian,
                robots_geode: state.robots_geode + 1,
                resources_ore: state.resources_ore + state.robots_ore - tmp_geode_ore,
                resources_clay: state.resources_clay + state.robots_clay,
                resources_obsidian: state.resources_obsidian + state.robots_obsidian
                    - tmp_geode_obsidian,
                resources_geode: state.resources_geode + state.robots_geode,
                timer: state.timer + 1,
            });
        }
    }

    // return ret_max
    Ok(ret_max)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_tests(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulation, m)?)?;
    Ok(())
}
