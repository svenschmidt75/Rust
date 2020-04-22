// https://www.geeksforgeeks.org/given-n-appointments-find-conflicting-appointments/

fn overlap(appointment1: (i64, i64), appointment2: (i64, i64)) -> bool {
    appointment1.1 > appointment2.0 && appointment1.0 < appointment2.1
}

fn find_conflicts(appointments: &[(i64, i64)]) -> Vec<((i64, i64), (i64, i64))> {
    // SS: We're returning ALL conflicts between intervals.
    // The problem actually only asks whether an appointment
    // conflicts with at least one earlier one, which can
    // be answered in O(N log N) time by constructing an
    // interval tree. Notice that an interval tree is also
    // be efficient at returning all conflicting appointments,
    // which would be N lookups at O(log N) each, hence
    // O(N log N) as well!

    // SS: O(N log N)
    let mut sorted_appointments = appointments.to_vec();
    sorted_appointments.sort_by_key(|(start, end)| *start);

    let mut conflicts = vec![];

    // SS: O(N^2) solution
    let mut i = 0;
    while i < sorted_appointments.len() {
        let a1 = sorted_appointments[i];

        let mut j = i + 1;
        while j < sorted_appointments.len() {
            let a2 = sorted_appointments[j];

            if overlap(a1, a2) {
                conflicts.push((a1, a2));
                j += 1;
            } else {
                i += 1;
                break;
            }
        }

        if j == sorted_appointments.len() {
            // SS: done, no more conflicts
            break;
        }
    }

    conflicts
}

#[cfg(test)]
mod tests {
    use crate::find_conflicts;

    #[test]
    fn test1() {
        // Arrange
        let input = [(1, 5), (3, 7), (2, 6), (10, 15), (5, 6), (4, 100)];

        // Act
        let conflicts = find_conflicts(&input);

        // Act
        assert_eq!(conflicts.len(), 10);
    }
}
