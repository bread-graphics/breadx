// MIT/Apache2 License

use super::{double_to_fixed, fixed_to_double};
use crate::{
    auto::render::{Fixed, Linefix, Pointfix, Trapezoid, Triangle},
    log_trace,
};
use alloc::{vec, vec::Vec};
use core::{
    cmp::Ordering,
    iter::{Fuse, FusedIterator},
    mem,
};

/// Tesselate a shape into a set of triangles. This function takes an iterator of points that represent a closed
/// shape, and returns a lazy iterator over the triangles.
///
/// # Notes
///
/// The given iterator is collected into a `Vec` before the real iterator is returned, as this algorithm relies
/// on a sorted list of edges before it begins generating. It is impossible to sort data as in sorted order from
/// the iterator proper (at least, until dtolnay released his newest crate), so keep this in mind for iterators
/// that have side effects.
///
/// Also note that it is more efficient to use fused iterators with this function.
#[inline]
pub fn tesselate_shape<I: IntoIterator<Item = Pointfix>>(i: I) -> impl Iterator<Item = Triangle> {
    // Note: it is more efficient to ignore horizontal edges
    // Note 2: The "trapezoids" request is given as deprecated, so we use triangles instead
    edges_to_trapezoids(
        PointsToEdges {
            inner: i.into_iter().fuse(),
            first: None,
            last: None,
        }
        .filter(|e| e.y1 != e.y2),
    )
    .flat_map(trapezoid_to_triangles)
}

#[inline]
fn trapezoid_to_triangles(t: Trapezoid) -> Twice<Triangle> {
    /// Tell if two `Fixed` values are equal within a margin of error.
    #[inline]
    fn eq_within_margin_of_error(lhs: Fixed, rhs: Fixed) -> bool {
        const MARGIN_OF_ERROR: Fixed = 0xFF;

        (lhs - rhs).abs() <= MARGIN_OF_ERROR
    }

    let Trapezoid {
        top,
        bottom,
        left:
            Linefix {
                p1: Pointfix { x: lx1, y: ly1 },
                p2: Pointfix { x: lx2, y: ly2 },
            },
        right:
            Linefix {
                p1: Pointfix { x: rx1, y: ry1 },
                p2: Pointfix { x: rx2, y: ry2 },
            },
    } = t;

    // figure out x and y intercept for the two lines
    let (left_top_x, left_bottom_x) = (
        x_intercept_for_y(lx1, ly1, lx2, ly2, top),
        x_intercept_for_y(lx1, ly1, lx2, ly2, bottom),
    );
    let (right_top_x, right_bottom_x) = (
        x_intercept_for_y(rx1, ry1, rx2, ry2, top),
        x_intercept_for_y(rx1, ry1, rx2, ry2, bottom),
    );

    log_trace!(
        "left_top_x: {}, left_bottom_x: {}",
        left_top_x,
        left_bottom_x
    );
    log_trace!(
        "right_top_x: {}, right_bottom_x: {}",
        right_top_x,
        right_bottom_x
    );

    let top_eq = eq_within_margin_of_error(left_top_x, right_top_x);
    let bottom_eq = eq_within_margin_of_error(left_bottom_x, right_bottom_x);

    if top_eq && bottom_eq {
        // we just have a straight line. no need to render anything.
        log_trace!("Found a straight line");
        return Twice::empty();
    }

    // if two of the x intercepts are the same, this is a triangle
    if top_eq {
        log_trace!("Found a top-facing triangle");

        return Twice::once(Triangle {
            p1: Pointfix {
                x: left_top_x,
                y: top,
            },
            p2: Pointfix {
                x: right_bottom_x,
                y: bottom,
            },
            p3: Pointfix {
                x: left_bottom_x,
                y: bottom,
            },
        });
    }

    if bottom_eq {
        log_trace!("Found a bottom-facing triangle");

        return Twice::once(Triangle {
            p1: Pointfix {
                x: left_top_x,
                y: top,
            },
            p2: Pointfix {
                x: right_top_x,
                y: top,
            },
            p3: Pointfix {
                x: left_bottom_x,
                y: bottom,
            },
        });
    }

    // otherwise, we need two triangles to express the trapezoid
    log_trace!("Found a trapezoid that needs to be divided into two triangles");
    Twice::twice(
        Triangle {
            p1: Pointfix {
                x: left_top_x,
                y: top,
            },
            p2: Pointfix {
                x: right_top_x,
                y: top,
            },
            p3: Pointfix {
                x: left_bottom_x,
                y: bottom,
            },
        },
        Triangle {
            p1: Pointfix {
                x: right_top_x,
                y: top,
            },
            p2: Pointfix {
                x: right_bottom_x,
                y: bottom,
            },
            p3: Pointfix {
                x: left_bottom_x,
                y: bottom,
            },
        },
    )
}

