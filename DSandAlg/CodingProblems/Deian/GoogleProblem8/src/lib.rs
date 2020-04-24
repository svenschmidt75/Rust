/* You are given a set of clips from a game, taken from multiple cameras.
 * Each clip has a start and end time. Construct a clip covering the full
 * duration of the game, using the least number of clips.
 *
 * Example 1:
 * Input: [(1, 2), (4, 5), (0, 3), (3, 5)]
 * Output: [(0, 3), (3, 5)]
 *
 * Example 2:
 * Input: [(0, 2), (0, 3), (1, 2), (2, 5), (3, 4), (3, 5), (4, 5)]
 * Output: (0, 3), (3, 5)
 */

use std::collections::HashMap;

fn create_clip_min(clips: &[(i64, i64)]) -> Vec<(i64, i64)> {
    // SS: check for empty

    // SS: sort clips w.r.t. start time
    // O(N log N)
    let mut sorted = clips.to_owned();
    sorted.sort_by_key(|(start, _)| *start);

    let mut min_clips = vec![];

    let mut current_max = sorted[0].0;
    let mut i = 0;

    // SS: Although we have 2 nested loops, since we only "paint" each element once,
    // the runtime is O(N), NOT O(N^2)!!!
    while i < sorted.len() {
        let mut clip1 = sorted[i];
        if clip1.1 <= current_max {
            // SS: interval is already covered
            i += 1;
            continue;
        }

        // SS: while same start time, find the clip
        // with the maximum duration...
        let mut j = i + 1;
        while j < sorted.len() {
            let clip2 = sorted[j];
            if clip1.0 == clip2.0 {
                if clip2.1 > clip1.1 {
                    clip1 = clip2;
                    j += 1;
                    continue;
                }
            }

            // SS: different start times, must include clip1
            i = j - 1;
            break;
        }

        min_clips.push(clip1);
        current_max = clip1.1;
        i += 1;
    }

    min_clips
}

fn follow_up_problem_set_covering(clips: &[(i64, i64)]) -> Vec<(i64, i64)> {
    // SS: check for empty

    /* Follow-up: Construct a clip covering the full duration of the game while minimizing the sum of lengths of all clips.
     * Example:
     *   Input: [(1, 2), (4, 5), (0, 3), (3, 5)]
     *   Output: [(0, 3), (3, 5)]
     *
     * A lower bound on the sum is the duration of the entire game. This would be the ideal solution.
     * Note that the problem statement does NOT ask for a minimum number of clips!
     *
     * I think this is the set covering problem. We want to find a set of intervals ("radio stations") covering the whole
     * game duration with a minimum number of overlap between clips.
     * Strategy: Find the clip power set (i.e. all combinations of clips, there are 2^N). Then find the one with
     * minimum overlap. Note that the solution does not have to minimize the number of clips at the same time!
     */

    // SS: sort clips w.r.t. start time
    // O(N log N)
    let mut sorted = clips.to_owned();
    sorted.sort_by_key(|(start, _)| *start);

    let min = sorted[0].0;
    let &(_, max) = sorted.iter().max_by_key(|(_, end)| *end).unwrap();

    let mut complete_coverage = HashMap::new();
    for i in min..=max {
        complete_coverage.insert(i, 1);
    }

    let mut min_overlap = std::u64::MAX;
    let mut max_coverage = std::u64::MIN;
    let mut best_set = vec![];

    let mut check = |set: &Vec<(i64, i64)>| {
        let m = complete_coverage.clone();
        let (coverage, overlap) = find_overlap(set, m);
        if coverage > max_coverage {
            max_coverage = coverage;
            best_set = set.clone();
        } else if coverage == max_coverage && overlap < min_overlap {
            min_overlap = overlap;
            best_set = set.clone();
        }
    };

    create(&sorted, 0, 0, 0, vec![], &mut check);

    best_set
}

fn find_overlap(set: &Vec<(i64, i64)>, mut complete_set: HashMap<i64, i64>) -> (u64, u64) {
    // SS: returns (interval covered, overlap count)
    for i in 0..set.len() {
        let clip = set[i];

        for j in clip.0..clip.1 {
            let slot = complete_set.get_mut(&j).unwrap();
            *slot -= 1;
        }
    }

    let (coverage, overlap) = evaluate(&complete_set);
    (coverage, overlap)
}

fn evaluate(complete_set: &HashMap<i64, i64>) -> (u64, u64) {
    // SS: coverage if value is <= 0
    let mut coverage = 0;
    let mut overlap = 0;
    for (_, &item) in complete_set {
        if item <= 0 {
            coverage += 1;
            if item < 0 {
                overlap += -item;
            }
        }
    }
    (coverage, overlap as u64)
}

fn create<F>(
    clips: &[(i64, i64)],
    min: i64,
    max: i64,
    index: usize,
    set: Vec<(i64, i64)>,
    check: &mut F,
) where
    F: FnMut(&Vec<(i64, i64)>),
{
    if index == clips.len() {
        // SS: check set for optimality
        check(&set);
    } else {
        // SS: do not include clips[index]
        create(clips, min, max, index + 1, set.clone(), check);

        // SS: include clips[index]
        let mut new_set = set.clone();
        let clip = clips[index];
        new_set.push(clip);
        create(clips, min, max, index + 1, new_set, check);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_1() {
        // Arrange
        let clips = [(1, 2), (4, 5), (0, 3), (3, 5)];

        // Act
        let min_clips = create_clip_min(&clips);

        // Assert
        assert_eq!(min_clips.len(), 2);
        assert_eq!(min_clips, vec![(0, 3), (3, 5)]);
    }

    #[test]
    fn test_min_2() {
        // Arrange
        let clips = [(0, 2), (0, 3), (1, 2), (2, 5), (3, 4), (3, 5), (4, 5)];

        // Act
        let min_clips = create_clip_min(&clips);

        // Assert
        assert_eq!(min_clips.len(), 2);
        assert_eq!(min_clips, vec![(0, 3), (2, 5)]);
    }

    #[test]
    fn test_min_3() {
        // Arrange
        let clips = [(1, 2), (4, 5), (0, 3), (3, 5)];

        // Act
        let min_clips = follow_up_problem_set_covering(&clips);

        // Assert
        assert_eq!(min_clips.len(), 2);
        assert_eq!(min_clips, vec![(0, 3), (3, 5)]);
    }

    #[test]
    fn test_min_4() {
        // Arrange
        let clips = [(0, 2), (1, 2), (2, 4), (3, 5)];

        // Act
        let min_clips = follow_up_problem_set_covering(&clips);

        // Assert
        assert_eq!(min_clips.len(), 3);
        assert_eq!(min_clips, vec![(0, 2), (2, 4), (3, 5)]);
    }
}
