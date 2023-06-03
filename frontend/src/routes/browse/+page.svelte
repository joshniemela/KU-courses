<script>
import theme from "../../theme";
import SearchIcon from "../../assets/SearchIcon.svelte";
import FilterButton from "../../components/FilterButton/FilterButton.svelte";
import Dk from "../../assets/Dk.svelte";

</script>
<div class="browse-container">
    <div class="control-container">
        <FilterButton paddingLR={"2vw"} fontSize={"1.25rem"} />
        <input class="search" type="search" placeholder="Search" 
            style="--bg-color: {theme.colors.neutral[800]}"
        />
        <SearchIcon />
    </div>
    <div class="card-container">
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {0};
                "
        >
            <div class="card-header-container">
                <div class="card-title-container">
                    <h1 class="card-title" style="--text-color: {theme.colors.neutral[200]}">Course name</h1>
                    <h2 class="card-subtitle" style="--text-color: {theme.colors.neutral[600]}">abcdefg-course-id - SCIENCE</h2>
                </div>
                <table class="card-info-table">
                    <tr>
                        <td class="card-td-left-top">Bachelor</td>
                        <td class="card-td-right-top">15 ECTS</td>
                    </tr>
                    <tr>
                        <td class="card-td-left-bot"> Block 4</td>
                        <td> Group A</td>
                    </tr>
                </table>
            </div>
            <div class="card-description-container">
            <p class="card-description" style="--text-color: {theme.colors.neutral[200]}"> Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis. </p>
            </div>
            <div class="card-exam-text-container"
                style="--bg-color: {theme.colors.neutral[300]}"
            >
                <p class="card-exam-text"
                style="--text-color: {theme.colors.neutral[900]}"

                >Løbende evaluering - Mundtlig Eksamen</p>
            </div>
            <Dk />
        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {1};
                "
        >
        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {2};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {3};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {4};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {5};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {6};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {7};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {8};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {6};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {7};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {8};
                "
        >
        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {8};
                "
        >
        </div>
    </div>
</div>

<style scoped>
.browse-container {
    height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.control-container {
    margin-top: 1vh;
    height: 4vh;
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
}

.search {
    height: 100%;
    width: 30vw;
    max-width: 800px;
    margin-left: 1vw;
    font-size: 1.25rem;
    padding-left: 1vw;
    border: 0;
    background-color: var(--bg-color);

}

.card-container {
    height: 100%;
    width:96%;
    margin: 2%;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-auto-rows: 33vh;
    justify-content: center;
    align-items: center;
    gap: 2vh;
}

.card {
    display: flex;
    position: relative;
    flex-direction: column;
    justify-content: start;
    align-items: center;
    height: 100%;
    width: 100%; 
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
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    width: 96%;
}

.card-title-container {
    display: flex;
    flex-direction: column;
    justify-content: start;
    align-items: start;
}


.card-description-container {
    width: 96%;
    padding-left: 2%;
    padding-right: 2%;
    height: 80%;
    overflow-y: scroll;

}

.card-exam-text-container {
    position: relative;
    background-color: var(--bg-color);
    bottom: 0;
    width: 100%;
    height: 11.58%;
    display: flex;
    justify-content: center;
    align-items: center;
}

.card-exam-text {
    color: var(--text-color);
}

.card-title {
    font-size: 2rem;
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

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}


</style>


