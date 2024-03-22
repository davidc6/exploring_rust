#[derive(Debug, PartialEq)]
enum EventType {
    Start,
    End,
    Query,
}

#[derive(Debug, PartialEq)]
struct Event {
    event_type: EventType,
    value: i32,
}

impl Event {
    fn new(event_type: EventType, value: i32) -> Self {
        Event { event_type, value }
    }
}

/// Count all queries that are within the boundaries of start time and ttl
///
/// Example:
/// first value is start time, second is ttl
/// data = [[1, 10], [3, 10], [5, 10]]
/// queries = [1, 12, 14, 60]
///
/// 1  is within the boundaries of 1 and 1 + 10 so increment count by 1
/// 12 is within the boundaries of 3 and 3 + 10 and 5 + 10 so increment by two
/// 14 is withing the boundaries of 5 + 10 so increment by one
/// 60 is not withing the boundaries so do not increment
///
/// So the result will be [1, 2, 1, 0]
///
pub fn run_ttl(data: &[[i32; 2]], queries: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    // insert Start and End
    for data_point in data.iter() {
        let [start, ttl] = data_point;

        let start_time = Event::new(EventType::Start, *start);
        result.push(start_time);

        let end_time = Event::new(EventType::End, start + ttl);
        result.push(end_time);
    }

    // insert Query
    for query in queries.iter() {
        let query_e = Event::new(EventType::Query, *query);
        result.push(query_e);
    }

    // sort all values (events)
    result.sort_by(|a, b| a.value.cmp(&b.value));

    let mut query_count = vec![];
    let mut start_count = 0;

    for res in result {
        let Event { event_type, .. } = res;

        if event_type == EventType::Start {
            start_count += 1;
            continue;
        }

        if event_type == EventType::End {
            start_count -= 1;
            continue;
        }

        if event_type == EventType::Query {
            query_count.push(start_count);
        }
    }

    query_count
}

#[cfg(test)]
mod run_ttl_tests {
    use super::run_ttl;

    #[test]
    fn test_queries() {
        let data = vec![[10, 10], [20, 10], [10, 30]];
        let queries = [15, 50, 40, 30];

        let actual = run_ttl(&data, &queries);
        let expected = vec![2, 1, 0, 0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_queries_2() {
        let data = vec![[1, 10], [3, 10], [5, 10]];
        let queries = [1, 12, 14, 60];

        let actual = run_ttl(&data, &queries);
        let expected = vec![1, 2, 1, 0];

        assert_eq!(actual, expected);
    }
}
