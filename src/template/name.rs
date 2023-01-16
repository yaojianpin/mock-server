use rand::Rng;

pub fn first() -> String {
    let first_male_names = [
        "James",
        "John",
        "Robert",
        "Michael",
        "William",
        "David",
        "Richard",
        "Charles",
        "Joseph",
        "Thomas",
        "Christopher",
        "Daniel",
        "Paul",
        "Mark",
        "Donald",
        "George",
        "Kenneth",
        "Steven",
        "Edward",
        "Brian",
        "Ronald",
        "Anthony",
        "Kevin",
        "Jason",
        "Matthew",
        "Gary",
        "Timothy",
        "Jose",
        "Larry",
        "Jeffrey",
        "Frank",
        "Scott",
        "Eric",
    ];
    let first_female_names = [
        "Mary",
        "Patricia",
        "Linda",
        "Barbara",
        "Elizabeth",
        "Jennifer",
        "Maria",
        "Susan",
        "Margaret",
        "Dorothy",
        "Lisa",
        "Nancy",
        "Karen",
        "Betty",
        "Helen",
        "Sandra",
        "Donna",
        "Carol",
        "Ruth",
        "Sharon",
        "Michelle",
        "Laura",
        "Sarah",
        "Kimberly",
        "Deborah",
        "Jessica",
        "Shirley",
        "Cynthia",
        "Angela",
        "Melissa",
        "Brenda",
        "Amy",
        "Anna",
    ];
    let first_names = [first_male_names, first_female_names].concat();

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(1..first_names.len());
    first_names[idx].to_string()
}

pub fn last() -> String {
    let last_names = [
        "Smith",
        "Johnson",
        "Williams",
        "Brown",
        "Jones",
        "Miller",
        "Davis",
        "Garcia",
        "Rodriguez",
        "Wilson",
        "Martinez",
        "Anderson",
        "Taylor",
        "Thomas",
        "Hernandez",
        "Moore",
        "Martin",
        "Jackson",
        "Thompson",
        "White",
        "Lopez",
        "Lee",
        "Gonzalez",
        "Harris",
        "Clark",
        "Lewis",
        "Robinson",
        "Walker",
        "Perez",
        "Hall",
        "Young",
        "Allen",
    ];

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(1..last_names.len());
    last_names[idx].to_string()
}

pub fn name(_min: usize, _max: usize) -> String {
    first() + " " + &last()
}
