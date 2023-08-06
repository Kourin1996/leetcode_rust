#!/bin/bash

# Validate first argument (category name)
if [ -z "$1" ]; then
  echo "Category name is not given at the 1st argument"
  exit
fi

CATEGORY_DIR="src/problems/$1"
if [ ! -d "./$CATEGORY_DIR" ]; then
  echo "Category directory $CATEGORY_DIR does not exist, exiting"
  exit 1
fi

# Validate second argument (problem name)
if [ -z "$2" ]; then
  echo "Problem name is not given at the 2nd argument"
  exit
fi

PROBLEM_DIR="src/problems/$1/$2"
if [ -d "./$PROBLEM_DIR" ]; then
  echo "Problem directory $PROBLEM_DIR already exists, exiting"
  exit 1
fi

echo "Creating new runner for a problem into src/problems/$1/$2"

echo "pub mod $2;" >> "src/problems/$1.rs"

echo "pub mod problem;" >> "src/problems/$1/$2.rs"
echo "pub mod solution;" >> "src/problems/$1/$2.rs"

mkdir "src/problems/$1/$2"

cat << EOF > "src/problems/$1/$2/problem.rs"
/*
Description:
*/

#[cfg(test)]
mod tests {
    use crate::problems::$1::$2::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {

    }
}
EOF

cat << EOF > "src/problems/$1/$2/solution.rs"
pub struct Solution {}

/* Submission Code Begins */

/* Submission Code Ends */
EOF