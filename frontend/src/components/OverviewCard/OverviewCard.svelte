<script>
    import { navigate } from "svelte-navigator";
import Dk from "../../assets/Dk.svelte";
import Gb from "../../assets/Gb.svelte";
import theme from "../../theme.js";

export let stagger = 0;
export let data = {
  "description": {
    "description": [
      {
        "type": "p",
        "string": "Bioinformatik er en central del af biologiske uddannelser og central for forståelse af biologi og den bioteknologiske udnyttelse af biologiske data og en central integreret del af biologiske forskningsprojekter i både akademiske institutioner og biotekfirmaer. Bioinformatik 1 kurset vil give jer mulighed for at forstå og bruge bioinformatiske værktøjer til at analysere biologiske data. Kurset er præsenteret som en biologisk disciplin forankret i evolutionære teorier. Kurset består af fire hoved elementer:"
      },
      {
        "type": "p",
        "string": "•       At forstå grundlæggende bioinformatiske metoder og deres anvendelsesmuligheder."
      },
      {
        "type": "p",
        "string": "•       Hands-on erfaring med bioinformatiske metoder og værktøjer"
      },
      {
        "type": "p",
        "string": "•       Forståelse for de bagvedliggende principper."
      },
      {
        "type": "p",
        "string": "•       At kunne forholde sig kritisk til resultater opnået ved brug af bioinformatiske metoder."
      },
      {
        "type": "p",
        "string": "Kurset er bygget op over følgende faglige elementer:"
      },
      {
        "type": "li",
        "string": "Molekylære og proteinstrukturdatabaser, datakvalitet og database struktur."
      },
      {
        "type": "li",
        "string": "DNA og protein sekvens alignments, herunder pairwise og multiple aligments samt BLAST"
      },
      {
        "type": "li",
        "string": "Molekylær fylogeni og bootstrapping"
      },
      {
        "type": "li",
        "string": "RNA og Transcriptomics"
      },
      {
        "type": "li",
        "string": "Genom annotering og gene finding (protein kodende)"
      },
      {
        "type": "li",
        "string": "Motiv søgning"
      },
      {
        "type": "li",
        "string": "Genom analyser"
      },
      {
        "type": "li",
        "string": "Klassifikation af 3D-proteinstrukturer, sekundær struktur forudsigelse og 3D-struktur (komparativ) modellering"
      }
    ],
    "learning_outcome": [
      {
        "type": "p",
        "string": "Kursets hovedformål er at uddanne de studerende til at forstå grundlæggende digitale bioinformatiske metoder og deres anvendelsesmuligheder. Der er lagt vægt på hands-on erfaring samt at der opnås en grundlæggende forståelse for de bagvedliggende principper, således at de studerende bringes i stand til at de udnytte de enorme mængder tilgængelige data til at opnå biologisk viden, og til at kunne forholde sig kritisk til data opnået ved brug af bioinformatiske digitale metoder.  Når kurset er færdigt forventes den studerende at:"
      },
      {
        "type": "p",
        "string": "Viden:"
      },
      {
        "type": "li",
        "string": "have kendskab til klassificering og opbevaring af molekylære og protein databaser og deres struktur."
      },
      {
        "type": "li",
        "string": "kunne beskrive opbygning af molekylære og proteinstrukturdatabaser"
      },
      {
        "type": "li",
        "string": "have viden om Gene Ontologies"
      },
      {
        "type": "li",
        "string": "kunne beskrive grundlæggende love og principper for de mest udbredte bioinformatiske metoder (digitale værktøjer) til parvis og multiple sekvenssammenligning, fylogeni, detektion af sekvensmotiver, (herunder f.eks. bindingsites i promotorer)"
      },
      {
        "type": "li",
        "string": "have teknologisk forståelse for principperne bag bioinformatiske digitale værktøjer inden for opbygning af databaser, motiv søgning, parvise og multiple alignments, genom- og transkriptom analyse og protein homologi modellering"
      },
      {
        "type": "li",
        "string": "kunne beskrive grundlæggende principper for klassifikation af proteiners 3D strukturer og -modellering"
      },
      {
        "type": "li",
        "string": "have viden om transcriptomics og have kendskab til principper i gen findning af protein kodende og ikke-kodende RNA gener"
      },
      {
        "type": "p",
        "string": "Færdigheder:"
      },
      {
        "type": "li",
        "string": "kunne forholde sig kritisk til brug af digitale værktøjer inden for bioinformatikken og reflektere over valg af input data og forholde sig kritisk til output data."
      },
      {
        "type": "li",
        "string": "søge viden i eksisterende DNA, RNA og proteinstrukturdatabaser og genome browsers"
      },
      {
        "type": "li",
        "string": "udføre parvise og multiple sammenligninger af sekvenser"
      },
      {
        "type": "li",
        "string": "fremstille og forstå fylogenetiske træer"
      },
      {
        "type": "li",
        "string": "søge efter og identificere sekvensmotiver"
      },
      {
        "type": "li",
        "string": "udføre grundlæggende genomanalyse"
      },
      {
        "type": "li",
        "string": "grundlæggende håndtering af data ifbm sekvens assembly/read mapping"
      },
      {
        "type": "li",
        "string": "søge efter bindings motiv i DNA"
      },
      {
        "type": "li",
        "string": "søge efter ikke-kodende RNA gener"
      },
      {
        "type": "li",
        "string": "søge viden i og bruge eksisterende transcriptomics databaser"
      },
      {
        "type": "li",
        "string": "fremstille 3D-protein strukturer baseret på computerbaseret komparativ modellering"
      },
      {
        "type": "p",
        "string": "Kompetencer:"
      },
      {
        "type": "li",
        "string": "Forholde sig kritisk til valg af og brug af digitale værktøjer baseret på en forståelse af de grundlæggende principper bag bioinformatik. Forholde sig kritisk til input og output af data og resultater opnået ved brug af bioinformatikse metoder, herunder refleksion over tilgang til data, valg af digitale værktøjer og genereret data."
      }
    ],
    "recommended_academic_qualifications": [
      {
        "type": "p",
        "string": "Viden om DNA, RNA, proteiner og geners opbygning og funktioner. Basal genetik."
      }
    ]
  },
  "start_block": "1",
  "exams": [
    {
      "minutes": 20,
      "exam_type": "oral_examination"
    }
  ],
  "study_level": "Bachelor",
  "course_language": "da",
  "title": "Bioinformatik 1",
  "credits": 7.5,
  "course_id": "LBIB10123U",
  "schedules": [
    {
      "schedule_type": "A"
    }
  ],
  "employees": [
    {
      "email": "bak@plen.ku.dk",
      "full_name": "Søren Bak"
    }
  ],
  "workloads": [
    {
      "hours": 1,
      "workload_type": "exam"
    },
    {
      "hours": 10,
      "workload_type": "theory_exercises"
    },
    {
      "hours": 35,
      "workload_type": "lectures"
    },
    {
      "hours": 50,
      "workload_type": "practical_exercises"
    },
    {
      "hours": 110,
      "workload_type": "preparation"
    }
  ]
}

