<script>
import theme from '../../theme';

// Props
export let paddingLR = "1vw";
export let fontSize = "1.5rem";

// Button on hover animation
let buttonTextColor = theme.colors.brand[200]
let buttonBgColor = theme.colors.brand[800]
function handleHover(e) {
    buttonTextColor = theme.colors.brand[900]
    buttonBgColor = theme.colors.brand[500]
}
function handleHoverOut(e) {
    buttonTextColor = theme.colors.brand[200]
    buttonBgColor = theme.colors.brand[800]
}
</script>

<button class="filter-button" style=
    "
    --bg-color: {buttonBgColor};
    --text-color: {buttonTextColor};
    --padding: {paddingLR};
    --font-size: {fontSize}
    "
    on:mouseover={handleHover}
    on:mouseout={handleHoverOut}
>Filter</button>

<style scoped>
.filter-button {
    background: none;
    font-size: var(--font-size);
    border: 0;
    padding-right: var(--padding);
    padding-left: var(--padding);
    border-color: var(--text-color);
    color: var(--text-color);
    height: 100%;
    background-color: var(--bg-color);
    transition: ease-in-out 0.1s;
}
</style>
