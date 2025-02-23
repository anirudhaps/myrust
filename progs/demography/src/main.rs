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

fn subject_to_string(sub: &Subject) -> String {
    match sub {
        Subject::Math => "Mathematics".to_string(),
        Subject::Physics => "Physics".to_string(),
        Subject::Chemistry => "Chemistry".to_string(),
        Subject::Biology => "Biology".to_string(),
        Subject::History => "History".to_string(),
        Subject::Geography => "Geography".to_string(),
        Subject::Literature => "Literature".to_string(),
        Subject::Philosophy => "Philosophy".to_string(),
        Subject::Music => "Music".to_string(),
        Subject::Art => "Art".to_string(),
        Subject::PhysicalEducation => "Physical Education".to_string(),
        Subject::ForeignLanguage => "Foreign Language".to_string(),
        Subject::ComputerScience => "Computer Science".to_string(),
        Subject::Civics => "Civics".to_string(),
    }
}

enum Field {
    Biology,
    Chemistry,
    Physics,
    MaterialScience,
}

fn field_to_string(field: &Field) -> String {
    match field {
        Field::Biology => "Biology".to_string(),
        Field::Chemistry => "Chemistry".to_string(),
        Field::Physics => "Physics".to_string(),
        Field::MaterialScience => "Material Science".to_string(),
    }
}

enum House {
    LokSabha,
    RajyaSabha,
}

fn house_to_string(house: &House) -> String {
    match house {
        House::LokSabha => "Lok Sabha".to_string(),
        House::RajyaSabha => "Rajya Sabha".to_string(),
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

fn constituency_to_string(constituency: &Constituency) -> String {
    match constituency {
        Constituency::North => "North".to_string(),
        Constituency::South => "South".to_string(),
        Constituency::East => "East".to_string(),
        Constituency::West => "West".to_string(),
        Constituency::Central => "Central".to_string(),
        Constituency::Suburban => "Suburban".to_string(),
        Constituency::Rural => "Rural".to_string(),
        Constituency::Urban => "Urban".to_string(),
        Constituency::Metropolitan => "Metropolitan".to_string(),
        Constituency::Industrial => "Industrial".to_string(),
        Constituency::Agricultural => "Agricultural".to_string(),
        Constituency::Coastal => "Coastal".to_string(),
        Constituency::Hilly => "Hilly".to_string(),
        Constituency::Desert => "Desert".to_string(),
        Constituency::Island => "Island".to_string(),
        Constituency::Border => "Border".to_string(),
        Constituency::Tribal => "Tribal".to_string(),
        Constituency::Reserved => "Reserved".to_string(),
        Constituency::Unreserved => "Unreserved".to_string(),
        Constituency::General => "General".to_string(),
        Constituency::Scheduled => "Scheduled".to_string(),
        Constituency::Backward => "Backward".to_string(),
        Constituency::Minority => "Minority".to_string(),
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
                subject_to_string(sub)
            ),
            Job::Scientist(field) => println!(
                "{} aged {} is a scientist in field {}",
                self.name,
                self.age,
                field_to_string(field)
            ),
            Job::MemberOfParliament(house, constituency) => println!(
                "{} aged {} is a member of parliament in the {} representing the constituency of {}",
                self.name,
                self.age,
                house_to_string(house),
                constituency_to_string(constituency)
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
