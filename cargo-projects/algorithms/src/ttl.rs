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
}
