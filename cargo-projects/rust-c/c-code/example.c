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
    // const char *other_name = "Other";
    // person->last_name = other_name;

    // const char *prefix = "Pref";
    // update_done_cb("Some", person->last_name);

    // update_done_cb("Original person name", person->first_name);
    person_cap_first_name(person, update_done_cb);
    const char *last_name = "Brown";
    person_update_last_name(person, last_name, update_done_cb);

    struct CPerson *cperson;
    enum PersonStatus p = Person_c_new("A", "B", &cperson);
    update_done_cb("Original CPerson f name", cperson->first_name);
    cperson->first_name = "C";
    update_done_cb("Updated CPerson f name", cperson->first_name);
}

int main(void)
{
    // we declare an instance of struct Person here.
    // *person is a pointer to struct tagged as Person.
    // We can use it to indirectly access and manipulate the members of this struct
    struct Person *person;
    // We call Person_new (wrapper level function in Rust) and pass two string arguments
    // and a reference to the struct
    enum PersonStatus p = Person_new(
        "original F Name",
        "Name",
        &person);

    run(person);
}
