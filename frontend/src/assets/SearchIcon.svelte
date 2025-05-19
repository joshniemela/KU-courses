<script>
    import { createBubbler } from "svelte/legacy";

    const bubble = createBubbler();
    import theme from "../theme";
    const fill = theme.colors.neutral[200];
    const BgColor = theme.colors.neutral[800];
    const fillHover = theme.colors.neutral[900];
    const BgColorHover = theme.colors.neutral[500];
</script>

<button
    class="svg-container"
    style="--bg: {BgColor}; --text-color: {fill} --bg-Hover: {fillHover}; --text-color-Hover: {BgColorHover}"
    onclick={bubble("click")}
>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        style="--fill: {fill}"
    >
        <path fill="none" d="M0 0h24v24H0z"></path>
        <path
            d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z"
        ></path>
    </svg>
</button>

<style scoped>
    svg {
        fill: var(--fill);
        height: 100%;
        padding: 10%;
        transition: ease-in-out 0.2s;
    }

    .svg-container {
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: var(--bg);
        height: 100%;
    }

    svg:hover {
        fill: var(--fill-hover);
    }
    .svg-container:hover {
        background-color: var(--bg-hover);
    }
</style>