/**
* Function to scale the font sizes of the course titles based on their length
* @function calcFontSize
*/
function calcFontSize(string) {
    return (1 + 12/string.length)*16 + "px"
}

/**
* Function to navigate to the course corresponding with the course_id
* @function navigateToCourse
*/
function navigateToCourse() {
    navigate(`/course/${data.course_id}`);
    location.reload();
}


function convertExamToString(inputString) {
    return inputString.replace(/(\w)_(\w)/g, "$1 $2");
}

</script>
<div class="card-container" on:click={navigateToCourse}>
    <div class="card"
        style="
            --bg-color: {theme.colors.neutral[800]};
            --stagger: {stagger};
            "
    >
        <div class="card-header-container">
            <div class="card-title-container">
                <div class="title-container">
                    <a href={`/course/${data.course_id}`}>
                        <h1 class="card-title" 
                            style="
                                --text-color: {theme.colors.neutral[200]};
                                --text-size: {calcFontSize(data.title)}
                                "
                        >
                            {data.title}
                        </h1>
                    </a>
                </div>
                <h2 class="card-subtitle" style="--text-color: {theme.colors.neutral[600]}">{data.course_id} - SCIENCE</h2>
            </div>
            <table class="card-info-table">
                <tr>
                    <td class="card-td-left-top">{data.study_level}</td>
                    <td class="card-td-right-top">{data.credits} ECTS</td>
                </tr>
                <tr>
                    <td class="card-td-left-bot">Block {data.start_block}</td>
                    <td> Group: SCHEDULES</td>
                </tr>
            </table>
        </div>
        <div class="card-description-container">
            <p class="card-description">qweqweqweqweqweqweqweqweqwe</p>
        </div>
        <div class="card-exam-text-container"
            style="--bg-color: {theme.colors.neutral[300]}"
        >
            <p class="card-exam-text"
            style="--text-color: {theme.colors.neutral[900]}"

            >
                {convertExamToString(data.exams[0].exam_type)} - {data.exams[0].minutes} min
            </p>
            {#if data.course_language == "da"}
                <Dk />
            {:else}
                <Gb />
            {/if}
        </div>
    </div>
</div>

<style scoped>
.card {
    display: flex;
    position: relative;
    flex-direction: column;
    justify-content: start;
    align-items: center;
    width: 100%;
    height: 33vh;
    max-width: 30vw;
    background-color: var(--bg-color);
    border-radius: 10px;
    box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.15);
    transition: ease-in-out 0.2s;
    opacity: 0%;
    animation: fadeIn 0.5s calc(var(--stagger)*0.05s);
    animation-fill-mode: forwards;
    overflow: scroll;
}

.card:hover {
    scale: 1.02;
    box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.25);
    transition: ease-in-out 0.2s;
}

.card-header-container {
    height: 12vh;
    max-height: 12vh;
    overflow: hidden;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    width: 96%;
}

.card-title-container {
    max-width: 70%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: start;
}

.card-description-container {
    width: 96%;
    padding-left: 2%;
    padding-right: 2%;
    height: 80%;
    overflow: scroll;
}
.card-exam-text-container {
    position: relative;
    background-color: var(--bg-color);
    bottom: 0;
    width: 100%;
    height: 6vh;
    display: flex;
    justify-content: center;
    align-items: center;
}

.card-exam-text {
    font-size: 1rem;
    color: var(--text-color);
}

.card-title {
    font-size: var(--text-size);
    color: var(--text-color);
}

.card-subtitle {
    font-size: 1rem;
    color: var(--text-color);
}

.card-description {
    font-size: 1rem;
    color: var(--text-color);
}

.card-info-table {
    width: 30%;
    text-align: center;
    border-spacing: 0;
}

.card-td-left-top {
    border-bottom: 1px solid;
    border-right: 1px solid;
}

.card-td-right-top {
    border-bottom: 1px solid;
}

.card-td-left-bot {
    border-right: 1px solid;
}

.title-container {
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: auto;
    word-wrap: break-word;
}

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

</style>
