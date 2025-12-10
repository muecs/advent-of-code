//! Collection of shared utility functions

/// Greatest Common Divisor (GCD)
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Default + PartialEq,
    T: std::ops::Rem<Output = T>,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least Common Multiple (LCM)
pub fn lcm<T>(vals: &[T]) -> T
where
    T: Copy + Default + PartialEq,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Rem<Output = T>,
{
    if vals.len() == 1 {
        return vals[0];
    }
    let a = vals[0];
    let b = lcm(&vals[1..]);
    a * b / gcd(a, b)
}

/// Solves a system of linear equations for non-negative integer solutions, minimizing the sum of variables.
/// Uses Gaussian elimination and recursive search for free variables.
pub fn solve_min_integer_sum(mut matrix: Vec<Vec<f64>>) -> usize {
    let num_eqs = matrix.len();
    let num_vars = matrix[0].len() - 1;

    let mut pivot_row = 0;
    let mut col = 0;
    let mut pivots = Vec::new();

    // Gaussian elimination to Reduced Row Echelon Form (RREF)
    // https://en.wikipedia.org/wiki/Row_echelon_form#Reduced_row_echelon_form
    while pivot_row < num_eqs && col < num_vars {
        let mut max_row = pivot_row;
        for i in pivot_row + 1..num_eqs {
            if matrix[i][col].abs() > matrix[max_row][col].abs() {
                max_row = i;
            }
        }

        if matrix[max_row][col].abs() < 1e-9 {
            col += 1;
            continue;
        }

        matrix.swap(pivot_row, max_row);
        pivots.push((col, pivot_row));

        let pivot_val = matrix[pivot_row][col];
        for j in col..=num_vars {
            matrix[pivot_row][j] /= pivot_val;
        }

        for i in 0..num_eqs {
            if i != pivot_row {
                let factor = matrix[i][col];
                for j in col..=num_vars {
                    matrix[i][j] -= factor * matrix[pivot_row][j];
                }
            }
        }

        pivot_row += 1;
        col += 1;
    }

    // Check for inconsistencies (0 = non-zero)
    for i in pivot_row..num_eqs {
        if matrix[i][num_vars].abs() > 1e-9 {
            return usize::MAX;
        }
    }

    let pivot_cols: std::collections::HashSet<_> = pivots.iter().map(|&(c, _)| c).collect();
    let free_vars: Vec<_> = (0..num_vars).filter(|c| !pivot_cols.contains(c)).collect();

    solve_recursive(&matrix, &pivots, &free_vars, &mut Vec::new())
}

fn solve_recursive(
    matrix: &Vec<Vec<f64>>,
    pivots: &Vec<(usize, usize)>,
    free_vars: &[usize],
    free_vals: &mut Vec<f64>,
) -> usize {
    if free_vals.len() == free_vars.len() {
        let mut current_presses = 0;
        for &val in free_vals.iter() {
            current_presses += val as usize;
        }

        for &(_p_col, p_row) in pivots {
            let mut val = matrix[p_row][matrix[0].len() - 1];
            for (i, &f_col) in free_vars.iter().enumerate() {
                val -= matrix[p_row][f_col] * free_vals[i];
            }

            if val < -1e-9 || (val.round() - val).abs() > 1e-9 {
                return usize::MAX;
            }
            current_presses += val.round() as usize;
        }
        return current_presses;
    }

    // Optimization for the last free variable
    if free_vals.len() == free_vars.len() - 1 {
        let mut min_val = 0.0f64;
        let mut max_val = f64::INFINITY;

        let f_col = free_vars[free_vals.len()];

        for &(_p_col, p_row) in pivots {
            let mut rhs = matrix[p_row][matrix[0].len() - 1];
            for (i, &prev_f_col) in free_vars.iter().enumerate().take(free_vals.len()) {
                rhs -= matrix[p_row][prev_f_col] * free_vals[i];
            }

            let coeff = matrix[p_row][f_col];

            if coeff.abs() < 1e-9 {
                if rhs < -1e-9 {
                    return usize::MAX;
                }
            } else if coeff > 0.0 {
                max_val = max_val.min(rhs / coeff);
            } else {
                min_val = min_val.max(rhs / coeff);
            }
        }

        let start = (min_val - 1e-9).ceil() as usize;
        let end = if max_val.is_infinite() {
            usize::MAX
        } else {
            (max_val + 1e-9).floor() as usize
        };

        if start > end {
            return usize::MAX;
        }

        let mut min_res = usize::MAX;
        let limit = 1000;

        let actual_end = if end > start + limit {
            start + limit
        } else {
            end
        };

        for val in start..=actual_end {
            free_vals.push(val as f64);
            let res = solve_recursive(matrix, pivots, free_vars, free_vals);
            min_res = min_res.min(res);
            free_vals.pop();
        }
        return min_res;
    }

    let mut min_res = usize::MAX;
    for i in 0..1000 {
        free_vals.push(i as f64);
        let res = solve_recursive(matrix, pivots, free_vars, free_vals);
        min_res = min_res.min(res);
        free_vals.pop();
    }
    min_res
}
