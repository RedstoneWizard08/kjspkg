<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { randFonts, randStretches, randStyles, randWeights, randVariants } from "$lib/font";
    import { onMount } from "svelte";
    import TablerIcon from "$components/icons/TablerIcon.svelte";
    import ProjectScroller from "$components/ui/ProjectScroller.svelte";

    const fontChangeDuration = 500;
    const addCharDelay = 150;

    let currentFonts = $state(randFonts(6));
    let currentStretches = $state(randStretches(6));
    let currentStyles = $state(randStyles(6));
    let currentWeights = $state(randWeights(6));
    let currentVariants = $state(randVariants(6));

    let tagline = $_("site.tagline");
    let adding = $state(true);
    let text = $state("");

    const canAddChar = () => text.length < tagline.length;

    const addChar = () => {
        const index = text.length;

        if (canAddChar()) {
            text += tagline[index];
        }
    };

    const scheduleAddChar = () => {
        addChar();

        if (canAddChar()) {
            setTimeout(scheduleAddChar, addCharDelay);
        } else {
            setTimeout(() => {
                adding = false;
            }, addCharDelay);
        }
    };

    const changeFont = () => {
        currentFonts = randFonts(6);
        currentStretches = randStretches(6);
        currentStyles = randStyles(6);
        currentWeights = randWeights(6);
        currentVariants = randVariants(6);

        setTimeout(changeFont, fontChangeDuration);
    };

    onMount(() => {
        // setTimeout(changeFont, fontChangeDuration); // Yeah ok no its not that cool
        scheduleAddChar();
    });

    locale.subscribe(() => {
        tagline = $_("site.tagline");
        text = "";
        scheduleAddChar();
    });
</script>

<svelte:head>
    <title>Home - KJSPKG</title>
</svelte:head>

<div
    class="m-0 flex w-full flex-col items-center justify-center bg-gradient-to-b from-primary-900 from-10% via-secondary-700 to-surface-900 to-90% p-10 py-40"
>
    <!-- When you can't decide what font to use: -->
    <h2 class="flex flex-row items-center justify-center text-4xl">
        {#each "KJSPKG" as letter}
            <!-- <span
                style:font-family={currentFonts[idx]}
                style:font-style={currentStyles[idx]}
                style:font-weight={currentWeights[idx]}
                style:font-variant={currentVariants[idx]}
                style:font-stretch={currentStretches[idx]}>{letter}</span
            > -->
            <span>{letter}</span>
        {/each}

        <span class="variant-filled-primary badge ml-4">{$_("site.beta")}</span>
    </h2>

    <span class="mt-2 animate-border-blink border-r-2 pr-1 text-xl font-bold">
        {text}
    </span>

    <div
        class="mt-16 flex flex-col items-center justify-center space-y-4 md:flex-row md:space-x-8 md:space-y-0"
    >
        <a href="/s" class="variant-filled-primary btn">
            <span><TablerIcon name="search" class="mr-2" /></span>
            <span>{$_("site.browse")}</span>
        </a>

        <a href="/install" class="variant-filled-secondary btn">
            <span><TablerIcon name="download" class="mr-2" /></span>
            <span>{$_("site.install")}</span>
        </a>
    </div>

    <ProjectScroller />
</div>
