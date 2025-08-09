use std::env;

use std::time;
pub mod parser;

//const DEFAULT_DATA_DIR: &str = "../../data";
//const HTMLS_DIR: &str = "../../data/pages";
//const TEST_DIR: &str = "./test_data";
//const TEST_HTMLS_DIR: &str = "./test_data/pages";
//const JSON_DIR: &str = "../../data/new_json";

// make a function that takes a path and returns the number of fails and the total number of courses
fn count_fails(htmls_dir: &str, json_dir: &str) -> (usize, usize) {
    let mut fails = 0;
    let mut passes = 0;
    let dir = std::fs::read_dir(htmls_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        // read the string from the file
        let html = std::fs::read_to_string(entry.path()).unwrap();
        // parse the string
        let course = parser::parse_course(&html);
        // if the error cause (this is an anyhow context) contains <EXPECTED>, then we ignore it and continue
        match course {
            Ok(c) => {
                // emit json to file
                let json = serde_json::to_string(&c).unwrap();
                let path = format!("{}/{}.json", json_dir, c.info.id);
                std::fs::write(path, json).unwrap();
                passes += 1;
            }

            Err(e) => {
                // if any of the causes contain <EXPECTED>, then we ignore it and continue
                if e.chain().any(|c| c.to_string().contains("<EXPECTED>")) {
                    continue;
                } else {
                    fails += 1;
                    println!("Error: {e:?}\n\n");
                }
            }
        }
    }
    (fails, passes)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let timer = time::Instant::now();
    let html_dir = &args[1];
    let json_dir = &args[2];

    // print all files in the html directory
    let _dir = std::fs::read_dir(html_dir).unwrap();
    println!("fails and total: {:?}", count_fails(html_dir, json_dir));

    println!("Time elapsed: {:?}", timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    const TEST_HTMLS_DIR: &str = "./test_data/pages";
    use parser::{
        Coordinator, Course, Department, Description, Exam, Faculty, LogisticInformation, Workload,
        WorkloadType,
    };

    // check that there are files in PAGES_DIR
    #[test]
    fn test_pages_dir() {
        let dir = std::fs::read_dir(TEST_HTMLS_DIR).unwrap();
        assert!(dir.count() > 0);
    }

    #[test]
    fn test_lsls10061_u() {
        let html = std::fs::read_to_string(format!("{TEST_HTMLS_DIR}/LSLS10061U.html")).unwrap();
        let course = parser::parse_course(&html);
        let expected_course = Course {
            title: "International Naturforvaltning".to_string(),
            info: parser::CourseInformation::new(
                "LSLS10061U".to_string(),
                7.5,
                vec![parser::Block::Two],
                vec![parser::Schedule::B],
                vec![parser::Language::Danish],
                parser::Duration::One,
                vec![parser::Degree::Bachelor],
                parser::Capacity(Some(70)),
            ),
            description: Description {
                content: "<p><strong>Skov og naturressourcer globalt:</strong></p><p>Globale klima- og vegetationszoner og deres økologi</p><p>Verdens naturressourcer, skove, nationalparker og\nnaturområder</p><p>Naturbeskyttelse og bevarelsesprogrammer</p><p>Globale skovopgørelser, fakta og trends, klassifikationer\u{a0}-\nog problematikker</p><p>Udfordringer i forhold til en bæredygtig anvendelse af\nnaturressourcer globalt</p><p>\u{a0}</p><p><strong>International forvaltning:</strong></p><p>International skov og naturressourceforvaltning - politisk,\ninstitutionelt og økonomisk</p><p>- Skovpolitik og regeringsførelse</p><p>- Internationale konventioner</p><p>- Regional EU politik og regelsæt\u{a0}</p><p>International skov og naturressourceforvaltning - decentrale\nsystemer\u{a0}</p><p>- Agroforestry</p><p>- Samfundsbaseret naturressourceforvaltning, indfødte folk og\nviden</p><p>Naturressourcebaserede konflikter</p><p>Klimaændringer ift. skov og natur</p><p>\u{a0}</p><p><strong>Produktion, markedet og handel:</strong></p><p>International skov og naturressourceforvaltning -\nkommercielt</p><p>- Plantage-\u{a0} koncessionsskovbrug</p><p>- Dyrkningssystemer og problematikker, optimering og\ngenbevarelse</p><p>- International handel med træprodukter \u{a0}</p><p>- Ulovlig hugst og handel</p><p>- Global og EU markedsbaseret regulering (skovcertificering,\nlegalitetsverificering, FLEGT)</p>".to_string(),
                learning_outcome: "<p>Kursets overordnede formål er at give den enkelte studerende\nviden om de vilkår, der danner rammerne for forvaltningen af skov\nog naturressourcer verden over. Det er også at give en\ngrundlæggende forståelse for hvordan de internationale rammer og\nvilkår har betydning for den måde, som skov og naturressourcer\nforvaltes i Danmark.</p><p><br></br><strong>Viden:</strong></p><p>- Kendskab til vækstvilkår og vegetationstyper globalt</p><p>- Kendskab til direkte og underliggende årsager til afskovning\nog over-udnyttelse af ressourcer</p><p>- Indsigt i koncessionsskovbrug, plantagedrift og\nagro-forestry</p><p>- Kendskab til internationale konventioner og EU lovgivning på\nskov og naturressourceområdet</p><p>- Kendsakbs til bevaringsklassifikationer og naturbeskyttelse i\npraksis</p><p>- Kendskab til de mest almindelige\u{a0}tømmertræarter</p><p>- Kendskab til det internationale markeds betydning for\nforvaltningen af skov- og naturressourcer</p><p>- Indsigt i mekanismerne der driver ulovlig hugst og handel med\ntræ</p><p>- Indsigt i markedsbaseret regulering</p><p>- Kendskab til de mest almindelige certificeringssystemer</p><p>- Indsigt i betydningen af\u{a0}klimaforandringer\u{a0}for den\nmåde skov og naturressourcer\u{a0} forvaltes\u{a0}i fremtiden</p><p>\u{a0}</p><p><strong>Færdigheder:</strong></p><p>- Redegøre for hvordan de naturgivne vækstvilkår i forskellige\nvegetationszoner har betydning for den måde som skov – og\nnaturressourcer i disse zoner bør forvaltes</p><p>- Redegøre for og diskutere hvordan problemer over-udnyttelse af\nskov og naturressourcer kan afhjælpes\n<br></br><br></br>\n- Redegøre for hvordan internationale konventioner og EU lovgivning\nhar betydning for forvaltningen af skov og naturressourcer\nlokalt</p><p>- Analysere globale markedstrends og forklare hvordan disse har\nbetydning for forvaltningen af skov- og naturressourcer</p><p>- Redegøre for de grundlæggende principper bag decentraliseret\nressourceforvaltning</p><p>- Identificere de udfordringer, som forvaltningen af\nfællesressourcer kan medføre</p><p>- Forklare hvordan markedsbaseret regulering af skov og\nnaturressourceforvaltning fungerer</p><p>- Redegøre for de vigtigste drivere bag ulovlig hugst og handel\nsamt hvordan det kan bekæmpes</p><p>- Opstille praktiske løsninger for hvordan negative konsekvenser\naf klimaforandringer kan afhjælpes</p><p>- Kan argumentere for og imod decentraliseret skov- og\nnaturressourceforvaltning</p><p><br></br><strong>Kompetencer:</strong></p><p>- Arbejde med skov og naturressourceforvaltning under de\nrammevilkår som internationale konventioner og EU lovgivning\ngiver</p><p>- Bidrage til at afhjælpe degradering og over-udnyttelse af skov\nog naturressourcer</p><p>- Indgå i samarbejder omkring forvaltningen af skov og\nnaturressource i forskellige klimazoner - både nationalt og\ninternationalt</p><p>- Tage informerede driftsbeslutninger under hensyntagen til\nnationale og internationale markedstrends</p><p>- Arbejde med certificering og legalitetsveriticering</p><p>- Integrere klimatilpasningshensyn i forvaltningen af skov og\nnaturressourcer</p>".to_string(),
                recommended_qualifications: Some("Basal forståelse for\nnaturforvaltning og/eller samfundsvidenskab.\n<br></br>\nSprogkundskaber til at kunne læse og forstå engelsk faglitteratur.\n<br></br>\nTilnærmelsesvis alt litteratur er på engelsk.".to_string()),
                summary: "Skov og naturressourcer globalt:Globale klima- og vegetationszoner og deres økologiVerdens naturressourcer, skove, nationalparker og\nnaturområderNaturbeskyttelse og bevarelsesprogrammerGlobale skovopgørelser, fakta og trends, klassifikationer\u{a0}-\nog problematikkerUdfordringer i forhold til en bæredygt".to_string(),
            },
            exams: vec![Exam::Oral(Some(30))],
            /*
             * >    workloads: [
>        Workload {
>            workload_type: Lectures,
>            hours: 98.0,
>        },
>        Workload {
>            workload_type: Preparation,
>            hours: 97.0,
>        },
>        Workload {
>            workload_type: ProjectWork,
>            hours: 8.0,
>        },
>        Workload {
>            workload_type: Exam,
>            hours: 1.0,
>        },
>    ],
             */
            workloads: vec![
                Workload::new(WorkloadType::Lectures, 98.0),
                Workload::new(WorkloadType::Preparation, 97.0),
                Workload::new(WorkloadType::ProjectWork, 8.0),
                Workload::new(WorkloadType::Exam, 1.0),
            ],

            logistics: LogisticInformation::new(
                vec![Department::GeosciencesAndNaturalResourceManagement],
                Faculty::Science,
                vec![Coordinator::new(
                    "Kirsten Carlsen".into(),
                    "bdk748@alumni.ku.dk".into(),
                )],
            ),
        };
        assert_eq!(expected_course, course.unwrap());
    }

    // We need to ignore the duration if the course is known to be a summer course.
    #[ignore]
    #[test]
    fn test_nbik15000_u() {
        let html = std::fs::read_to_string(format!("{TEST_HTMLS_DIR}/NBIK15000U.html")).unwrap();
        let course = parser::parse_course(&html);
        let expected_course = Course {
            title: "BAdvanced Plant Identification".to_string(),
            info: parser::CourseInformation::new(
                "NBIK15000U".to_string(),
                7.5,
                vec![parser::Block::Summer],
                vec![parser::Schedule::B], // doesnt exist
                vec![parser::Language::English],
                parser::Duration::One,
                vec![parser::Degree::Master],
                parser::Capacity(Some(16)),
            ),
            description: Description {
                content: "".to_string(),
                learning_outcome: "".to_string(),
                recommended_qualifications: Some("".to_string()),
                summary: "".to_string(),
            },
            exams: vec![Exam::Oral(Some(30))],
            workloads: Vec::new(),
            logistics: LogisticInformation::new(Vec::new(), Faculty::Science, Vec::new()),
        };
        assert_eq!(expected_course, course.unwrap());
    }
}
