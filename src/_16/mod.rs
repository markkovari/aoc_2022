use anyhow::Result;
use ndarray::Array3;
use std::collections::HashMap;

pub fn solve_16() -> Result<(usize, usize)> {
    let input = include_str!("../../inputs/16/input.data");

    let mut valves = Vec::<(&str, u16, Vec<&str>)>::new();
    for line in input.trim().split('\n') {
        let (valve, flow, _, tunnels) = sscanf::sscanf!(
            line,
            "Valve {str} has flow rate={u16}; {str:/tunnels? leads? to valves?/} {str}"
        )
        .unwrap();
        let tunnels = tunnels.split(", ").collect::<Vec<_>>();
        valves.push((valve, flow, tunnels));
    }

    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let m = valves.iter().filter(|v| v.1 > 0).count();
    let n = valves.len();
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];
    for v in valves.iter() {
        let i = lab2idx[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(lab2idx[w]);
        }
    }
    let aa = lab2idx["AA"];

    let mm = 1 << m;
    let mut opt = Array3::<u16>::zeros([30, n, mm]);
    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    let res = opt[(29, aa, mm - 1)];

    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(opt[(25, aa, x)] + opt[(25, aa, y)]);
    }

    Ok((res.into(), best.into()))
}
