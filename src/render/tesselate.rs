// MIT/Apache2 License

use super::{double_to_fixed, fixed_to_double};
use crate::auto::render::{Fixed, Linefix, Pointfix, Trapezoid};
use alloc::{vec, vec::Vec};
use core::{cmp::Ordering, iter::FusedIterator};

#[inline]
pub fn tesselate_shape<I: IntoIterator<Item = Pointfix>>(i: I) -> Vec<Trapezoid> {
    let traps = edges_to_trapezoids(PolyEdges {
        inner: Some(i.into_iter()),
        firstx: 0,
        firsty: 0,
        prevx: 0,
        prevy: 0,
        is_first_point: true,
    });
    log::debug!("Tesselated shape into {} trapezoids", traps.len());
    traps
}

#[derive(Copy, Clone)]
struct Edge {
    x1: Fixed,
    y1: Fixed,
    x2: Fixed,
    y2: Fixed,
    //    clockwise: bool,
    current_x: Fixed,
}

impl Edge {
    #[inline]
    fn compute_x(&mut self, y: Fixed) {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;
        let ex = (y - self.y1) as f64 * dx as f64;

        self.current_x = self.x1 + (ex / (dy as f64)) as Fixed;
    }

    #[inline]
    fn intersection(self, other: Self) -> Fixed {
        double_to_fixed(
            (other.x_intercept() - self.x_intercept())
                / (self.inverse_slope() - other.inverse_slope()),
        )
    }

    #[inline]
    fn inverse_slope(self) -> f64 {
        fixed_to_double(self.x2 - self.x1) / fixed_to_double(self.y2 - self.y1)
    }

    #[inline]
    fn x_intercept(self) -> f64 {
        fixed_to_double(self.x1) - (self.inverse_slope() * fixed_to_double(self.y1))
    }
}

struct PolyEdges<I> {
    inner: Option<I>,
    firstx: Fixed,
    firsty: Fixed,
    prevx: Fixed,
    prevy: Fixed,
    is_first_point: bool,
}

impl<I> Iterator for PolyEdges<I>
where
    I: Iterator<Item = Pointfix>,
{
    type Item = Edge;

    #[inline]
    fn next(&mut self) -> Option<Edge> {
        loop {
            match self.inner.take() {
                None => return None,
                Some(mut inner) => {
                    // if inner produces None, use the last edge. otherwise, use the edge returned
                    let Pointfix { x, y } = match inner.next() {
                        Some(pt) => {
                            self.inner = Some(inner);
                            pt
                        }
                        None => Pointfix {
                            x: self.firstx,
                            y: self.firsty,
                        },
                    };

                    if self.is_first_point {
                        self.is_first_point = false;
                        self.firstx = x;
                        self.firsty = y;
                    } else {
                        if self.prevy < y {
                            return Some(Edge {
                                x1: self.prevx,
                                y1: self.prevy,
                                x2: x,
                                y2: y,
                                //                                clockwise: true,
                                current_x: 0,
                            });
                        } else if self.prevy > y {
                            return Some(Edge {
                                x1: x,
                                y1: y,
                                x2: self.prevx,
                                y2: self.prevy,
                                //                                clockwise: false,
                                current_x: 0,
                            });
                        }

                        // note: horizontal edges are dropped
                    }

                    self.prevx = x;
                    self.prevy = y;
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            None => (0, Some(0)),
            Some(inner) => inner.size_hint(),
        }
    }
}

impl<I> FusedIterator for PolyEdges<I> where I: Iterator<Item = Pointfix> {}
impl<I: ExactSizeIterator> ExactSizeIterator for PolyEdges<I> where I: Iterator<Item = Pointfix> {}

#[inline]
fn edges_to_trapezoids<I: IntoIterator<Item = Edge>>(i: I) -> Vec<Trapezoid> {
    // put the edges into a vec and then sort them
    let mut edges: Vec<Edge> = i.into_iter().collect();

    if edges.is_empty() {
        return vec![];
    }

    edges.sort_unstable_by_key(|edge| edge.y1);
    let edges_len = edges.len();

    // active and inactive lists
    let mut active: Vec<Edge> = Vec::with_capacity(edges.len());
    let mut inactive: usize = 0;

    // trapezoids
    let mut trapezoids: Vec<Trapezoid> = Vec::with_capacity(edges_len * edges_len);

    let mut y = edges[0].y1;
    let mut next_y;

    while !active.is_empty() || !edges.is_empty() {
        log::trace!("Tesselation loop, y = {}", y);

        while !edges.is_empty() {
            let e = edges.remove(0);
            if e.y1 > y {
                break;
            }

            // put e into the active list
            active.push(e);
            inactive += 1;
        }

        active.iter_mut().for_each(|e| e.compute_x(y));

        // sort the active list
        active.sort_unstable_by(|e1, e2| {
            if e2.current_x < e1.current_x || (e2.current_x == e1.current_x && e2.x2 < e1.x2) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // find the inflection point
        next_y = active
            .iter()
            .map(|e| e.y2)
            .chain(active.windows(2).filter_map(|edges| {
                let e1 = edges[0];
                let e2 = edges[1];
                if e1.x2 > e2.x2 {
                    Some(e1.intersection(e2) + 1)
                } else {
                    None
                }
            }))
            .min()
            .unwrap();

        if let Some(e) = edges.get(0) {
            if e.y1 < next_y {
                next_y = edges[inactive].y1;
            }
        }

        // generate some trapezoids
        trapezoids.extend(active.chunks_exact(2).map(|edges| {
            let e1 = edges[0];
            let e2 = edges[1];
            Trapezoid {
                top: y,
                bottom: next_y,
                left: Linefix {
                    p1: Pointfix { x: e1.x1, y: e1.y1 },
                    p2: Pointfix { x: e1.x2, y: e1.y2 },
                },
                right: Linefix {
                    p1: Pointfix { x: e2.x1, y: e2.y1 },
                    p2: Pointfix { x: e2.x2, y: e2.y2 },
                },
            }
        }));

        y = next_y;

        active.retain(|e| e.y2 > y);
    }

    trapezoids
}