/// One or two elements in an iterator.
struct Twice<T> {
    first: Option<T>,
    last: Option<T>,
}

impl<T> Twice<T> {
    #[inline]
    fn empty() -> Twice<T> {
        Twice {
            first: None,
            last: None,
        }
    }

    #[inline]
    fn once(t: T) -> Twice<T> {
        Twice {
            first: Some(t),
            last: None,
        }
    }

    #[inline]
    fn twice(t1: T, t2: T) -> Twice<T> {
        Twice {
            first: Some(t1),
            last: Some(t2),
        }
    }
}

impl<T> Iterator for Twice<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        match self.first.take() {
            Some(first) => Some(first),
            None => self.last.take(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.first.is_none() as usize + self.last.is_none() as usize;
        (sz, Some(sz))
    }

    #[inline]
    fn fold<B, F: FnMut(B, T) -> B>(mut self, init: B, mut f: F) -> B {
        let mut accum = init;

        if let Some(first) = self.first.take() {
            accum = f(accum, first);
        }

        if let Some(last) = self.last.take() {
            accum = f(accum, last);
        }

        accum
    }
}

impl<T> FusedIterator for Twice<T> {}
impl<T> ExactSizeIterator for Twice<T> {}

#[inline]
fn edges_to_trapezoids<I: IntoIterator<Item = Edge>>(i: I) -> Trapezoids {
    let mut edges: Vec<Edge> = i.into_iter().collect();
    if edges.is_empty() {
        // yields nothing
        return Trapezoids {
            y: 0,
            last_y: 0,
            active: vec![],
            inactive: vec![],
            last_trapezoid: core::usize::MAX,
        };
    }

    // sort and reverse "edges" so it's easy enough to pop edges off
    edges.sort_unstable_by(|e1, e2| match e1.y1.cmp(&e2.y1) {
        Ordering::Equal => e1.x1.cmp(&e2.x1),
        o => o,
    });
    edges.reverse();

    #[cfg(debug_assertions)]
    log::trace!("Edges are: {:?}", &edges);

    Trapezoids {
        y: edges.last().unwrap().y1,
        last_y: 0,
        active: Vec::with_capacity(edges.len()),
        inactive: edges,
        last_trapezoid: core::usize::MAX,
    }
}

/// Given a set of edges, this iterates over them and produces trapezoids.
struct Trapezoids {
    // "active" edges. any edge in this list should intersect with the current scanline.
    active: Vec<Edge>,
    // "inactive" edges. these edges have yet to be made active.
    //
    // this list is sorted in reverse y1 order. it should be sorted in non-reverse order, but sorting it in
    // reverse order allows us to just pop off newly active edges, which should be a performance win
    inactive: Vec<Edge>,
    // the current scanline we're operating on.
    y: Fixed,
    // the former scanline we're operating on. combined with the current scanline, this forms the top and bottom
    // of any potential trapezoids
    last_y: Fixed,
    // keeps track of the index of the active lines where we are.
    last_trapezoid: usize,
}

impl Trapezoids {
    /// Populates `active` with the list of active edges, and sets `last_trapezoid` to zero. Returns false if it
    /// was unable to do anything.
    #[inline]
    fn populate(&mut self) -> bool {
        log::debug!(
            "Running populate(). There are {} active elements and {} inactive elements",
            self.active.len(),
            self.inactive.len()
        );
        #[cfg(debug_assertions)]
        log::trace!(
            "Creating trapezoids at y: {} ({})",
            fixed_to_double(self.y),
            self.y
        );

        // if both the active and inactive lists are empty, this iterator should stop
        if self.active.is_empty() && self.inactive.is_empty() {
            return false;
        }

        let y = self.y;

        // first, move any qualifying edges into the active group
        while !self.inactive.is_empty() {
            let edge = self.inactive.last().unwrap();
            if edge.y1 > y {
                break;
            }

            // edge qualifies; move it into the active group
            self.active.push(self.inactive.pop().unwrap());
        }

        // compute the x-interception along the current y
        self.active
            .iter_mut()
            .for_each(move |edge| edge.compute_x(y));

        #[cfg(debug_assertions)]
        log::trace!("Active edges are: {:#?}", &self.active);

        // sort the active list by current x intercept
        // likely to be fast since list is close to already sorted and is, for most polygons, smaller than
        // 20 elements
        self.active
            .sort_unstable_by(|e1, e2| match e1.current_x.cmp(&e2.current_x) {
                Ordering::Equal => e1.x2.cmp(&e2.x2),
                o => o,
            });

        // find the next y-level
        let next_y = self
            .active
            .iter()
            .map(|e| e.y2)
            .chain(self.active.windows(2).filter_map(|es| {
                if es[0].x2 > es[1].x2 {
                    Some(es[0].compute_intersect(es[1]) + 1)
                } else {
                    None
                }
            }))
            .chain(self.inactive.last().map(|e| e.y1))
            .min()
            .expect("Iteration should've ended by now");

        // set up variables for trapezoid generation
        self.last_y = self.y;
        self.y = next_y;
        self.last_trapezoid = 0;

        true
    }

