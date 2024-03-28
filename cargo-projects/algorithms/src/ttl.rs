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

fn query_count(events: Vec<Event>) -> Vec<i32> {
    let mut query_count = vec![];
    let mut start_count = 0;

    for res in events {
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

fn start_end_events(events: &mut Vec<Event>, data: &[[i32; 2]]) {
    for data_point in data.iter() {
        let [start, ttl] = data_point;

        let start_time = Event::new(EventType::Start, *start);
        events.push(start_time);

        let end_time = Event::new(EventType::End, start + ttl);
        events.push(end_time);
    }
}

fn query_events(events: &mut Vec<Event>, queries: &[i32]) {
    for query in queries.iter() {
        let query_e = Event::new(EventType::Query, *query);
        events.push(query_e);
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
    let mut events = Vec::new();

    // insert Start and End events
    start_end_events(&mut events, data);

    // insert Query events
    query_events(&mut events, queries);

    // sort all values (events)
    events.sort_by(|a, b| a.value.cmp(&b.value));

    query_count(events)
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
