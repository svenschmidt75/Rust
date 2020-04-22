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

fn create_clip_min(clips: &[(i64, i64)]) -> Vec<(i64, i64)> {
    // SS: check for empty

    // SS: sort clips w.r.t. start time
    // O(N log N)
    let mut sorted = clips.to_owned();
    sorted.sort_by_key(|(start, end)| *start);

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

#[cfg(test)]
mod tests {
    use crate::create_clip_min;

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
}