    // if we have any trapezoids left, this pops one
    #[inline]
    fn pop_trapezoid(&mut self) -> Trapezoid {
        // take the first two active edges we haven't processed already and use them to create trapezoids
        let last_trapezoid = self.last_trapezoid;
        self.last_trapezoid += 2;
        let e1 = self.active[last_trapezoid];
        let e2 = self.active[last_trapezoid + 1];

        return Trapezoid {
            top: self.last_y,
            bottom: self.y,
            left: Linefix {
                p1: Pointfix { x: e1.x1, y: e1.y1 },
                p2: Pointfix { x: e1.x2, y: e1.y2 },
            },
            right: Linefix {
                p1: Pointfix { x: e2.x1, y: e2.y1 },
                p2: Pointfix { x: e2.x2, y: e2.y2 },
            },
        };
    }
}

impl Iterator for Trapezoids {
    type Item = Trapezoid;

    #[inline]
    fn next(&mut self) -> Option<Trapezoid> {
        loop {
            // if last_trapezoid is greater than the length of the list of active edges, we need to generate more
            // trapezoids
            //
            // the +1 here ensures we don't end up with odd edges
            if self.last_trapezoid.saturating_add(1) >= self.active.len() {
                // clean out now-inactive edges
                let y = self.y;
                self.active.retain(move |e| e.y2 > y);

                // try to generate more edges
                if !self.populate() {
                    // we could not generate more active edges
                    return None;
                }
            } else {
                return Some(self.pop_trapezoid());
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some((self.active.len() + self.inactive.len()).pow(2)))
    }

    // Implement some adaptors that may make this function more efficient.
    // Note that this iterator is put right into a flat_map(), which only implements custom fold() and try_fold()
    // that call the underlying iterator. Normally I'd implement nth() here, but since FlatMap doesn't forward
    // it it would not be used.
    //
    // TODO: implement try_fold() once the Try trait becomes stable

    #[inline]
    fn fold<B, F: FnMut(B, Trapezoid) -> B>(mut self, init: B, mut f: F) -> B {
        let mut accum = init;

        loop {
            // if we have any cached trapezoids, pop them and use them
            while self.last_trapezoid.saturating_add(1) < self.active.len() {
                accum = f(accum, self.pop_trapezoid());
            }

            // generate moar
            let y = self.y;
            self.active.retain(move |e| e.y2 > y);
            if !self.populate() {
                return accum;
            }
        }
    }
}

impl FusedIterator for Trapezoids {}

/// Iterate over a set of points, transforming them into a set of edges.
struct PointsToEdges<I> {
    inner: Fuse<I>,
    first: Option<Pointfix>,
    last: Option<Pointfix>,
}

impl<I: Iterator<Item = Pointfix>> Iterator for PointsToEdges<I> {
    type Item = Edge;

