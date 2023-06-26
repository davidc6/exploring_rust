#include <stdio.h>
#include "../rust-code/include/bindings.h"

// const char *something - non-modifiable (read-only) character data pointer
// or pointer to constant char
void update_done_cb(const char *prefix, const char *name)
{
    fprintf(stderr, "%s: %s\n", prefix, name);

    // some other functionality can be added here as an example
}

void run(struct Person *person)
{
    person_cap_first_name(person, update_done_cb);

    const char *last_name = "Brown";

    person_update_last_name(person, last_name, update_done_cb);
}

int main(void)
{
    struct Person *person;
    enum PersonStatus p = Person_new(
        "test",
        "Name",
        &person);

    run(person);
}
