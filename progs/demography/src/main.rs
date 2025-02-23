trait Stringify {
    fn to_string(&self) -> String;
}

enum Subject {
    Math,
    Physics,
    Chemistry,
    Biology,
    History,
    Geography,
    Literature,
    Philosophy,
    Music,
    Art,
    PhysicalEducation,
    ForeignLanguage,
    ComputerScience,
    Civics,
}

/* Enums can also implement traits (similar to structs) */
impl Stringify for Subject {
    fn to_string(&self) -> String {
        use Subject::*;
        match self {
            Math => "Mathematics".to_string(),
            Physics => "Physics".to_string(),
            Chemistry => "Chemistry".to_string(),
            Biology => "Biology".to_string(),
            History => "History".to_string(),
            Geography => "Geography".to_string(),
            Literature => "Literature".to_string(),
            Philosophy => "Philosophy".to_string(),
            Music => "Music".to_string(),
            Art => "Art".to_string(),
            PhysicalEducation => "Physical Education".to_string(),
            ForeignLanguage => "Foreign Language".to_string(),
            ComputerScience => "Computer Science".to_string(),
            Civics => "Civics".to_string(),
        }
    }
}

enum Field {
    Biology,
    Chemistry,
    Physics,
    MaterialScience,
}

impl Stringify for Field {
    fn to_string(&self) -> String {
        use Field::*;
        match self {
            Biology => "Biology".to_string(),
            Chemistry => "Chemistry".to_string(),
            Physics => "Physics".to_string(),
            MaterialScience => "Material Science".to_string(),
        }
    }
}

enum House {
    LokSabha,
    RajyaSabha,
}

impl Stringify for House {
    fn to_string(&self) -> String {
        match self {
            House::LokSabha => "Lok Sabha".to_string(),
            House::RajyaSabha => "Rajya Sabha".to_string(),
        }
    }
}

enum Constituency {
    North,
    South,
    East,
    West,
    Central,
    Suburban,
    Rural,
    Urban,
    Metropolitan,
    Industrial,
    Agricultural,
    Coastal,
    Hilly,
    Desert,
    Island,
    Border,
    Tribal,
    Reserved,
    Unreserved,
    General,
    Scheduled,
    Backward,
    Minority,
}

impl Stringify for Constituency {
    fn to_string(&self) -> String {
        use Constituency::*;
        match self {
            North => "North".to_string(),
            South => "South".to_string(),
            East => "East".to_string(),
            West => "West".to_string(),
            Central => "Central".to_string(),
            Suburban => "Suburban".to_string(),
            Rural => "Rural".to_string(),
            Urban => "Urban".to_string(),
            Metropolitan => "Metropolitan".to_string(),
            Industrial => "Industrial".to_string(),
            Agricultural => "Agricultural".to_string(),
            Coastal => "Coastal".to_string(),
            Hilly => "Hilly".to_string(),
            Desert => "Desert".to_string(),
            Island => "Island".to_string(),
            Border => "Border".to_string(),
            Tribal => "Tribal".to_string(),
            Reserved => "Reserved".to_string(),
            Unreserved => "Unreserved".to_string(),
            General => "General".to_string(),
            Scheduled => "Scheduled".to_string(),
            Backward => "Backward".to_string(),
            Minority => "Minority".to_string(),
        }
    }
}

enum Job {
    Teacher(Subject),
    Scientist(Field),
    MemberOfParliament(House, Constituency),
}

struct Person {
    name: String,
    age: u8,
    job: Job,
}

impl Person {
    fn new(name: String, age: u8, job: Job) -> Self {
        Person { name, age, job }
    }

    fn is_teacher(&self) -> bool {
        match self.job {
            Job::Teacher(_) => true,
            _ => false,
        }
    }

    fn is_scientist(&self) -> bool {
        match self.job {
            Job::Scientist(_) => true,
            _ => false,
        }
    }

    fn is_member_of_parliament(&self) -> bool {
        match self.job {
            Job::MemberOfParliament(_, _) => true,
            _ => false,
        }
    }

    fn describe(&self) {
        match &self.job {
            Job::Teacher(sub) => println!(
                "{} aged {} is a teacher of subject {}",
                self.name,
                self.age,
                sub.to_string()
            ),
            Job::Scientist(field) => println!(
                "{} aged {} is a scientist in field {}",
                self.name,
                self.age,
                field.to_string()
            ),
            Job::MemberOfParliament(house, constituency) => println!(
                "{} aged {} is a member of parliament in the {} representing the constituency of {}",
                self.name,
                self.age,
                house.to_string(),
                constituency.to_string()
            ),
        }
    }
}

fn main() {
    let teacher = Person::new("Sandeep Jha".to_string(), 35, Job::Teacher(Subject::Math));
    let scientist = Person::new(
        "Jeff Thompson".to_string(),
        52,
        Job::Scientist(Field::MaterialScience),
    );
    let mp = Person::new(
        "Narendra Modi".to_string(),
        70,
        Job::MemberOfParliament(House::LokSabha, Constituency::General),
    );
    teacher.describe();
    scientist.describe();
    mp.describe();

    if teacher.is_teacher() {
        println!("{} is a teacher", teacher.name);
    }

    if scientist.is_scientist() {
        println!("{} is a scientist", scientist.name);
    }

    if mp.is_member_of_parliament() {
        println!("{} is a member of parliament", mp.name);
    }

    if !teacher.is_scientist() {
        println!("{} is not a scientist", teacher.name);
    }

    if !scientist.is_teacher() {
        println!("{} is not a teacher", scientist.name);
    }

    if !mp.is_teacher() {
        println!("{} is not a teacher", mp.name);
    }

    if !teacher.is_member_of_parliament() {
        println!("{} is not a member of parliament", teacher.name);
    }
}