    #[inline]
    fn next(&mut self) -> Option<Edge> {
        loop {
            match self.inner.next() {
                Some(pt) => {
                    // we have a point. if this is the first point, store it in "first" and "last". otherwise,
                    // just store it in "last" and return the combination of this point and the former last point
                    match mem::replace(&mut self.last, Some(pt)) {
                        None => {
                            self.first = Some(pt);
                        }
                        Some(last) => {
                            return Some(Edge::from_points(last, pt));
                        }
                    }
                }
                None => {
                    // if "first" is none, or if "first" and "last" are equal, return None
                    // otherwise, combine "first" and "last
                    match (self.first.take(), self.last.take()) {
                        (Some(first), Some(last)) => {
                            if first == last {
                                return None;
                            } else {
                                return Some(Edge::from_points(last, first));
                            }
                        }
                        _ => return None,
                    }
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        #[inline]
        fn cvt_size(s: usize) -> usize {
            match s {
                0 | 1 => 0,
                s => s,
            }
        }

        let (lo, hi) = self.inner.size_hint();
        (cvt_size(lo), hi.map(cvt_size))
    }
}

impl<I: Iterator<Item = Pointfix>> FusedIterator for PointsToEdges<I> {}
impl<I: Iterator<Item = Pointfix> + ExactSizeIterator> ExactSizeIterator for PointsToEdges<I> {}

/// An edge between two points.
#[derive(Debug, Copy, Clone)]
struct Edge {
    x1: Fixed,
    y1: Fixed,
    x2: Fixed,
    y2: Fixed,
    current_x: Fixed,
}

impl Edge {
    #[inline]
    fn from_points(p1: Pointfix, p2: Pointfix) -> Edge {
        if p1.y < p2.y {
            Edge {
                x1: p1.x,
                y1: p1.y,
                x2: p2.x,
                y2: p2.y,
                current_x: 0,
            }
        } else {
            Edge {
                x1: p2.x,
                y1: p2.y,
                x2: p1.x,
                y2: p1.y,
                current_x: 0,
            }
        }
    }
}

impl Edge {
    #[inline]
    fn inverse_slope(self) -> f64 {
        fixed_to_double(self.x2 - self.x1) / fixed_to_double(self.y2 - self.y1)
    }

    #[inline]
    fn x_intercept(self) -> f64 {
        fixed_to_double(self.x1) - (self.inverse_slope() * fixed_to_double(self.y1))
    }

    #[inline]
    fn compute_x(&mut self, y: Fixed) {
        self.current_x = x_intercept_for_y(self.x1, self.y1, self.x2, self.y2, y);
    }

    #[inline]
    fn compute_intersect(self, other: Edge) -> Fixed {
        let m1 = self.inverse_slope();
        let b1 = self.x_intercept();
        let m2 = other.inverse_slope();
        let b2 = other.x_intercept();
        double_to_fixed((b2 - b1) / (m2 - m1))
    }
}

/// Get the intercept of a line containing the points (`x1`, `y1`) and (`x2`, `y2`) and the horizontal line at
/// `y`.
#[inline]
fn x_intercept_for_y(x1: Fixed, y1: Fixed, x2: Fixed, y2: Fixed, y: Fixed) -> Fixed {
    // fast paths
    if y1 == y {
        return x1;
    }
    if y2 == y {
        return x2;
    }

    let dx = x2 - x1;
    let ex = (y - y1) as f64 * (dx as f64);
    let dy = y2 - y1;
    x1 + ((ex / dy as f64) as Fixed)
}

#[test]
fn test_intercept_function() {
    assert_eq!(
        0,
        x_intercept_for_y(-1 << 16, -1 << 16, 1 << 16, 1 << 16, 0)
    );
    assert_eq!(
        1 << 16,
        x_intercept_for_y(-1 << 16, -1 << 16, 1 << 16, 1 << 16, 1 << 16)
    );
    assert_eq!(
        3 << 16,
        x_intercept_for_y(-1 << 16, -1 << 16, 1 << 16, 1 << 16, 3 << 16)
    );
    assert_eq!(2 << 16, x_intercept_for_y(0, 6 << 16, 1 << 16, 3 << 16, 0));
}

#[test]
fn rectangle_test() {
    // the area of the resulting triangles should be equal to the area of the rectangle
    let left = 50i32 << 16;
    let right = 250i32 << 16;
    let top = 100i32 << 16;
    let bottom = 200i32 << 16;

    let area = fixed_to_double(right - left) * fixed_to_double(bottom - top);

    let triangles: Vec<_> = tesselate_shape(vec![
        Pointfix { x: left, y: top },
        Pointfix { x: right, y: top },
        Pointfix {
            x: right,
            y: bottom,
        },
        Pointfix { x: left, y: bottom },
    ])
    .collect();

    assert!(!triangles.is_empty());

    approx::assert_abs_diff_eq!(
        triangles.into_iter().map(area_of_triangle).sum::<f64>(),
        area
    );
}

/// Helper function to get the area of a triangle.
#[cfg(test)]
fn area_of_triangle(triangle: Triangle) -> f64 {
    let Triangle {
        p1: Pointfix { x: x1, y: y1 },
        p2: Pointfix { x: x2, y: y2 },
        p3: Pointfix { x: x3, y: y3 },
    } = triangle;

    let y1 = fixed_to_double(y1);
    let y2 = fixed_to_double(y2);
    let y3 = fixed_to_double(y3);
    let a = fixed_to_double(x1) * (y2 - y3);
    let b = fixed_to_double(x2) * (y3 - y1);
    let c = fixed_to_double(x3) * (y1 - y2);

    (a + b + c).abs() / 2.0
}
